use axum::{routing::{get, post}, Router};
use chrono::Utc;
use std::{env, fs};

const DATA_FILE: &str = "/data/log.txt";

#[tokio::main]
async fn main() {
    let base = env::var("HANGAR_BASE_PATH").unwrap_or_default();

    fs::create_dir_all("/data").ok();

    let app = Router::new()
        .route(&format!("{}/", base), get(index))
        .route(&format!("{}/write", base), post(write))
        .route(&format!("{}/read", base), get(read))
        .route(&format!("{}/health", base), get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("hangar-volume-test listening on :8080");
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> axum::response::Html<&'static str> {
    axum::response::Html(r#"<!DOCTYPE html>
<html>
<head><title>Volume Test</title></head>
<body style="font-family:monospace;max-width:600px;margin:2rem auto;padding:0 1rem;">
  <h2>Hangar Volume Test</h2>
  <p>Tests writing and reading from <code>/data/log.txt</code> (persistent volume).</p>
  <form method="POST" action="write">
    <button type="submit">Write timestamp to /data/log.txt</button>
  </form>
  <br>
  <a href="read">Read /data/log.txt</a>
</body>
</html>"#)
}

async fn write() -> axum::response::Html<String> {
    let line = format!("{}\n", Utc::now().to_rfc3339());
    let result = match fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(DATA_FILE)
        .and_then(|mut f| { use std::io::Write; f.write_all(line.as_bytes()) })
    {
        Ok(_) => format!("Written: {}", line.trim()),
        Err(e) => format!("ERROR: {e}"),
    };
    axum::response::Html(format!(r#"<!DOCTYPE html>
<html><body style="font-family:monospace;max-width:600px;margin:2rem auto;padding:0 1rem;">
<p>{result}</p>
<a href="./">← Back</a> | <a href="read">Read log</a>
</body></html>"#))
}

async fn read() -> axum::response::Html<String> {
    let content = fs::read_to_string(DATA_FILE)
        .unwrap_or_else(|e| format!("(no data — {e})"));
    let lines: Vec<&str> = content.lines().collect();
    let body = if lines.is_empty() {
        "(empty)".to_string()
    } else {
        lines.iter().map(|l| format!("{l}<br>")).collect()
    };
    axum::response::Html(format!(r#"<!DOCTYPE html>
<html><body style="font-family:monospace;max-width:600px;margin:2rem auto;padding:0 1rem;">
<h2>/data/log.txt ({} lines)</h2>
<div style="background:#f5f5f5;padding:1rem;border-radius:4px;">{body}</div>
<br><a href="./">← Back</a>
</body></html>"#, lines.len()))
}

async fn health() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({"ok": true}))
}
