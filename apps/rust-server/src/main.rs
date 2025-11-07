use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
// use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // ログ初期化（最低限）
    // tracing_subscriber::fmt()
    //     .with_env_filter(EnvFilter::from_default_env())
    //     .init();

    // ルータ定義
    let app: Router = Router::new()
      .route("/health", get(health))
      .route("/hello", get(hello))
      .route("/echo", post(echo));

    // アドレス
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on http://{}", addr);

    // 起動
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
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
