use sqlx::SqlitePool;

#[derive(Debug)]
pub struct Data {
    pub db: SqlitePool,
}

impl Default for Data {
    async fn default() -> Self {
        let mut db_options = SqliteConnectOptions::default()
            .filename("burst.db")
            .create_if_missing(true);

        let db_options = db_options
            .log_statements(LevelFilter::Debug)
            // TODO: Fine tune the duration.
            .log_slow_statements(LevelFilter::Warn, Duration::from_secs(1));

        let db_pool = SqlitePoolOptions::default()
            .max_connections(100)
            .connect_with(db_options.clone())
            .await?;

        Self { db: db_pool }
    }
}
