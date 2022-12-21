use sqlx::SqlitePool;

#[derive(Debug)]
pub struct Data {
    pub db: SqlitePool,
}
