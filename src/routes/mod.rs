use axum::{http::Method, routing::post, Extension, Router};
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

mod create_data_mahasiswa;

pub async fn create_routes(database: DatabaseConnection) -> Router {
    let cors = CorsLayer::new()
    .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_credentials(false);

    Router::new()
    .route("/create_mahasiswa", post(create_data_mahasiswa::create_data_mahasiswa))
    .layer(Extension(database))
    .layer(cors)
    

}
