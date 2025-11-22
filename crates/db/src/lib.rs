pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub async fn establish_connection(url: &str) -> Result<sqlx::PgPool, sqlx::Error> {
    let pool = sqlx::PgPool::connect(url).await?;
    Ok(pool)
}

pub async fn check_connection(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await.map(|_| ())
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
