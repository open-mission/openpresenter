use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};

use op_core::{
    DisplayMode, MediaItem, MediaKind, OutputsConfig, PresentationState, Setlist, Slide, WsMessage,
};

use crate::session::SessionView;

/// Capacidade do canal de broadcast de mensagens em tempo real.
const BROADCAST_CAPACITY: usize = 128;

/// Estado global compartilhado do aplicativo em memória.
#[derive(Clone)]
pub struct AppState {
    pub session: SessionView,
    pub presentation: Arc<RwLock<PresentationState>>,
    pub media_items: Arc<RwLock<Vec<MediaItem>>>,
    pub active_setlist: Arc<RwLock<Option<Setlist>>>,
    pub outputs: Arc<RwLock<OutputsConfig>>,
    pub broadcast_tx: broadcast::Sender<WsMessage>,
    pub db: Option<Arc<tokio::sync::Mutex<op_db::Database>>>,
    pub http_client: reqwest::Client,
}

impl AppState {
    /// Inicializa o estado com itens de demonstração, canal de transmissão e conexão com banco.
    pub fn new(
        session: SessionView,
        outputs: OutputsConfig,
        mut db: Option<op_db::Database>,
    ) -> Self {
        let (broadcast_tx, _) = broadcast::channel(BROADCAST_CAPACITY);
        let sample_media = create_sample_media();

        let mut initial_setlist = Setlist {
            id: "setlist-default".into(),
            title: "Culto Principal".into(),
            description: Some("Repertório e programação principal do culto".into()),
            wallpaper_path: Some("/api/media/files/wallpapers/dark-ambient.svg".into()),
            wallpaper_mode: Some("cover".into()),
            items: sample_media.clone(),
            is_active: true,
            created_at: "".into(),
            updated_at: "".into(),
        };

        let mut initial_presentation = PresentationState {
            active_setlist_id: Some(initial_setlist.id.clone()),
            wallpaper_path: initial_setlist.wallpaper_path.clone(),
            wallpaper_mode: initial_setlist.wallpaper_mode.clone(),
            ..Default::default()
        };

        let mut initial_items = sample_media;

        // Se houver banco de dados, recupera a setlist ativa ou persiste a inicial
        if let Some(ref mut db_conn) = db {
            if let Ok(Some(active)) = db_conn.get_active_setlist() {
                initial_presentation.active_setlist_id = Some(active.id.clone());
                initial_presentation.wallpaper_path = active.wallpaper_path.clone();
                initial_presentation.wallpaper_mode = active.wallpaper_mode.clone();
                initial_items = active.items.clone();
                initial_setlist = active;
            } else {
                let _ = db_conn.save_setlist(&initial_setlist);
            }
        }

        Self {
            session,
            presentation: Arc::new(RwLock::new(initial_presentation)),
            media_items: Arc::new(RwLock::new(initial_items)),
            active_setlist: Arc::new(RwLock::new(Some(initial_setlist))),
            outputs: Arc::new(RwLock::new(outputs)),
            broadcast_tx,
            db: db.map(|d| Arc::new(tokio::sync::Mutex::new(d))),
            http_client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
        }
    }

    /// Adiciona ou atualiza um item na lista de mídias (Setlist) e transmite o novo estado via WebSocket.
    pub async fn add_media_item(&self, item: MediaItem) {
        let mut items = self.media_items.write().await;
        if let Some(existing) = items.iter_mut().find(|m| m.id == item.id) {
            *existing = item;
        } else {
            items.push(item);
        }
        let all_items = items.clone();
        drop(items);

        // Atualiza a setlist em memória e banco
        self.sync_active_setlist_items(&all_items).await;

        let pres = self.presentation.read().await.clone();
        let _ = self.broadcast_tx.send(WsMessage::SyncState {
            presentation: pres,
            media_items: all_items,
        });
    }

    /// Remove um item da setlist ativa.
    pub async fn remove_media_item(&self, item_id: &str) -> bool {
        let mut items = self.media_items.write().await;
        let original_len = items.len();
        items.retain(|m| m.id != item_id);
        let removed = items.len() < original_len;
        let all_items = items.clone();
        drop(items);

        if removed {
            // Se o item removido estava sendo apresentado, encerra a apresentação
            let mut pres = self.presentation.write().await;
            if pres.active_media_id.as_deref() == Some(item_id) {
                pres.active_media_id = None;
                pres.active_slide_index = None;
                pres.display_mode = DisplayMode::ClearText;
            }
            let updated_pres = pres.clone();
            drop(pres);

            self.sync_active_setlist_items(&all_items).await;

            let _ = self.broadcast_tx.send(WsMessage::SyncState {
                presentation: updated_pres,
                media_items: all_items,
            });
        }

        removed
    }

    /// Reordena os itens da setlist ativa (ou move um item para o topo).
    pub async fn reorder_media_items(&self, items: Vec<MediaItem>) {
        let mut current_items = self.media_items.write().await;
        *current_items = items.clone();
        drop(current_items);

        self.sync_active_setlist_items(&items).await;

        let pres = self.presentation.read().await.clone();
        let _ = self.broadcast_tx.send(WsMessage::SyncState {
            presentation: pres,
            media_items: items,
        });
    }

    /// Renomeia um item da setlist ativa.
    pub async fn rename_media_item(&self, item_id: &str, new_title: &str) -> bool {
        let mut items = self.media_items.write().await;
        let mut found = false;
        for item in items.iter_mut() {
            if item.id == item_id {
                item.title = new_title.to_string();
                found = true;
                break;
            }
        }
        let all_items = items.clone();
        drop(items);

        if found {
            self.sync_active_setlist_items(&all_items).await;

            let pres = self.presentation.read().await.clone();
            let _ = self.broadcast_tx.send(WsMessage::SyncState {
                presentation: pres,
                media_items: all_items,
            });
        }

        found
    }

    /// Troca a Setlist ativa.
    pub async fn switch_active_setlist(&self, setlist_id: &str) -> Option<Setlist> {
        let db_opt = self.db.as_ref()?;
        let mut db = db_opt.lock().await;

        if db.set_active_setlist(setlist_id).is_err() {
            return None;
        }

        let setlist = db.get_active_setlist().ok()??;
        drop(db);

        // Atualiza memória
        let mut active_ref = self.active_setlist.write().await;
        *active_ref = Some(setlist.clone());
        drop(active_ref);

        let mut items_ref = self.media_items.write().await;
        *items_ref = setlist.items.clone();
        drop(items_ref);

        let mut pres = self.presentation.write().await;
        pres.active_setlist_id = Some(setlist.id.clone());
        pres.wallpaper_path = setlist.wallpaper_path.clone();
        pres.wallpaper_mode = setlist.wallpaper_mode.clone();
        let updated_pres = pres.clone();
        drop(pres);

        let _ = self.broadcast_tx.send(WsMessage::SyncState {
            presentation: updated_pres,
            media_items: setlist.items.clone(),
        });
        let _ = self.broadcast_tx.send(WsMessage::SetlistUpdated {
            setlist: setlist.clone(),
        });

        Some(setlist)
    }

    /// Sincroniza a lista de itens com a setlist ativa no banco.
    async fn sync_active_setlist_items(&self, items: &[MediaItem]) {
        let mut active = self.active_setlist.write().await;
        if let Some(ref mut s) = *active {
            s.items = items.to_vec();
            if let Some(ref db_opt) = self.db {
                let mut db = db_opt.lock().await;
                let _ = db.save_setlist(s);
            }
        }
    }

    /// Dispara um slide específico de uma mídia para o modo ao vivo.
    pub async fn select_slide(&self, media_id: String, slide_index: usize) -> PresentationState {
        let mut state = self.presentation.write().await;
        state.active_media_id = Some(media_id);
        state.active_slide_index = Some(slide_index);
        state.display_mode = DisplayMode::Live;
        let updated = state.clone();
        drop(state);

        let _ = self.broadcast_tx.send(WsMessage::StateUpdated {
            presentation: updated.clone(),
        });

        updated
    }

    /// Altera o modo de exibição (Live, Blackout, ClearText).
    pub async fn set_display_mode(&self, mode: DisplayMode) -> PresentationState {
        let mut state = self.presentation.write().await;
        state.display_mode = mode;
        let updated = state.clone();
        drop(state);

        let _ = self.broadcast_tx.send(WsMessage::StateUpdated {
            presentation: updated.clone(),
        });

        updated
    }

    /// Alterna o modo Blackout (tela preta total).
    pub async fn toggle_blackout(&self) -> PresentationState {
        let mut state = self.presentation.write().await;
        if state.display_mode == DisplayMode::Blackout {
            state.display_mode = DisplayMode::Live;
        } else {
            state.display_mode = DisplayMode::Blackout;
        }
        let updated = state.clone();
        drop(state);

        let _ = self.broadcast_tx.send(WsMessage::StateUpdated {
            presentation: updated.clone(),
        });

        updated
    }

    /// Alterna o modo Limpar Texto (fundo mantido, sem texto).
    pub async fn toggle_clear_text(&self) -> PresentationState {
        let mut state = self.presentation.write().await;
        if state.display_mode == DisplayMode::ClearText {
            state.display_mode = DisplayMode::Live;
        } else {
            state.display_mode = DisplayMode::ClearText;
        }
        let updated = state.clone();
        drop(state);

        let _ = self.broadcast_tx.send(WsMessage::StateUpdated {
            presentation: updated.clone(),
        });

        updated
    }

    /// Atualiza status de conexão de uma tela.
    pub async fn set_connection(&self, client_type: &str, connected: bool) {
        let mut state = self.presentation.write().await;
        match client_type {
            "public" => state.public_connected = connected,
            "stage" => state.stage_connected = connected,
            _ => return,
        }
        let updated = state.clone();
        drop(state);

        let _ = self.broadcast_tx.send(WsMessage::StateUpdated {
            presentation: updated,
        });
    }
}

/// Cria o acervo de demonstração inicial com louvores, bíblia e avisos (estilo Holyrics e ProPresenter).
fn create_sample_media() -> Vec<MediaItem> {
    vec![
        MediaItem {
            id: "song-1".into(),
            title: "Porque Ele Vive".into(),
            artist_or_ref: Some("Harpa Cristã / Tradicional".into()),
            kind: MediaKind::Song,
            slides: vec![
                Slide::new(
                    "s1-1",
                    "Verso 1",
                    "Deus enviou Seu Filho amado\nPara morrer em meu lugar\nNa cruz pagou por meus pecados\nMas o sepulcro vazio está",
                ),
                Slide::new(
                    "s1-2",
                    "Refrão",
                    "Porque Ele vive, posso crer no amanhã\nPorque Ele vive, temor não há\nMas eu bem sei, eu sei, que a minha vida\nEstá nas mãos de meu Jesus, que vivo está",
                ),
                Slide::new(
                    "s1-3",
                    "Verso 2",
                    "E quando, enfim, chegar a hora\nEm que a morte enfrentarei\nSem medo, então, terei vitória\nIrei à glória, ao meu Jesus que vivo está",
                ),
                Slide::new(
                    "s1-4",
                    "Refrão Final",
                    "Porque Ele vive, posso crer no amanhã\nPorque Ele vive, temor não há\nMas eu bem sei, eu sei, que a minha vida\nEstá nas mãos de meu Jesus, que vivo está",
                ),
            ],
            align_h: None,
            align_v: None,
        },
        MediaItem {
            id: "song-2".into(),
            title: "Graça Maravilhosa".into(),
            artist_or_ref: Some("John Newton / Harpa".into()),
            kind: MediaKind::Song,
            slides: vec![
                Slide::new(
                    "s2-1",
                    "Verso 1",
                    "Graça maravilhosa, tão doce o som\nQue salvou um pecador como eu\nEstava cego, agora eu vejo\nPerdido estava, mas me achou",
                ),
                Slide::new(
                    "s2-2",
                    "Refrão",
                    "Minhas cadeias foram quebradas\nLivre estou, livre estou\nE como um rio Sua graça flui\nAmor sem fim, graça sem par",
                ),
                Slide::new(
                    "s2-3",
                    "Verso 2",
                    "O Senhor prometeu o bem para mim\nSua palavra garante a minha esperança\nEle será meu escudo e porção\nEnquanto a vida durar",
                ),
            ],
            align_h: None,
            align_v: None,
        },
        MediaItem {
            id: "bible-1".into(),
            title: "Salmos 23:1-4".into(),
            artist_or_ref: Some("Antigo Testamento".into()),
            kind: MediaKind::Scripture,
            slides: vec![
                Slide::new(
                    "b1-1",
                    "Salmos 23:1",
                    "O Senhor é o meu pastor;\nnada me faltará.",
                ),
                Slide::new(
                    "b1-2",
                    "Salmos 23:2",
                    "Deitar-me faz em verdes pastos,\nguia-me mansamente a águas tranquilas.",
                ),
                Slide::new(
                    "b1-3",
                    "Salmos 23:3",
                    "Refrigera a minha alma;\nguia-me pelas veredas da justiça,\npor amor do seu nome.",
                ),
                Slide::new(
                    "b1-4",
                    "Salmos 23:4",
                    "Ainda que eu andasse pelo vale da sombra da morte,\nnão temeria mal algum, porque tu estás comigo;\na tua vara e o teu cajado me consolam.",
                ),
            ],
            align_h: None,
            align_v: None,
        },
        MediaItem {
            id: "notice-1".into(),
            title: "Boas-vindas e Comunicados".into(),
            artist_or_ref: Some("Avisos da Semana".into()),
            kind: MediaKind::Announcement,
            slides: vec![
                Slide::new(
                    "n1-1",
                    "Boas-vindas",
                    "Sejam bem-vindos à Casa do Senhor!\nUma noite de louvor, comunhão e palavra.",
                ),
                Slide::new(
                    "n1-2",
                    "Culto de Oração",
                    "Quarta-feira às 19h30:\nReunião de Oração e Estudo Bíblico.\nParticipe conosco!",
                ),
                Slide::new(
                    "n1-3",
                    "Escola Bíblica",
                    "Domingo às 09h00:\nEscola Bíblica Dominical para todas as idades.\nTraga sua família!",
                ),
            ],
            align_h: None,
            align_v: None,
        },
    ]
}
