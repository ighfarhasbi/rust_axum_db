use axum::{http::Method, routing::{delete, get, patch, post, put}, Extension, Router};
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

mod create_data_mahasiswa;
mod get_mahasiswa;
mod update_mahasiswa;
mod update_partial_mahasiswa;
mod delete_mahasiswa;

pub mod response_format;

pub async fn create_routes(database: DatabaseConnection) -> Router {
    let cors = CorsLayer::new()
    .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_credentials(false);

    Router::new()
    .route("/mahasiswa", post(create_data_mahasiswa::create_data_mahasiswa))
    .route("/mahasiswa", get(get_mahasiswa::get_all_mahasiswa))
    .route("/mahasiswa/:id", get(get_mahasiswa::one_mahasiswa))
    .route("/mahasiswa/:id", put(update_mahasiswa::update_data_mahasiswa))
    .route("/mahasiswa/:id", patch(update_partial_mahasiswa::update_partial_data_mahasiswa))
    .route("/mahasiswa/:id", delete(delete_mahasiswa::delete_mahasiswa_id))
    .layer(Extension(database))
    .layer(cors)
    

}
