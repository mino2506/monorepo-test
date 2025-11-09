use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing;
use tracing_subscriber::EnvFilter;
use dotenvy::dotenv;
#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("fatal error: {e}");
        std::process::exit(1);
    }
}

async fn run()-> Result<(), Box<dyn std::error::Error>> {
    // .env読み込み
    dotenv().ok();

    // ログ初期化（最低限）
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("start Logging successfully");

    // ルータ定義
    let app: Router = Router::new()
        .route("/health", get(health))
        .route("/hello", get(hello))
        .route("/echo", post(echo))
        .layer(TraceLayer::new_for_http());

    // アドレス
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on http://{}", addr);

    // 起動
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}



// ハンドラ（なんでもいいが文字列返却が一番シンプル）
async fn health() -> &'static str {
    "ok"
}

// JSON返すだけの例
#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

async fn hello() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "hello from axum".into(),
    })
}

// JSON受け取って、そのまま返す例
#[derive(Deserialize, Serialize)]
struct EchoPayload {
    text: String,
}

async fn echo(Json(payload): Json<EchoPayload>) -> Json<EchoPayload> {
    Json(payload)
}
