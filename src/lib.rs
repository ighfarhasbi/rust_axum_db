mod routes;
mod database;
use sea_orm::Database;

pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await.unwrap(); // konek ke db, dan db harus jalan
    let app = routes::create_routes(database).await; // membuat jalur awal routing
   
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(); // port server aplikasi
    axum::serve(listener, app).await.unwrap(); // run server aplikasi dan db
}