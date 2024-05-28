use axum::{extract::Path, http::StatusCode, Extension, Json};
use chrono::NaiveDate;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::database::data_mahasiswa::{self};

#[derive(Deserialize)]
pub struct RequestBody {
    // id: i32,
    nama: String,
    alamat: Option<String>,
    tgl_lahir: String,
}

#[derive(Serialize)]
pub struct ResponseBody {
   message: String,
   id: i32,
   nama: String,
   alamat: String,
   tgl_lahir: String,
}
pub async fn update_data_mahasiswa(
    Path(id_mahasiswa): Path<i32>, 
    Extension(database): Extension<DatabaseConnection>,
    Json(request_body): Json<RequestBody>
) -> Result<Json<ResponseBody>, StatusCode>{
    // Parse string menjadi NaiveDate
    let tgl_lahir = match NaiveDate::parse_from_str(&request_body.tgl_lahir, "%Y-%m-%d") {
        Ok(date) => Some(date),
        Err(e) => {
            println!("Error parsing date: {}", e);
            None
        }
    };

    let update_request = data_mahasiswa::ActiveModel {
        id: Set(id_mahasiswa),
        nama: Set(Some(request_body.nama)),
        alamat: Set(request_body.alamat),
        tgl_lahir: Set(tgl_lahir),
    };

    let result = data_mahasiswa::Entity::update(update_request)
        .filter(data_mahasiswa::Column::Id.eq(id_mahasiswa))
        .exec(&database)
        .await
        .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?;
    let response_body = ResponseBody {
        message: format!("Mahasiswa dengan id {} berhasil di update", id_mahasiswa),
        id: result.id,
        nama: result.nama.unwrap_or_default(),
        alamat: result.alamat.unwrap_or_default(),
        tgl_lahir: result.tgl_lahir.unwrap_or_default().to_string(),
    };

        Ok(Json(response_body))
}