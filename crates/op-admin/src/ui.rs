use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use op_core::OutputKind;
use slint::{CloseRequestResponse, ComponentHandle, ModelRc, TimerMode, VecModel};

use crate::commands::{CommandAction, CommandCategory, CommandRegistry};
use crate::outputs::OutputsController;
use crate::session::{LoadedSession, load_session};
use crate::{
    MainWindow, MediaLibraryItemData, PaletteItemData, ProgramItemData, SettingsWindow,
    SlidePreviewData, settings,
};

/// Monta o painel, abre as saídas configuradas e executa o loop de eventos.
pub fn run() -> anyhow::Result<()> {
    let LoadedSession { view, db } = load_session();
    let database_error = if view.db_ready {
        None
    } else {
        Some(view.db_error.clone())
    };
    let controller = Rc::new(RefCell::new(OutputsController::new(db, database_error)));
    let settings_slot = Rc::new(RefCell::new(None::<SettingsWindow>));
    let command_registry = Rc::new(CommandRegistry::new());

    let ui = MainWindow::new()?;

    // 1. Metadados e Estado da Aplicação
    ui.set_app_name(op_core::APP_NAME.into());
    ui.set_app_version(view.app_version.as_str().into());
    ui.set_db_ready(view.db_ready);
    ui.set_db_error(view.db_error.as_str().into());

    // 2. Estado Inicial da Sidebar (Biblioteca de Mídias)
    let library_items = initial_library_items();
    ui.set_media_library_items(ModelRc::new(VecModel::from(library_items)));

    // 3. Estado Inicial da Programação (Ordem do Culto)
    let program_items = Rc::new(RefCell::new(initial_program_items()));
    ui.set_program_items(ModelRc::new(VecModel::from(program_items.borrow().clone())));

    // 4. Estado Inicial do Preview dos Slides
    let active_program_id = "prog_2";
    ui.set_selected_program_item_id(active_program_id.into());
    ui.set_preview_header_title("Porque Ele Vive".into());
    ui.set_preview_header_subtitle("Harpa Cristã · 4 telas".into());

    let current_slides = Rc::new(RefCell::new(slides_for_program_item(active_program_id)));
    ui.set_preview_slides(ModelRc::new(VecModel::from(
        current_slides.borrow().clone(),
    )));
    ui.set_selected_slide_index(0);
    ui.set_live_slide_index(0);

    // 5. Estado Inicial da Command Palette
    let initial_palette_results = search_palette_results(&command_registry, "", 0);
    ui.set_palette_results(ModelRc::new(VecModel::from(initial_palette_results)));

    // 6. Configuração de Monitores e Posicionamento na Inicialização
    controller.borrow_mut().refresh_monitors();

    // Posiciona o painel administrativo no monitor principal
    if let Some(primary) = controller
        .borrow()
        .monitors()
        .iter()
        .find(|monitor| monitor.is_primary)
    {
        ui.window()
            .set_position(slint::PhysicalPosition::new(primary.x, primary.y));
    }

    // A janela principal abre maximizada cobrindo todo o espaço disponível
    ui.window().set_maximized(true);
    ui.show()?;

    // Inicia as saídas após exibir o painel administrativo, mantendo o isolamento
    controller.borrow_mut().start();

    // 7. Callbacks da Command Palette
    {
        let ui_handle = ui.as_weak();
        let registry = Rc::clone(&command_registry);
        ui.on_toggle_command_palette(move || {
            let Some(ui) = ui_handle.upgrade() else {
                return;
            };
            let is_open = ui.get_command_palette_open();
            if !is_open {
                // Ao abrir, atualiza os resultados conforme a query e categoria atuais
                let query = ui.get_palette_search_query();
                let category = ui.get_palette_selected_category();
                let results = search_palette_results(&registry, &query, category);
                ui.set_palette_results(ModelRc::new(VecModel::from(results)));
                ui.set_palette_selected_index(0);
            }
            ui.set_command_palette_open(!is_open);
        });
    }

    {
        let ui_handle = ui.as_weak();
        ui.on_close_command_palette(move || {
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_command_palette_open(false);
            }
        });
    }

    {
        let ui_handle = ui.as_weak();
        let registry = Rc::clone(&command_registry);
        ui.on_palette_query_changed(move |query| {
            let Some(ui) = ui_handle.upgrade() else {
                return;
            };
            let category = ui.get_palette_selected_category();
            let results = search_palette_results(&registry, &query, category);
            ui.set_palette_results(ModelRc::new(VecModel::from(results)));
            ui.set_palette_selected_index(0);
        });
    }

    {
        let ui_handle = ui.as_weak();
        let registry = Rc::clone(&command_registry);
        ui.on_palette_category_selected(move |category| {
            let Some(ui) = ui_handle.upgrade() else {
                return;
            };
            let query = ui.get_palette_search_query();
            let results = search_palette_results(&registry, &query, category);
            ui.set_palette_results(ModelRc::new(VecModel::from(results)));
            ui.set_palette_selected_index(0);
        });
    }

    {
        let ui_handle = ui.as_weak();
        let registry = Rc::clone(&command_registry);
        let controller = Rc::clone(&controller);
        let settings_slot = Rc::clone(&settings_slot);
        let slides_slot = Rc::clone(&current_slides);

        ui.on_palette_execute_command(move |cmd_id| {
            let Some(ui) = ui_handle.upgrade() else {
                return;
            };
            ui.set_command_palette_open(false);

            let Some(item) = registry.find_by_id(&cmd_id) else {
                return;
            };

            match &item.action {
                CommandAction::OpenSettings => {
                    if let Err(error) =
                        settings::open(Rc::clone(&controller), Rc::clone(&settings_slot))
                    {
                        eprintln!("Não foi possível abrir a janela de Configurações: {error}");
                    }
                }
                CommandAction::TogglePublic => {
                    let mut ctrl = controller.borrow_mut();
                    let enabled = !ctrl.config().public.enabled;
                    ctrl.set_enabled(OutputKind::Public, enabled);
                }
                CommandAction::ToggleStage => {
                    let mut ctrl = controller.borrow_mut();
                    let enabled = !ctrl.config().stage.enabled;
                    ctrl.set_enabled(OutputKind::Stage, enabled);
                }
                CommandAction::Blackout => {
                    let is_active = !ui.get_is_blackout();
                    ui.set_is_blackout(is_active);
                }
                CommandAction::ClearText => {
                    let is_active = !ui.get_is_clear_text();
                    ui.set_is_clear_text(is_active);
                }
                CommandAction::ShowLogo => {
                    let is_active = !ui.get_is_logo_screen();
                    ui.set_is_logo_screen(is_active);
                }
                CommandAction::SelectSong(title) => {
                    ui.set_preview_header_title(title.as_str().into());
                    ui.set_preview_header_subtitle("Música selecionada pela busca rápida".into());
                    let new_slides = slides_for_program_item(&cmd_id);
                    *slides_slot.borrow_mut() = new_slides.clone();
                    ui.set_preview_slides(ModelRc::new(VecModel::from(new_slides)));
                    ui.set_selected_slide_index(0);
                }
                CommandAction::SelectScripture(ref_text) => {
                    ui.set_preview_header_title(ref_text.as_str().into());
                    ui.set_preview_header_subtitle("Passagem bíblica da busca rápida".into());
                    let new_slides = slides_for_program_item(&cmd_id);
                    *slides_slot.borrow_mut() = new_slides.clone();
                    ui.set_preview_slides(ModelRc::new(VecModel::from(new_slides)));
                    ui.set_selected_slide_index(0);
                }
            }
        });
    }

    // 8. Callbacks de Seleção da Programação e Preview
    {
        let ui_handle = ui.as_weak();
        let program_items = Rc::clone(&program_items);
        let slides_slot = Rc::clone(&current_slides);

        ui.on_select_program_item(move |item_id| {
            let Some(ui) = ui_handle.upgrade() else {
                return;
            };

            // Atualiza o item ativo na lista de programação
            let mut list = program_items.borrow_mut();
            let mut found_title = String::new();
            let mut found_subtitle = String::new();

            for item in list.iter_mut() {
                item.is_active = item.id == item_id.as_str();
                if item.is_active {
                    found_title = item.title.to_string();
                    found_subtitle = format!("{} · {} telas", item.subtitle, item.slides_count);
                }
            }

            ui.set_selected_program_item_id(item_id.clone());
            ui.set_preview_header_title(found_title.as_str().into());
            ui.set_preview_header_subtitle(found_subtitle.as_str().into());
            ui.set_program_items(ModelRc::new(VecModel::from(list.clone())));

            // Carrega os slides correspondentes ao item selecionado
            let new_slides = slides_for_program_item(&item_id);
            *slides_slot.borrow_mut() = new_slides.clone();
            ui.set_preview_slides(ModelRc::new(VecModel::from(new_slides)));
            ui.set_selected_slide_index(0);
        });
    }

    {
        let ui_handle = ui.as_weak();
        ui.on_select_slide(move |slide_idx| {
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_selected_slide_index(slide_idx);
            }
        });
    }

    {
        let ui_handle = ui.as_weak();
        let slides_slot = Rc::clone(&current_slides);

        ui.on_action_go_live(move |slide_idx| {
            let Some(ui) = ui_handle.upgrade() else {
                return;
            };
            ui.set_live_slide_index(slide_idx);

            let mut slides = slides_slot.borrow_mut();
            for (idx, slide) in slides.iter_mut().enumerate() {
                slide.is_live = idx as i32 == slide_idx;
            }
            ui.set_preview_slides(ModelRc::new(VecModel::from(slides.clone())));
        });
    }

    {
        let ui_handle = ui.as_weak();
        ui.on_action_toggle_blackout(move || {
            if let Some(ui) = ui_handle.upgrade() {
                let current = ui.get_is_blackout();
                ui.set_is_blackout(!current);
            }
        });
    }

    {
        let ui_handle = ui.as_weak();
        ui.on_action_toggle_clear_text(move || {
            if let Some(ui) = ui_handle.upgrade() {
                let current = ui.get_is_clear_text();
                ui.set_is_clear_text(!current);
            }
        });
    }

    {
        let ui_handle = ui.as_weak();
        ui.on_action_toggle_logo(move || {
            if let Some(ui) = ui_handle.upgrade() {
                let current = ui.get_is_logo_screen();
                ui.set_is_logo_screen(!current);
            }
        });
    }

    {
        let ui_handle = ui.as_weak();
        let program_items = Rc::clone(&program_items);

        ui.on_add_media_to_program(move |media_id| {
            let Some(ui) = ui_handle.upgrade() else {
                return;
            };

            let mut list = program_items.borrow_mut();
            let next_order = (list.len() + 1) as i32;

            let (title, subtitle, kind_label, kind_icon, slides_count) = match media_id.as_str() {
                "song_1" => ("Porque Ele Vive", "Harpa Cristã", "Louvor", "🎵", 4),
                "song_2" => ("Bondade de Deus", "Isaías Saad", "Louvor", "🎵", 5),
                "song_3" => ("Grandioso És Tu", "Harpa Cristã", "Louvor", "🎵", 4),
                "bible_1" => ("Salmos 23:1-6", "O Senhor é meu pastor", "Bíblia", "📖", 6),
                "bible_2" => ("João 3:16", "Porque Deus amou o mundo", "Bíblia", "📖", 2),
                _ => ("Mídia Adicionada", "Recurso", "Mídia", "🖼️", 1),
            };

            list.push(ProgramItemData {
                id: format!("prog_custom_{next_order}").into(),
                order: next_order,
                title: title.into(),
                subtitle: subtitle.into(),
                kind_label: kind_label.into(),
                kind_icon: kind_icon.into(),
                slides_count,
                is_active: false,
            });

            ui.set_program_items(ModelRc::new(VecModel::from(list.clone())));
        });
    }

    // 9. Menu de Configurações e Encerramento
    {
        let controller = Rc::clone(&controller);
        let settings_slot = Rc::clone(&settings_slot);
        ui.on_open_settings(move || {
            if let Err(error) = settings::open(Rc::clone(&controller), Rc::clone(&settings_slot)) {
                eprintln!("Não foi possível abrir a janela de Configurações: {error}");
            }
        });
    }

    {
        let controller = Rc::clone(&controller);
        let settings_slot = Rc::clone(&settings_slot);
        ui.window().on_close_requested(move || {
            settings::hide(&settings_slot);
            controller.borrow_mut().close_all();
            if let Err(error) = slint::quit_event_loop() {
                eprintln!("Não foi possível encerrar o loop de eventos: {error}");
            }
            CloseRequestResponse::HideWindow
        });
    }

    // 10. Atualização Periódica de Status das Saídas
    let status_timer = slint::Timer::default();
    {
        let controller = Rc::clone(&controller);
        let settings_slot = Rc::clone(&settings_slot);
        let ui_handle = ui.as_weak();

        status_timer.start(TimerMode::Repeated, Duration::from_millis(500), move || {
            let Ok(controller) = controller.try_borrow() else {
                return;
            };
            settings::refresh_if_open(&settings_slot, &controller);

            if let Some(ui) = ui_handle.upgrade() {
                let public_status = controller.status(OutputKind::Public);
                let stage_status = controller.status(OutputKind::Stage);

                ui.set_public_status_label(
                    format!("Público: {}", public_status.text_pt_br()).into(),
                );
                ui.set_public_status_kind(public_status.kind());

                ui.set_stage_status_label(format!("Palco: {}", stage_status.text_pt_br()).into());
                ui.set_stage_status_kind(stage_status.kind());
            }
        });
    }

    slint::run_event_loop()?;
    drop(status_timer);
    Ok(())
}

fn initial_library_items() -> Vec<MediaLibraryItemData> {
    vec![
        MediaLibraryItemData {
            id: "song_1".into(),
            title: "Porque Ele Vive".into(),
            category: "Louvor".into(),
            icon: "🎵".into(),
            details: "Harpa Cristã · 4 estrofes".into(),
        },
        MediaLibraryItemData {
            id: "song_2".into(),
            title: "Bondade de Deus".into(),
            category: "Louvor".into(),
            icon: "🎵".into(),
            details: "Bethel Music / Isaías Saad · 5 estrofes".into(),
        },
        MediaLibraryItemData {
            id: "song_3".into(),
            title: "Grandioso És Tu".into(),
            category: "Louvor".into(),
            icon: "🎵".into(),
            details: "Harpa Cristã · 4 estrofes".into(),
        },
        MediaLibraryItemData {
            id: "bible_1".into(),
            title: "Salmos 23:1-6".into(),
            category: "Bíblia".into(),
            icon: "📖".into(),
            details: "O Senhor é o meu pastor · 6 versículos".into(),
        },
        MediaLibraryItemData {
            id: "bible_2".into(),
            title: "João 3:16-17".into(),
            category: "Bíblia".into(),
            icon: "📖".into(),
            details: "Porque Deus amou o mundo · 2 versículos".into(),
        },
        MediaLibraryItemData {
            id: "media_1".into(),
            title: "Fundo Abstrato Azul".into(),
            category: "Mídia".into(),
            icon: "🖼️".into(),
            details: "Imagem HD 1920×1080".into(),
        },
        MediaLibraryItemData {
            id: "announcement_1".into(),
            title: "Avisos da Semana".into(),
            category: "Avisos".into(),
            icon: "📢".into(),
            details: "Cultos e comunicados · 3 slides".into(),
        },
    ]
}

fn initial_program_items() -> Vec<ProgramItemData> {
    vec![
        ProgramItemData {
            id: "prog_1".into(),
            order: 1,
            title: "Avisos Iniciais".into(),
            subtitle: "Boas-vindas e comunicados".into(),
            kind_label: "Avisos".into(),
            kind_icon: "📢".into(),
            slides_count: 2,
            is_active: false,
        },
        ProgramItemData {
            id: "prog_2".into(),
            order: 2,
            title: "Porque Ele Vive".into(),
            subtitle: "Harpa Cristã".into(),
            kind_label: "Louvor".into(),
            kind_icon: "🎵".into(),
            slides_count: 4,
            is_active: true,
        },
        ProgramItemData {
            id: "prog_3".into(),
            order: 3,
            title: "Bondade de Deus".into(),
            subtitle: "Isaías Saad".into(),
            kind_label: "Louvor".into(),
            kind_icon: "🎵".into(),
            slides_count: 5,
            is_active: false,
        },
        ProgramItemData {
            id: "prog_4".into(),
            order: 4,
            title: "Leitura Bíblica: Salmos 23".into(),
            subtitle: "Versículos 1 a 6".into(),
            kind_label: "Bíblia".into(),
            kind_icon: "📖".into(),
            slides_count: 6,
            is_active: false,
        },
        ProgramItemData {
            id: "prog_5".into(),
            order: 5,
            title: "Mensagem da Palavra".into(),
            subtitle: "Ministração pastoral".into(),
            kind_label: "Palavra".into(),
            kind_icon: "📖".into(),
            slides_count: 1,
            is_active: false,
        },
    ]
}

fn slides_for_program_item(id: &str) -> Vec<SlidePreviewData> {
    match id {
        "prog_2" | "song_1" | "song_porque_ele_vive" => vec![
            SlidePreviewData {
                index: 0,
                label: "Estrofe 1".into(),
                text_content: "Deus enviou Seu Filho amado\nPara morrer em meu lugar\nNa cruz pagou por meus pecados\nMas o sepulcro vazio está\nPorque Ele vive!".into(),
                is_live: true,
            },
            SlidePreviewData {
                index: 1,
                label: "Refrão".into(),
                text_content: "Porque Ele vive, eu posso crer no amanhã\nPorque Ele vive, temor não há\nMas eu bem sei, eu sei que a minha vida\nEstá nas mãos de meu Jesus, que vivo está!".into(),
                is_live: false,
            },
            SlidePreviewData {
                index: 2,
                label: "Estrofe 2".into(),
                text_content: "E quando enfim chegar a hora\nEm que a morte enfrentarei\nSem medo então, terei vitória\nIrei à Glória do meu Jesus\nQue vivo está!".into(),
                is_live: false,
            },
            SlidePreviewData {
                index: 3,
                label: "Refrão Final".into(),
                text_content: "Porque Ele vive, eu posso crer no amanhã\nPorque Ele vive, temor não há\nMas eu bem sei, eu sei que a minha vida\nEstá nas mãos de meu Jesus, que vivo está!".into(),
                is_live: false,
            },
        ],
        "prog_3" | "song_2" | "song_bondade_de_deus" => vec![
            SlidePreviewData {
                index: 0,
                label: "Verso 1".into(),
                text_content: "Te amo, Deus\nTua graça nunca falha\nTodos os dias\nEu estou em Tuas mãos".into(),
                is_live: false,
            },
            SlidePreviewData {
                index: 1,
                label: "Refrão".into(),
                text_content: "És fiel em todo tempo\nEm todo tempo Tu és tão, tão bom\nCom tudo que sou, com tudo que tenho\nEu cantarei da bondade de Deus".into(),
                is_live: false,
            },
            SlidePreviewData {
                index: 2,
                label: "Ponte".into(),
                text_content: "Tua fidelidade vai me perseguir\nTua fidelidade vai me perseguir\nEu me rendo a Ti, Te dou o meu sim\nTua fidelidade vai me perseguir".into(),
                is_live: false,
            },
        ],
        "prog_4" | "bible_1" | "bible_salmos_23_1" => vec![
            SlidePreviewData {
                index: 0,
                label: "Salmos 23:1".into(),
                text_content: "O Senhor é o meu pastor;\nnada me faltará.".into(),
                is_live: false,
            },
            SlidePreviewData {
                index: 1,
                label: "Salmos 23:2".into(),
                text_content: "Deitar-me faz em verdes pastos,\nguia-me mansamente a águas tranquilas.".into(),
                is_live: false,
            },
            SlidePreviewData {
                index: 2,
                label: "Salmos 23:3".into(),
                text_content: "Refrigera a minha alma;\nguia-me pelas veredas da justiça,\npor amor do seu nome.".into(),
                is_live: false,
            },
            SlidePreviewData {
                index: 3,
                label: "Salmos 23:4".into(),
                text_content: "Ainda que eu andasse pelo vale da sombra da morte,\nnão temeria mal algum,\nporque tu estás comigo;\na tua vara e o teu cajado me consolam.".into(),
                is_live: false,
            },
        ],
        _ => vec![
            SlidePreviewData {
                index: 0,
                label: "Slide 1".into(),
                text_content: "Sejam bem-vindos à Casa do Senhor!\n\nCulto de Celebração e Adoração".into(),
                is_live: false,
            },
            SlidePreviewData {
                index: 1,
                label: "Slide 2".into(),
                text_content: "Avisos da semana:\n• Terça: Reunião de Oração às 19h30\n• Quinta: Estudo Bíblico às 20h00".into(),
                is_live: false,
            },
        ],
    }
}

fn search_palette_results(
    registry: &CommandRegistry,
    query: &str,
    category_index: i32,
) -> Vec<PaletteItemData> {
    let category = CommandCategory::from_index(category_index);
    let items = registry.search(query, category);
    items
        .into_iter()
        .map(|item| PaletteItemData {
            id: item.id.as_str().into(),
            title: item.title.as_str().into(),
            subtitle: item.subtitle.as_str().into(),
            category_label: item.category.label_pt_br().into(),
            category_icon: item.category.icon().into(),
            shortcut: item.shortcut.as_deref().unwrap_or("").into(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_data_models_are_populated() {
        let lib = initial_library_items();
        assert!(!lib.is_empty());
        assert!(lib.iter().any(|i| i.category == "Louvor"));
        assert!(lib.iter().any(|i| i.category == "Bíblia"));

        let prog = initial_program_items();
        assert!(!prog.is_empty());
        assert_eq!(prog[0].order, 1);
        assert!(prog.iter().any(|i| i.is_active));

        let slides = slides_for_program_item("prog_2");
        assert_eq!(slides.len(), 4);
        assert!(slides[0].is_live);
        assert!(!slides[1].is_live);
    }

    #[test]
    fn search_palette_results_converts_commands_to_slint_data() {
        let registry = CommandRegistry::new();
        let results = search_palette_results(&registry, "Porque Ele", 0);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Porque Ele Vive");
        assert_eq!(results[0].category_label, "Músicas");
        assert_eq!(results[0].category_icon, "🎵");

        let cmd_results = search_palette_results(&registry, "Configurações", 3);
        assert_eq!(cmd_results.len(), 1);
        assert_eq!(cmd_results[0].title, "Abrir Configurações");
        assert_eq!(cmd_results[0].category_label, "Comandos");
    }
}
