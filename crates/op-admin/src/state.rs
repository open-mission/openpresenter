use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};

use op_core::{
    DisplayMode, MediaItem, MediaKind, OutputsConfig, PresentationState, Slide, WsMessage,
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
    pub outputs: Arc<RwLock<OutputsConfig>>,
    pub broadcast_tx: broadcast::Sender<WsMessage>,
    pub db: Option<Arc<tokio::sync::Mutex<op_db::Database>>>,
    pub http_client: reqwest::Client,
}

impl AppState {
    /// Inicializa o estado com itens de demonstração, canal de transmissão e conexão com banco.
    pub fn new(session: SessionView, outputs: OutputsConfig, db: Option<op_db::Database>) -> Self {
        let (broadcast_tx, _) = broadcast::channel(BROADCAST_CAPACITY);
        let sample_media = create_sample_media();

        Self {
            session,
            presentation: Arc::new(RwLock::new(PresentationState::default())),
            media_items: Arc::new(RwLock::new(sample_media)),
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
        let pres = self.presentation.read().await.clone();
        drop(items);

        let _ = self.broadcast_tx.send(WsMessage::SyncState {
            presentation: pres,
            media_items: all_items,
        });
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
                Slide {
                    id: "s1-1".into(),
                    label: "Verso 1".into(),
                    content: "Deus enviou Seu Filho amado\nPara morrer em meu lugar\nNa cruz pagou por meus pecados\nMas o sepulcro vazio está".into(),
                },
                Slide {
                    id: "s1-2".into(),
                    label: "Refrão".into(),
                    content: "Porque Ele vive, posso crer no amanhã\nPorque Ele vive, temor não há\nMas eu bem sei, eu sei, que a minha vida\nEstá nas mãos de meu Jesus, que vivo está".into(),
                },
                Slide {
                    id: "s1-3".into(),
                    label: "Verso 2".into(),
                    content: "E quando, enfim, chegar a hora\nEm que a morte enfrentarei\nSem medo, então, terei vitória\nIrei à glória, ao meu Jesus que vivo está".into(),
                },
                Slide {
                    id: "s1-4".into(),
                    label: "Refrão Final".into(),
                    content: "Porque Ele vive, posso crer no amanhã\nPorque Ele vive, temor não há\nMas eu bem sei, eu sei, que a minha vida\nEstá nas mãos de meu Jesus, que vivo está".into(),
                },
            ],
        },
        MediaItem {
            id: "song-2".into(),
            title: "Graça Maravilhosa".into(),
            artist_or_ref: Some("John Newton / Harpa".into()),
            kind: MediaKind::Song,
            slides: vec![
                Slide {
                    id: "s2-1".into(),
                    label: "Verso 1".into(),
                    content: "Graça maravilhosa, tão doce o som\nQue salvou um pecador como eu\nEstava cego, agora eu vejo\nPerdido estava, mas me achou".into(),
                },
                Slide {
                    id: "s2-2".into(),
                    label: "Refrão".into(),
                    content: "Minhas cadeias foram quebradas\nLivre estou, livre estou\nE como um rio Sua graça flui\nAmor sem fim, graça sem par".into(),
                },
                Slide {
                    id: "s2-3".into(),
                    label: "Verso 2".into(),
                    content: "O Senhor prometeu o bem para mim\nSua palavra garante a minha esperança\nEle será meu escudo e porção\nEnquanto a vida durar".into(),
                },
            ],
        },
        MediaItem {
            id: "bible-1".into(),
            title: "Salmos 23:1-4".into(),
            artist_or_ref: Some("Antigo Testamento".into()),
            kind: MediaKind::Scripture,
            slides: vec![
                Slide {
                    id: "b1-1".into(),
                    label: "Salmos 23:1".into(),
                    content: "O Senhor é o meu pastor;\nnada me faltará.".into(),
                },
                Slide {
                    id: "b1-2".into(),
                    label: "Salmos 23:2".into(),
                    content: "Deitar-me faz em verdes pastos,\nguia-me mansamente a águas tranquilas.".into(),
                },
                Slide {
                    id: "b1-3".into(),
                    label: "Salmos 23:3".into(),
                    content: "Refrigera a minha alma;\nguia-me pelas veredas da justiça,\npor amor do seu nome.".into(),
                },
                Slide {
                    id: "b1-4".into(),
                    label: "Salmos 23:4".into(),
                    content: "Ainda que eu andasse pelo vale da sombra da morte,\nnão temeria mal algum, porque tu estás comigo;\na tua vara e o teu cajado me consolam.".into(),
                },
            ],
        },
        MediaItem {
            id: "notice-1".into(),
            title: "Boas-vindas e Comunicados".into(),
            artist_or_ref: Some("Avisos da Semana".into()),
            kind: MediaKind::Announcement,
            slides: vec![
                Slide {
                    id: "n1-1".into(),
                    label: "Boas-vindas".into(),
                    content: "Sejam bem-vindos à Casa do Senhor!\nUma noite de louvor, comunhão e palavra.".into(),
                },
                Slide {
                    id: "n1-2".into(),
                    label: "Culto de Oração".into(),
                    content: "Quarta-feira às 19h30:\nReunião de Oração e Estudo Bíblico.\nParticipe conosco!".into(),
                },
                Slide {
                    id: "n1-3".into(),
                    label: "Escola Bíblica".into(),
                    content: "Domingo às 09h00:\nEscola Bíblica Dominical para todas as idades.\nTraga sua família!".into(),
                },
            ],
        },
    ]
}
