use axum::{extract::{Path, Query}, http::StatusCode, Extension, Json};
use sea_orm::{prelude::Date, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::database::data_mahasiswa;

// use super::response_format; // untuk panggil file response_format

#[derive(Serialize)] // tampilkan value ke response
pub struct ResultMahasiswa {
    id: i32,
    nama: Option<String>,
    alamat: Option<String>,
    tgl_lahir: Option<Date>,
}

#[derive(Deserialize)] // ambil value dari request 
pub struct QueryParam {
    alamat: Option<String>,
}

pub async fn one_mahasiswa(Path(id): Path<i32>, Extension(database): Extension<DatabaseConnection>) -> Result<Json<ResultMahasiswa>, StatusCode> {
    let one_mahasiswa = data_mahasiswa::Entity::find_by_id(id).one(&database).await.unwrap();

    if let Some(one_mahasiswa) = one_mahasiswa {
        Ok(Json(ResultMahasiswa {
            id: one_mahasiswa.id,
            nama: one_mahasiswa.nama,
            alamat: one_mahasiswa.alamat,
            tgl_lahir: one_mahasiswa.tgl_lahir,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_all_mahasiswa(Extension(database): Extension<DatabaseConnection>, Query(query_param): Query<QueryParam>) -> Result<Json<Vec<ResultMahasiswa>>, StatusCode> {
    let mut alamat_filter = Condition::all();
    if let Some(alamat) = query_param.alamat {
        alamat_filter = alamat_filter.add(data_mahasiswa::Column::Alamat.eq(alamat))
    };
    let mahasiswa = data_mahasiswa::Entity::find()
    .filter(alamat_filter)
    .all(&database)
    .await
    .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
    .into_iter()
    .map(|db_mahasiswa| ResultMahasiswa {
        id: db_mahasiswa.id,
        nama: db_mahasiswa.nama,
        alamat: db_mahasiswa.alamat,
        tgl_lahir: db_mahasiswa.tgl_lahir,
    })
    .collect();

    Ok(Json(mahasiswa))
}

// pub async fn one_mahasiswa(Path(id): Path<i32>, Extension(database): Extension<DatabaseConnection>) -> Result<Json<response_format::Response>, StatusCode> {
//     let one_mahasiswa = data_mahasiswa::Entity::find_by_id(id).one(&database).await.unwrap();

//     if let Some(one_mahasiswa) = one_mahasiswa {
//         Ok(Json(response_format::Response {
//             success: true,
//             message: format!("Mahasiswa dengan id {} tidak ditemukan", id),
//             data: one_mahasiswa,
//         }))
//     } else {
//         Err(StatusCode::NOT_FOUND)
//     }
// }