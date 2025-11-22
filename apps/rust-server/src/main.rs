use axum::Router;
use dotenvy::dotenv;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

use db::{add, check_connection, get_connection};

mod error;
mod routes;

#[tokio::main]
async fn main() {
    println!("2 + 3 = {}\n", add(2, 3));
    println!("database_url = {}\n", get_connection());
    if let Err(e) = run().await {
        eprintln!("fatal error: {e}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // .env読み込み
    dotenv().ok();

    let query = check_connection().await;

    match query {
        Ok(_value) => {
            println!("Database connection test succeeded");
        }
        Err(e) => {
            println!("Database connection test failed: {}", e);
        }
    }

    // ログ初期化（最低限）
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("start Logging successfully");

    // ルータ定義
    let app: Router = routes::router();

    // アドレス
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on http://{}", addr);

    // 起動
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
