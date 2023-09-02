use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use tera::Tera;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub tmpl: Tera,
}

pub async fn get_appstate() -> AppState {
    let database_url: String = dotenvy::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let pool: MySqlPool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .unwrap();
    let tera = Tera::new("templates/*").unwrap();
    let app_state = AppState { pool: pool, tmpl: tera };
    return app_state;
}

#[derive(Clone)]
pub struct DB {
    pub pool: MySqlPool,
}

impl DB {
    pub async fn new() -> Self {
        let database_url: String = dotenvy::var("DATABASE_URL").expect("DATABASE_URL should be set");
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(database_url.as_str())
            .await
            .unwrap();
        let db = DB {pool: pool};
        return db
    }
}
