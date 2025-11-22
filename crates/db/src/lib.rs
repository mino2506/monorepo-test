use std::env;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn get_connection() -> String {
    dotenvy::dotenv().ok();

    let database_host = env::var("DB_HOST").unwrap();
    let database_user = env::var("DB_USER").unwrap();
    let database_password = env::var("DB_PASSWORD").unwrap();
    let database_name = env::var("DB_NAME").unwrap();
    let database_port = env::var("DB_PORT").unwrap_or_else(|_| "5432".into());

    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        database_user, database_password, database_host, database_port, database_name
    );

    url
}

pub async fn establish_connection() -> Result<sqlx::PgPool, sqlx::Error> {
    let database_url = get_connection();
    let pool = sqlx::PgPool::connect(&database_url).await?;
    Ok(pool)
}

pub async fn check_connection() -> Result<(), sqlx::Error> {
    let pool = establish_connection().await?;
    let (_value,): (i32,) = sqlx::query_as("SELECT 1").fetch_one(&pool).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
