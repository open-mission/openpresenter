use axum::{
    body::Body,
    extract::Path,
    http::{Response, StatusCode, header},
    response::{Html, IntoResponse, Json},
};
use rust_embed::RustEmbed;
use serde_json::{Value, json};

/// Assets offline para a documentação interativa (Scalar e Swagger UI) embutidos no binário.
#[derive(RustEmbed)]
#[folder = "assets/api-docs"]
pub struct EmbeddedApiDocs;

/// Manipulador para servir assets estáticos da documentação (JS e CSS) 100% offline.
pub async fn serve_api_docs_asset(Path(path): Path<String>) -> Response<Body> {
    let filename = path.trim_start_matches('/');

    if let Some(file) = EmbeddedApiDocs::get(filename) {
        let mime = mime_guess::from_path(filename).first_or_octet_stream();
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, mime.as_ref())
            .header(header::CACHE_CONTROL, "public, max-age=86400")
            .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .body(Body::from(file.data))
            .unwrap_or_else(|_| {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::empty())
                    .unwrap()
            })
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Asset não encontrado"))
            .unwrap()
    }
}

/// Serve a interface moderna do Scalar configurada para modo 100% offline.
pub async fn scalar_handler() -> Html<String> {
    Html(r#"<!doctype html>
<html lang="pt-BR">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>OpenPresenter — Documentação da API (Scalar)</title>
    <link rel="icon" type="image/svg+xml" href="/branding/open-presenter-app-icon.svg" />
    <style>
      body {
        margin: 0;
        padding: 0;
        background-color: #0f172a;
        color: #f8fafc;
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
      }
    </style>
  </head>
  <body>
    <script
      id="api-reference"
      data-url="/api/openapi.json"
      data-configuration='{"theme":"saturn","darkMode":true,"showSidebar":true}'></script>
    <script src="/docs/assets/scalar.standalone.js"></script>
  </body>
</html>"#
    .to_string())
}

/// Serve a interface clássica do Swagger UI configurada para modo 100% offline.
pub async fn swagger_handler() -> Html<String> {
    Html(r#"<!DOCTYPE html>
<html lang="pt-BR">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>OpenPresenter — Documentação da API (Swagger UI)</title>
    <link rel="icon" type="image/svg+xml" href="/branding/open-presenter-app-icon.svg" />
    <link rel="stylesheet" href="/docs/assets/swagger-ui.css" />
    <style>
      html {
        box-sizing: border-box;
        overflow: -moz-scrollbars-vertical;
        overflow-y: scroll;
      }
      *, *:before, *:after {
        box-sizing: inherit;
      }
      body {
        margin: 0;
        background: #1e293b;
        color: #f1f5f9;
      }
      .swagger-ui .topbar { display: none; }
      .swagger-ui { background: #0f172a; color: #f1f5f9; }
      .swagger-ui .info .title { color: #f1f5f9; }
      .swagger-ui .info p, .swagger-ui .info li { color: #94a3b8; }
      .swagger-ui .opblock .opblock-summary-operation-id, .swagger-ui .opblock .opblock-summary-path, .swagger-ui .opblock .opblock-summary-path__deprecated { color: #f8fafc; }
      .swagger-ui .opblock-description-wrapper p, .swagger-ui .opblock-external-docs-wrapper p, .swagger-ui .opblock-title_normal p { color: #cbd5e1; }
    </style>
  </head>
  <body>
    <div id="swagger-ui"></div>
    <script src="/docs/assets/swagger-ui-bundle.js"></script>
    <script src="/docs/assets/swagger-ui-standalone-preset.js"></script>
    <script>
      window.onload = () => {
        window.ui = SwaggerUIBundle({
          url: '/api/openapi.json',
          dom_id: '#swagger-ui',
          deepLinking: true,
          presets: [
            SwaggerUIBundle.presets.apis,
            SwaggerUIStandalonePreset
          ],
          layout: "StandaloneLayout"
        });
      };
    </script>
  </body>
</html>"#
    .to_string())
}

/// Retorna a especificação OpenAPI 3.0.3 completa do OpenPresenter.
pub async fn get_openapi_spec() -> impl IntoResponse {
    let spec = build_openapi_spec();
    Json(spec)
}

fn build_openapi_spec() -> Value {
    json!({
      "openapi": "3.0.3",
      "info": {
        "title": "OpenPresenter Minimal API",
        "version": op_core::VERSION,
        "description": "API REST e WebSocket para controle de projeção, repertório de louvor, escrituras bíblicas, equipe de mídia (Teams), escalas de culto e integrações externas (Planning Center, Voluts, bots e automações).\n\nDocumentação interativa 100% offline disponível via Scalar (`/docs` ou `/scalar`) e Swagger UI (`/swagger`).",
        "contact": {
          "name": "OpenPresenter Community",
          "url": "https://github.com/open-mission/openpresenter"
        },
        "license": {
          "name": "GPL-3.0-only",
          "url": "https://www.gnu.org/licenses/gpl-3.0.html"
        }
      },
      "servers": [
        {
          "url": "/",
          "description": "Servidor Local do OpenPresenter"
        }
      ],
      "tags": [
        { "name": "Teams", "description": "Gerenciamento de voluntários, departamentos da equipe de mídia e escalas de culto (Service Roster)." },
        { "name": "Presentation", "description": "Controle de projeção em tempo real na tela pública e retorno de palco." },
        { "name": "Timer & Stage", "description": "Controle do cronômetro sincronizado, avisos de palco e overtime." },
        { "name": "Songs", "description": "Catálogo de louvores com busca textual SQLite FTS5 e importação de cifras/letras." },
        { "name": "Setlists", "description": "Programação de culto, ordem do louvor e mídias associadas." },
        { "name": "Bibles", "description": "Bíblias em arquivos SQLite desacoplados, busca rápida e projeção de versículos." },
        { "name": "Media", "description": "Papéis de parede, imagens, pastas de mídias locais e uploads." },
        { "name": "Settings", "description": "Preferências do operador, margens de projeção e marca da congregação." },
        { "name": "System", "description": "Status, identificação na rede local, auditoria de ações e reinício do aplicativo." }
      ],
      "paths": {
        "/api/teams/departments": {
          "get": {
            "tags": ["Teams"],
            "summary": "Listar departamentos da equipe de mídia",
            "description": "Retorna todos os departamentos cadastrados (Projeção, Fotografia, Stories, Transmissão, Som, etc.) ordenados por prioridade.",
            "responses": {
              "200": {
                "description": "Lista de departamentos obtida com sucesso.",
                "content": { "application/json": { "schema": { "type": "array", "items": { "$ref": "#/components/schemas/TeamDepartment" } } } }
              }
            }
          },
          "post": {
            "tags": ["Teams"],
            "summary": "Criar novo departamento",
            "requestBody": {
              "required": true,
              "content": { "application/json": { "schema": { "$ref": "#/components/schemas/UpsertTeamDepartment" } } }
            },
            "responses": {
              "200": { "description": "Departamento criado com sucesso.", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/TeamDepartment" } } } }
            }
          }
        },
        "/api/teams/departments/{id}": {
          "get": {
            "tags": ["Teams"],
            "summary": "Obter departamento por ID",
            "parameters": [{ "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }],
            "responses": {
              "200": { "description": "Departamento encontrado.", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/TeamDepartment" } } } },
              "404": { "description": "Departamento não encontrado." }
            }
          },
          "put": {
            "tags": ["Teams"],
            "summary": "Atualizar departamento existente",
            "parameters": [{ "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }],
            "requestBody": { "required": true, "content": { "application/json": { "schema": { "$ref": "#/components/schemas/UpsertTeamDepartment" } } } },
            "responses": {
              "200": { "description": "Departamento atualizado.", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/TeamDepartment" } } } }
            }
          },
          "delete": {
            "tags": ["Teams"],
            "summary": "Excluir departamento",
            "parameters": [{ "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }],
            "responses": {
              "204": { "description": "Departamento excluído com sucesso." },
              "404": { "description": "Departamento não encontrado." }
            }
          }
        },
        "/api/teams/members": {
          "get": {
            "tags": ["Teams"],
            "summary": "Listar voluntários da equipe de mídia",
            "parameters": [
              { "name": "include_inactive", "in": "query", "required": false, "schema": { "type": "boolean", "default": false }, "description": "Incluir membros inativos." }
            ],
            "responses": {
              "200": { "description": "Lista de voluntários.", "content": { "application/json": { "schema": { "type": "array", "items": { "$ref": "#/components/schemas/TeamMember" } } } } }
            }
          },
          "post": {
            "tags": ["Teams"],
            "summary": "Cadastrar novo voluntário",
            "requestBody": { "required": true, "content": { "application/json": { "schema": { "$ref": "#/components/schemas/UpsertTeamMember" } } } },
            "responses": {
              "200": { "description": "Voluntário cadastrado.", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/TeamMember" } } } }
            }
          }
        },
        "/api/teams/members/{id}": {
          "get": {
            "tags": ["Teams"],
            "summary": "Obter voluntário por ID",
            "parameters": [{ "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }],
            "responses": {
              "200": { "description": "Dados do voluntário.", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/TeamMember" } } } },
              "404": { "description": "Voluntário não encontrado." }
            }
          },
          "put": {
            "tags": ["Teams"],
            "summary": "Atualizar dados do voluntário",
            "parameters": [{ "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }],
            "requestBody": { "required": true, "content": { "application/json": { "schema": { "$ref": "#/components/schemas/UpsertTeamMember" } } } },
            "responses": {
              "200": { "description": "Voluntário atualizado.", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/TeamMember" } } } }
            }
          },
          "delete": {
            "tags": ["Teams"],
            "summary": "Excluir voluntário",
            "parameters": [{ "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }],
            "responses": {
              "204": { "description": "Voluntário excluído." },
              "404": { "description": "Voluntário não encontrado." }
            }
          }
        },
        "/api/teams/rosters": {
          "get": {
            "tags": ["Teams"],
            "summary": "Listar escalas de culto",
            "responses": {
              "200": { "description": "Lista de resumos de escalas.", "content": { "application/json": { "schema": { "type": "array", "items": { "$ref": "#/components/schemas/ServiceRosterSummary" } } } } }
            }
          },
          "post": {
            "tags": ["Teams"],
            "summary": "Criar nova escala de culto",
            "requestBody": { "required": true, "content": { "application/json": { "schema": { "$ref": "#/components/schemas/UpsertServiceRoster" } } } },
            "responses": {
              "200": { "description": "Escala criada.", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ServiceRoster" } } } }
            }
          }
        },
        "/api/teams/rosters/active": {
          "get": {
            "tags": ["Teams"],
            "summary": "Obter escala ativa do culto corrente",
            "responses": {
              "200": { "description": "Escala atualmente ativa.", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ServiceRoster" } } } },
              "404": { "description": "Nenhuma escala marcada como ativa no momento." }
            }
          }
        },
        "/api/teams/rosters/{id}": {
          "get": {
            "tags": ["Teams"],
            "summary": "Obter escala detalhada com voluntários alocados",
            "parameters": [{ "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }],
            "responses": {
              "200": { "description": "Detalhes da escala.", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ServiceRoster" } } } },
              "404": { "description": "Escala não encontrada." }
            }
          },
          "put": {
            "tags": ["Teams"],
            "summary": "Atualizar dados e itens da escala",
            "parameters": [{ "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }],
            "requestBody": { "required": true, "content": { "application/json": { "schema": { "$ref": "#/components/schemas/UpsertServiceRoster" } } } },
            "responses": {
              "200": { "description": "Escala atualizada.", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ServiceRoster" } } } }
            }
          },
          "delete": {
            "tags": ["Teams"],
            "summary": "Excluir escala de culto",
            "parameters": [{ "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }],
            "responses": {
              "204": { "description": "Escala excluída." },
              "404": { "description": "Escala não encontrada." }
            }
          }
        },
        "/api/teams/rosters/{id}/active": {
          "post": {
            "tags": ["Teams"],
            "summary": "Definir escala como a escala ativa do culto",
            "parameters": [{ "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }],
            "responses": {
              "200": { "description": "Escala ativada com sucesso." }
            }
          }
        },
        "/api/teams/rosters/{id}/items": {
          "post": {
            "tags": ["Teams"],
            "summary": "Alocar voluntário a departamento na escala",
            "parameters": [{ "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }],
            "requestBody": { "required": true, "content": { "application/json": { "schema": { "$ref": "#/components/schemas/UpsertServiceRosterItem" } } } },
            "responses": {
              "200": { "description": "Voluntário alocado com sucesso.", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ServiceRosterItem" } } } }
            }
          }
        },
        "/api/teams/rosters/{id}/items/{item_id}": {
          "patch": {
            "tags": ["Teams"],
            "summary": "Atualizar status de confirmação do voluntário na escala",
            "parameters": [
              { "name": "id", "in": "path", "required": true, "schema": { "type": "string" } },
              { "name": "item_id", "in": "path", "required": true, "schema": { "type": "string" } }
            ],
            "requestBody": {
              "required": true,
              "content": {
                "application/json": {
                  "schema": {
                    "type": "object",
                    "properties": {
                      "status": { "type": "string", "enum": ["confirmed", "pending", "declined"] }
                    },
                    "required": ["status"]
                  }
                }
              }
            },
            "responses": {
              "200": { "description": "Status atualizado com sucesso." }
            }
          },
          "delete": {
            "tags": ["Teams"],
            "summary": "Remover voluntário da escala",
            "parameters": [
              { "name": "id", "in": "path", "required": true, "schema": { "type": "string" } },
              { "name": "item_id", "in": "path", "required": true, "schema": { "type": "string" } }
            ],
            "responses": {
              "204": { "description": "Alocação removida." },
              "404": { "description": "Item não encontrado." }
            }
          }
        },
        "/api/status": {
          "get": {
            "tags": ["System"],
            "summary": "Status do servidor e versão",
            "responses": {
              "200": { "description": "Status operacional do OpenPresenter." }
            }
          }
        },
        "/api/system/metrics": {
          "get": {
            "tags": ["System"],
            "summary": "Telemetria de hardware e recursos (CPU, RAM, GPU, Uptime)",
            "responses": {
              "200": {
                "description": "Métricas em tempo real do processo e da máquina anfitriã.",
                "content": {
                  "application/json": {
                    "schema": { "$ref": "#/components/schemas/SystemMetrics" }
                  }
                }
              }
            }
          }
        },
        "/api/presentation/live": {
          "post": {
            "tags": ["Presentation"],
            "summary": "Definir mídia e slide ao vivo",
            "responses": { "200": { "description": "Slide projetado." } }
          }
        },
        "/api/presentation/stop": {
          "post": {
            "tags": ["Presentation"],
            "summary": "Parar projeção ao vivo",
            "responses": { "200": { "description": "Apresentação parada." } }
          }
        },
        "/api/timer": {
          "get": {
            "tags": ["Timer & Stage"],
            "summary": "Obter estado do cronômetro",
            "responses": { "200": { "description": "Estado atual do cronômetro." } }
          }
        },
        "/api/songs": {
          "get": {
            "tags": ["Songs"],
            "summary": "Buscar músicas no catálogo",
            "parameters": [{ "name": "q", "in": "query", "schema": { "type": "string" }, "description": "Termo para busca full-text FTS5" }],
            "responses": { "200": { "description": "Músicas encontradas." } }
          },
          "post": {
            "tags": ["Songs"],
            "summary": "Salvar nova música no catálogo",
            "responses": { "200": { "description": "Música salva." } }
          }
        },
        "/api/setlists": {
          "get": {
            "tags": ["Setlists"],
            "summary": "Listar setlists de cultos",
            "responses": { "200": { "description": "Lista de setlists." } }
          }
        },
        "/api/bibles": {
          "get": {
            "tags": ["Bibles"],
            "summary": "Listar versões da Bíblia importadas",
            "responses": { "200": { "description": "Versões bíblicas disponíveis." } }
          }
        }
      },
      "components": {
        "schemas": {
          "TeamDepartment": {
            "type": "object",
            "required": ["id", "name", "color", "icon", "display_order", "created_at", "updated_at"],
            "properties": {
              "id": { "type": "string", "example": "projecao" },
              "name": { "type": "string", "example": "Projeção" },
              "description": { "type": "string", "nullable": true, "example": "Operação de letras e bíblia no telão" },
              "color": { "type": "string", "example": "#3b82f6" },
              "icon": { "type": "string", "example": "MonitorPlay" },
              "display_order": { "type": "integer", "example": 1 },
              "created_at": { "type": "string", "format": "date-time" },
              "updated_at": { "type": "string", "format": "date-time" }
            }
          },
          "UpsertTeamDepartment": {
            "type": "object",
            "required": ["name"],
            "properties": {
              "name": { "type": "string", "example": "Transmissão (Live)" },
              "description": { "type": "string", "nullable": true },
              "color": { "type": "string", "example": "#8b5cf6" },
              "icon": { "type": "string", "example": "Video" },
              "display_order": { "type": "integer", "example": 2 }
            }
          },
          "TeamMember": {
            "type": "object",
            "required": ["id", "name", "color", "role", "is_active", "department_ids", "created_at", "updated_at"],
            "properties": {
              "id": { "type": "string", "example": "member-1730810000" },
              "name": { "type": "string", "example": "João Silva" },
              "phone": { "type": "string", "nullable": true, "example": "11988887777" },
              "email": { "type": "string", "nullable": true, "example": "joao@example.com" },
              "avatar_url": { "type": "string", "nullable": true },
              "color": { "type": "string", "example": "#10b981" },
              "role": { "type": "string", "enum": ["leader", "vice_leader", "member"], "example": "leader", "description": "Cargo do membro na equipe (ex: leader, vice_leader, member)" },
              "notes": { "type": "string", "nullable": true },
              "is_active": { "type": "boolean", "example": true },
              "department_ids": { "type": "array", "items": { "type": "string" }, "example": ["projecao", "stories"] },
              "created_at": { "type": "string", "format": "date-time" },
              "updated_at": { "type": "string", "format": "date-time" }
            }
          },
          "UpsertTeamMember": {
            "type": "object",
            "required": ["name"],
            "properties": {
              "name": { "type": "string", "example": "Maria Santos" },
              "phone": { "type": "string", "nullable": true, "example": "11977776666" },
              "email": { "type": "string", "nullable": true },
              "avatar_url": { "type": "string", "nullable": true },
              "color": { "type": "string", "example": "#ec4899" },
              "role": { "type": "string", "enum": ["leader", "vice_leader", "member"], "example": "member", "description": "Cargo do membro na equipe" },
              "notes": { "type": "string", "nullable": true },
              "is_active": { "type": "boolean", "example": true },
              "department_ids": { "type": "array", "items": { "type": "string" }, "example": ["fotografia"] }
            }
          },
          "ServiceRosterItem": {
            "type": "object",
            "required": ["id", "roster_id", "department_id", "member_id", "role", "status", "created_at"],
            "properties": {
              "id": { "type": "string", "example": "item-1730810000" },
              "roster_id": { "type": "string", "example": "roster-1730810000" },
              "department_id": { "type": "string", "example": "projecao" },
              "member_id": { "type": "string", "example": "member-1730810000" },
              "role": { "type": "string", "enum": ["lead", "assistant", "backup"], "example": "lead" },
              "status": { "type": "string", "enum": ["confirmed", "pending", "declined"], "example": "confirmed" },
              "role_notes": { "type": "string", "nullable": true, "example": "Operador titular" },
              "created_at": { "type": "string", "format": "date-time" }
            }
          },
          "UpsertServiceRosterItem": {
            "type": "object",
            "required": ["department_id", "member_id"],
            "properties": {
              "department_id": { "type": "string", "example": "projecao" },
              "member_id": { "type": "string", "example": "member-1730810000" },
              "role": { "type": "string", "example": "lead" },
              "status": { "type": "string", "example": "confirmed" },
              "role_notes": { "type": "string", "nullable": true }
            }
          },
          "ServiceRoster": {
            "type": "object",
            "required": ["id", "title", "service_date", "is_active", "items", "created_at", "updated_at"],
            "properties": {
              "id": { "type": "string", "example": "roster-1730810000" },
              "title": { "type": "string", "example": "Culto de Domingo - Noite" },
              "service_date": { "type": "string", "format": "date", "example": "2026-11-08" },
              "service_time": { "type": "string", "nullable": true, "example": "19:00" },
              "notes": { "type": "string", "nullable": true },
              "is_active": { "type": "boolean", "example": true },
              "items": { "type": "array", "items": { "$ref": "#/components/schemas/ServiceRosterItem" } },
              "created_at": { "type": "string", "format": "date-time" },
              "updated_at": { "type": "string", "format": "date-time" }
            }
          },
          "ServiceRosterSummary": {
            "type": "object",
            "required": ["id", "title", "service_date", "is_active", "item_count", "updated_at"],
            "properties": {
              "id": { "type": "string", "example": "roster-1730810000" },
              "title": { "type": "string", "example": "Culto de Domingo - Noite" },
              "service_date": { "type": "string", "format": "date", "example": "2026-11-08" },
              "service_time": { "type": "string", "nullable": true, "example": "19:00" },
              "is_active": { "type": "boolean", "example": true },
              "item_count": { "type": "integer", "example": 6 },
              "updated_at": { "type": "string", "format": "date-time" }
            }
          },
          "UpsertServiceRoster": {
            "type": "object",
            "required": ["title", "service_date"],
            "properties": {
              "title": { "type": "string", "example": "Culto de Domingo - Noite" },
              "service_date": { "type": "string", "format": "date", "example": "2026-11-08" },
              "service_time": { "type": "string", "nullable": true, "example": "19:00" },
              "notes": { "type": "string", "nullable": true },
              "is_active": { "type": "boolean", "example": true },
              "items": { "type": "array", "items": { "$ref": "#/components/schemas/UpsertServiceRosterItem" } }
            }
          },
          "SystemMetrics": {
            "type": "object",
            "required": [
              "process_memory_bytes",
              "process_memory_mb",
              "process_cpu_percent",
              "system_memory_total_bytes",
              "system_memory_used_bytes",
              "system_memory_total_mb",
              "system_memory_used_mb",
              "system_memory_percent",
              "system_cpu_percent",
              "uptime_seconds",
              "timestamp"
            ],
            "properties": {
              "process_memory_bytes": { "type": "integer", "example": 47185920 },
              "process_memory_mb": { "type": "number", "example": 45.0 },
              "process_cpu_percent": { "type": "number", "example": 1.5 },
              "system_memory_total_bytes": { "type": "integer", "example": 17179869184_i64 },
              "system_memory_used_bytes": { "type": "integer", "example": 8589934592_i64 },
              "system_memory_total_mb": { "type": "number", "example": 16384.0 },
              "system_memory_used_mb": { "type": "number", "example": 8192.0 },
              "system_memory_percent": { "type": "number", "example": 50.0 },
              "system_cpu_percent": { "type": "number", "example": 12.4 },
              "gpu_model": { "type": "string", "nullable": true, "example": "Apple M1" },
              "gpu_usage_percent": { "type": "number", "nullable": true, "example": 25.0 },
              "gpu_memory_used_mb": { "type": "number", "nullable": true, "example": 1024.0 },
              "gpu_memory_total_mb": { "type": "number", "nullable": true, "example": 8192.0 },
              "uptime_seconds": { "type": "integer", "example": 3600 },
              "timestamp": { "type": "integer", "example": 1730000000000_i64 }
            }
          }
        }
      }
    })
}
