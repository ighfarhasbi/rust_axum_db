use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

use crate::database::data_mahasiswa;

#[derive(Serialize)]
pub struct ResponseBody {
    message: String,
}
pub async fn delete_mahasiswa_id (
    Path(id_mahasiswa): Path<i32>, 
    Extension(database): Extension<DatabaseConnection>
) -> Result<Json<ResponseBody>, StatusCode> {
    data_mahasiswa::Entity::delete_by_id(id_mahasiswa)
        .exec(&database).await
        .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?;
    let response_body = ResponseBody {
        message: format!("Mahasiswa dengan id {} berhasil di hapus", id_mahasiswa),
    };
    Ok(Json(ResponseBody {message: response_body.message}))
}