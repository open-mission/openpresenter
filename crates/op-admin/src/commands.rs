//! Registro, busca e execução da Command Palette (⌘K / Ctrl+K).

/// Categoria de comando ou conteúdo pesquisável na Command Palette.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandCategory {
    All,
    Songs,
    Bible,
    Commands,
}

impl CommandCategory {
    #[cfg(test)]
    pub const ALL: [Self; 4] = [Self::All, Self::Songs, Self::Bible, Self::Commands];

    /// Rótulo em português do Brasil para a interface.
    pub const fn label_pt_br(&self) -> &'static str {
        match self {
            Self::All => "Todos",
            Self::Songs => "Músicas",
            Self::Bible => "Bíblia",
            Self::Commands => "Comandos",
        }
    }

    /// Ícone / símbolo representativo da categoria.
    pub const fn icon(&self) -> &'static str {
        match self {
            Self::All => "✦",
            Self::Songs => "🎵",
            Self::Bible => "📖",
            Self::Commands => "⚙️",
        }
    }

    /// Converte um índice numérico de aba (0=Todos, 1=Músicas, 2=Bíblia, 3=Comandos) na categoria.
    pub const fn from_index(index: i32) -> Self {
        match index {
            1 => Self::Songs,
            2 => Self::Bible,
            3 => Self::Commands,
            _ => Self::All,
        }
    }

    /// Converte a categoria para o índice numérico da aba na interface.
    #[cfg(test)]
    pub const fn to_index(self) -> i32 {
        match self {
            Self::All => 0,
            Self::Songs => 1,
            Self::Bible => 2,
            Self::Commands => 3,
        }
    }
}

/// Ação disparada ao ativar um comando na paleta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandAction {
    OpenSettings,
    TogglePublic,
    ToggleStage,
    Blackout,
    ClearText,
    ShowLogo,
    SelectSong(String),
    SelectScripture(String),
}

/// Um item indexado na Command Palette.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandItem {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub category: CommandCategory,
    pub shortcut: Option<String>,
    pub action: CommandAction,
}

/// Registro central de comandos e buscas do aplicativo.
pub struct CommandRegistry {
    items: Vec<CommandItem>,
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandRegistry {
    /// Cria o registro carregando os comandos padrão do sistema e amostras de mídia.
    pub fn new() -> Self {
        let items = vec![
            // 1. Comandos do Sistema
            CommandItem {
                id: "settings".into(),
                title: "Abrir Configurações".into(),
                subtitle: "Gerenciar saídas, monitores e preferências".into(),
                category: CommandCategory::Commands,
                shortcut: Some("⌘,".into()),
                action: CommandAction::OpenSettings,
            },
            CommandItem {
                id: "toggle_public".into(),
                title: "Alternar Saída Pública".into(),
                subtitle: "Ligar ou desligar a projeção principal".into(),
                category: CommandCategory::Commands,
                shortcut: Some("F1".into()),
                action: CommandAction::TogglePublic,
            },
            CommandItem {
                id: "toggle_stage".into(),
                title: "Alternar Retorno de Palco".into(),
                subtitle: "Ligar ou desligar o monitor do palco (Stage)".into(),
                category: CommandCategory::Commands,
                shortcut: Some("F2".into()),
                action: CommandAction::ToggleStage,
            },
            CommandItem {
                id: "blackout".into(),
                title: "Limpar Tela (Blackout)".into(),
                subtitle: "Escurecer a saída pública imediatamente".into(),
                category: CommandCategory::Commands,
                shortcut: Some("F5".into()),
                action: CommandAction::Blackout,
            },
            CommandItem {
                id: "clear_text".into(),
                title: "Ocultar Texto".into(),
                subtitle: "Ocultar o texto mantendo o fundo na saída pública".into(),
                category: CommandCategory::Commands,
                shortcut: Some("F6".into()),
                action: CommandAction::ClearText,
            },
            CommandItem {
                id: "show_logo".into(),
                title: "Exibir Logotipo".into(),
                subtitle: "Projetar a marca OpenPresenter na saída pública".into(),
                category: CommandCategory::Commands,
                shortcut: Some("F7".into()),
                action: CommandAction::ShowLogo,
            },
            // 2. Acervo de Músicas (Amostras preparadas para a Command Palette)
            CommandItem {
                id: "song_porque_ele_vive".into(),
                title: "Porque Ele Vive".into(),
                subtitle: "Harpa Cristã · Deus enviou Seu Filho amado para morrer...".into(),
                category: CommandCategory::Songs,
                shortcut: None,
                action: CommandAction::SelectSong("Porque Ele Vive".into()),
            },
            CommandItem {
                id: "song_bondade_de_deus".into(),
                title: "Bondade de Deus".into(),
                subtitle: "Bethel Music / Isaías Saad · Te amo, Deus, Tua graça nunca falha..."
                    .into(),
                category: CommandCategory::Songs,
                shortcut: None,
                action: CommandAction::SelectSong("Bondade de Deus".into()),
            },
            CommandItem {
                id: "song_grandioso_es_tu".into(),
                title: "Grandioso És Tu".into(),
                subtitle: "Harpa Cristã · Senhor meu Deus, quando eu maravilhado...".into(),
                category: CommandCategory::Songs,
                shortcut: None,
                action: CommandAction::SelectSong("Grandioso És Tu".into()),
            },
            CommandItem {
                id: "song_vitorioso_es".into(),
                title: "Vitorioso És".into(),
                subtitle: "Gabriel Guedes · Quem foi que prometeu? Quem nunca falhou?...".into(),
                category: CommandCategory::Songs,
                shortcut: None,
                action: CommandAction::SelectSong("Vitorioso És".into()),
            },
            CommandItem {
                id: "song_em_fervente_oracao".into(),
                title: "Em Fervente Oração".into(),
                subtitle: "Harpa Cristã · Em fervente oração, vem o teu coração...".into(),
                category: CommandCategory::Songs,
                shortcut: None,
                action: CommandAction::SelectSong("Em Fervente Oração".into()),
            },
            // 3. Referências Bíblicas (Amostras preparadas para a Command Palette)
            CommandItem {
                id: "bible_joao_3_16".into(),
                title: "João 3:16".into(),
                subtitle:
                    "Porque Deus amou o mundo de tal maneira que deu o seu Filho unigênito..."
                        .into(),
                category: CommandCategory::Bible,
                shortcut: None,
                action: CommandAction::SelectScripture("João 3:16".into()),
            },
            CommandItem {
                id: "bible_salmos_23_1".into(),
                title: "Salmos 23:1".into(),
                subtitle: "O Senhor é o meu pastor; nada me faltará.".into(),
                category: CommandCategory::Bible,
                shortcut: None,
                action: CommandAction::SelectScripture("Salmos 23:1".into()),
            },
            CommandItem {
                id: "bible_filipenses_4_13".into(),
                title: "Filipenses 4:13".into(),
                subtitle: "Tudo posso naquele que me fortalece.".into(),
                category: CommandCategory::Bible,
                shortcut: None,
                action: CommandAction::SelectScripture("Filipenses 4:13".into()),
            },
            CommandItem {
                id: "bible_genesis_1_1".into(),
                title: "Gênesis 1:1".into(),
                subtitle: "No princípio criou Deus os céus e a terra.".into(),
                category: CommandCategory::Bible,
                shortcut: None,
                action: CommandAction::SelectScripture("Gênesis 1:1".into()),
            },
        ];

        Self { items }
    }

    /// Busca comandos filtrando por texto e categoria.
    pub fn search(&self, query: &str, category: CommandCategory) -> Vec<&CommandItem> {
        let query_clean = query.trim().to_lowercase();
        self.items
            .iter()
            .filter(|item| {
                if category != CommandCategory::All && item.category != category {
                    return false;
                }
                if query_clean.is_empty() {
                    return true;
                }
                item.title.to_lowercase().contains(&query_clean)
                    || item.subtitle.to_lowercase().contains(&query_clean)
            })
            .collect()
    }

    /// Localiza um comando pelo seu identificador único.
    pub fn find_by_id(&self, id: &str) -> Option<&CommandItem> {
        self.items.iter().find(|item| item.id == id)
    }

    /// Indica se o registro está vazio.
    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_registry_has_commands_songs_and_bible() {
        let registry = CommandRegistry::new();
        assert!(!registry.is_empty());

        let songs = registry.search("", CommandCategory::Songs);
        assert!(!songs.is_empty());
        assert!(songs.iter().all(|s| s.category == CommandCategory::Songs));

        let bible = registry.search("", CommandCategory::Bible);
        assert!(!bible.is_empty());
        assert!(bible.iter().all(|b| b.category == CommandCategory::Bible));

        let commands = registry.search("", CommandCategory::Commands);
        assert!(!commands.is_empty());
        assert!(
            commands
                .iter()
                .all(|c| c.category == CommandCategory::Commands)
        );
    }

    #[test]
    fn search_filters_by_title_and_subtitle() {
        let registry = CommandRegistry::new();

        // Busca por título
        let results = registry.search("Porque Ele Vive", CommandCategory::All);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Porque Ele Vive");

        // Busca por letra / subtítulo
        let results = registry.search("maravilhado", CommandCategory::All);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Grandioso És Tu");

        // Busca por referência bíblica
        let results = registry.search("23:1", CommandCategory::All);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Salmos 23:1");

        // Busca com categoria restritiva
        let results = registry.search("Configurações", CommandCategory::Songs);
        assert!(results.is_empty());

        let results = registry.search("Configurações", CommandCategory::Commands);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn category_indexing_round_trips() {
        for cat in CommandCategory::ALL {
            let idx = cat.to_index();
            assert_eq!(CommandCategory::from_index(idx), cat);
        }
        assert_eq!(CommandCategory::from_index(99), CommandCategory::All);
    }

    #[test]
    fn find_by_id_returns_exact_item() {
        let registry = CommandRegistry::new();
        let item = registry.find_by_id("settings").unwrap();
        assert_eq!(item.action, CommandAction::OpenSettings);
        assert_eq!(item.category, CommandCategory::Commands);
    }
}
