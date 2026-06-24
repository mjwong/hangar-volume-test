use axum::{routing::get, Router};
use std::env;

#[tokio::main]
async fn main() {
    let base = env::var("HANGAR_BASE_PATH").unwrap_or_default();
    let dep_id = env::var("HANGAR_DEPLOYMENT_ID").unwrap_or_else(|_| "unknown".into());

    let app = Router::new()
        .route(&format!("{}/", base), get({
            let base = base.clone();
            let dep_id = dep_id.clone();
            move || root(base.clone(), dep_id.clone())
        }))
        .route(&format!("{}/health", base), get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("hangar-hello-rust listening on :8080");
    axum::serve(listener, app).await.unwrap();
}

async fn root(base: String, dep_id: String) -> axum::response::Html<String> {
    axum::response::Html(format!(r#"<!DOCTYPE html>
<html>
<head><title>Hello from Rust</title></head>
<body>
  <h1>Hello from Rust + Axum</h1>
  <p>Deployment: <code>{dep_id}</code></p>
  <p>Base path: <code>{base}</code></p>
</body>
</html>"#))
}

async fn health() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({"ok": true}))
}
