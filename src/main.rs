use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use axum_learn_db;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_uri = dotenv!("DATABASE_URL"); // ambil value var dari .env
    axum_learn_db::run(db_uri).await;
}
