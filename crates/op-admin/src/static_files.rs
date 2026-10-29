use std::path::{Path, PathBuf};

use axum::{
    body::Body,
    http::{Response, StatusCode, Uri, header},
};

/// Caminho para a pasta onde o frontend Svelte compilado reside (`frontend/dist`).
pub fn find_frontend_dist() -> PathBuf {
    if let Ok(dir) = std::env::var("OPENPRESENTER_FRONTEND_DIR") {
        let p = PathBuf::from(dir);
        if p.exists() {
            return p;
        }
    }

    // Procura na raiz do repositório ou caminhos relativos conhecidos
    let candidates = [
        PathBuf::from("frontend/dist"),
        PathBuf::from("../frontend/dist"),
        PathBuf::from("../../frontend/dist"),
    ];

    for candidate in &candidates {
        if candidate.exists() {
            return candidate.clone();
        }
    }

    PathBuf::from("frontend/dist")
}

/// Manipulador de fallback para rotas SPA (`/`, `/admin`, `/public`, `/stage`).
pub async fn static_fallback_handler(uri: Uri) -> Response<Body> {
    let path = uri.path();
    let dist_dir = find_frontend_dist();

    // Se o arquivo existir na pasta dist, entrega diretamente
    let relative_path = path.trim_start_matches('/');
    if !relative_path.is_empty() {
        let asset_file = dist_dir.join(relative_path);
        if asset_file.is_file() {
            return match serve_local_file(&asset_file).await {
                Ok(resp) => resp,
                Err(_) => fallback_html_response(),
            };
        }
    }

    // Se for uma rota SPA (como /, /admin, /public, /stage), serve o index.html da pasta dist
    let index_file = dist_dir.join("index.html");
    if index_file.is_file() {
        match serve_local_file(&index_file).await {
            Ok(resp) => resp,
            Err(_) => fallback_html_response(),
        }
    } else {
        fallback_html_response()
    }
}

async fn serve_local_file(path: &Path) -> Result<Response<Body>, std::io::Error> {
    let content = tokio::fs::read(path).await?;
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime.as_ref())
        .body(Body::from(content))
        .map_err(std::io::Error::other)?;

    Ok(response)
}

fn fallback_html_response() -> Response<Body> {
    let html = r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
  <meta charset="UTF-8">
  <title>OpenPresenter — Servidor Ativo</title>
  <style>
    body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #09090b; color: #f4f4f5; display: flex; align-items: center; justify-content: center; height: 100vh; margin: 0; }
    .card { background: #18181b; border: 1px solid #27272a; padding: 2.5rem; border-radius: 1rem; max-width: 520px; text-align: center; box-shadow: 0 10px 25px -5px rgba(0,0,0,0.5); }
    h1 { color: #22c55e; margin-top: 0; font-size: 1.8rem; }
    p { color: #a1a1aa; line-height: 1.6; }
    code { background: #27272a; padding: 0.2rem 0.5rem; border-radius: 0.3rem; font-size: 0.9em; color: #e4e4e7; }
    .links { margin-top: 1.5rem; display: flex; gap: 0.75rem; justify-content: center; }
    a { display: inline-block; padding: 0.6rem 1.2rem; background: #22c55e; color: #09090b; text-decoration: none; font-weight: 600; border-radius: 0.5rem; }
    a.sec { background: #27272a; color: #f4f4f5; }
  </style>
</head>
<body>
  <div class="card">
    <h1>OpenPresenter</h1>
    <p>O backend em <strong>Rust + Axum</strong> está rodando com sucesso na porta 3000.</p>
    <p>Para carregar a interface completa em Svelte com Tailwind e shadcn-ui, execute o build do frontend:</p>
    <p><code>cd frontend && bun run build</code></p>
    <div class="links">
      <a href="/api/status" class="sec">API Status</a>
      <a href="/api/media" class="sec">API Mídias</a>
    </div>
  </div>
</body>
</html>"#;

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(Body::from(html))
        .unwrap()
}
