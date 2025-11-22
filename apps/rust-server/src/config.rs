use std::env;

pub fn build_database_url_from_env() -> Result<String, env::VarError> {
    dotenvy::dotenv().ok();

    let database_host = env::var("DB_HOST")?;
    let database_user = env::var("DB_USER")?;
    let database_password = env::var("DB_PASSWORD")?;
    let database_name = env::var("DB_NAME")?;
    let database_port = env::var("DB_PORT").unwrap_or_else(|_| "5432".into()); // Postgres デフォルトポート5432を使用

    Ok(format!(
        "postgres://{}:{}@{}:{}/{}",
        database_user, database_password, database_host, database_port, database_name
    ))
}
