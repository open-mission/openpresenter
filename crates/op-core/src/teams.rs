use serde::{Deserialize, Serialize};

/// Departamento da equipe de mídia da igreja (ex: Projeção, Fotografia, Stories, Transmissão, Áudio & Som).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamDepartment {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub color: String,
    pub icon: String,
    pub display_order: i32,
    pub created_at: String,
    pub updated_at: String,
}

/// DTO para criação e edição de departamento.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpsertTeamDepartment {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub display_order: Option<i32>,
}

/// Membro / voluntário da equipe de mídia.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamMember {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub avatar_url: Option<String>,
    pub color: String,
    #[serde(default)]
    pub notes: Option<String>,
    pub is_active: bool,
    #[serde(default)]
    pub department_ids: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// DTO para criação e edição de membro da equipe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpsertTeamMember {
    pub name: String,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub is_active: Option<bool>,
    #[serde(default)]
    pub department_ids: Vec<String>,
}

/// Item de atribuição de voluntário na escala de um culto.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceRosterItem {
    pub id: String,
    pub roster_id: String,
    pub department_id: String,
    pub member_id: String,
    #[serde(default = "default_role")]
    pub role: String,
    #[serde(default = "default_status")]
    pub status: String,
    #[serde(default)]
    pub role_notes: Option<String>,
    pub created_at: String,
}

fn default_role() -> String {
    "lead".to_string()
}

fn default_status() -> String {
    "pending".to_string()
}

/// DTO para adição ou alteração de item na escala.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpsertServiceRosterItem {
    pub department_id: String,
    pub member_id: String,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub role_notes: Option<String>,
}

/// Escala completa de um culto ou evento.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceRoster {
    pub id: String,
    pub title: String,
    pub service_date: String,
    #[serde(default)]
    pub service_time: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    pub is_active: bool,
    #[serde(default)]
    pub items: Vec<ServiceRosterItem>,
    pub created_at: String,
    pub updated_at: String,
}

/// Resumo de uma escala de culto para listagens rápidas.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceRosterSummary {
    pub id: String,
    pub title: String,
    pub service_date: String,
    #[serde(default)]
    pub service_time: Option<String>,
    pub is_active: bool,
    pub item_count: usize,
    pub updated_at: String,
}

/// DTO para criação ou atualização de escala de culto.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpsertServiceRoster {
    pub title: String,
    pub service_date: String,
    #[serde(default)]
    pub service_time: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub is_active: Option<bool>,
    #[serde(default)]
    pub items: Vec<UpsertServiceRosterItem>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_department_serialization() {
        let dept = TeamDepartment {
            id: "projecao".into(),
            name: "Projeção".into(),
            description: Some("Slides e letras".into()),
            color: "#3b82f6".into(),
            icon: "MonitorPlay".into(),
            display_order: 1,
            created_at: "2026-11-05T12:00:00Z".into(),
            updated_at: "2026-11-05T12:00:00Z".into(),
        };

        let json = serde_json::to_string(&dept).unwrap();
        let deserialized: TeamDepartment = serde_json::from_str(&json).unwrap();
        assert_eq!(dept, deserialized);
    }

    #[test]
    fn test_team_member_and_roster_serialization() {
        let member = TeamMember {
            id: "mem-1".into(),
            name: "João Silva".into(),
            phone: Some("11999998888".into()),
            email: None,
            avatar_url: None,
            color: "#10b981".into(),
            notes: Some("Disponível aos domingos".into()),
            is_active: true,
            department_ids: vec!["projecao".into(), "stories".into()],
            created_at: "2026-11-05T12:00:00Z".into(),
            updated_at: "2026-11-05T12:00:00Z".into(),
        };

        let json = serde_json::to_string(&member).unwrap();
        let deserialized: TeamMember = serde_json::from_str(&json).unwrap();
        assert_eq!(member, deserialized);

        let roster_item = ServiceRosterItem {
            id: "item-1".into(),
            roster_id: "roster-1".into(),
            department_id: "projecao".into(),
            member_id: "mem-1".into(),
            role: "lead".into(),
            status: "confirmed".into(),
            role_notes: Some("Titular".into()),
            created_at: "2026-11-05T12:00:00Z".into(),
        };

        let roster = ServiceRoster {
            id: "roster-1".into(),
            title: "Culto de Domingo - Noite".into(),
            service_date: "2026-11-08".into(),
            service_time: Some("19:00".into()),
            notes: None,
            is_active: true,
            items: vec![roster_item],
            created_at: "2026-11-05T12:00:00Z".into(),
            updated_at: "2026-11-05T12:00:00Z".into(),
        };

        let json_roster = serde_json::to_string(&roster).unwrap();
        let deserialized_roster: ServiceRoster = serde_json::from_str(&json_roster).unwrap();
        assert_eq!(roster, deserialized_roster);
    }
}
