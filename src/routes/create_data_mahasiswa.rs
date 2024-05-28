use axum::{Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::Deserialize;
use chrono::NaiveDate;
// use sea_orm::EntityTrait; // dependensi untuk insert many

use crate::database::data_mahasiswa;

#[derive(Deserialize)]
pub struct RequestUser {
    id: i32,
    nama: String,
    alamat: String,
    tgl_lahir: String,
}

pub async fn create_data_mahasiswa(Extension(database): Extension<DatabaseConnection>, Json(req_user): Json<RequestUser>) {
   
    // Parse string menjadi NaiveDate
    let tgl_lahir = match NaiveDate::parse_from_str(&req_user.tgl_lahir, "%Y-%m-%d") {
        Ok(date) => Some(date),
        Err(e) => {
            println!("Error parsing date: {}", e);
            None
        }
    };
    
    let new_mahasiswa = data_mahasiswa::ActiveModel { 
        id: Set(req_user.id), 
        nama: Set(Some(req_user.nama)), 
        alamat: Set(Some(req_user.alamat)), 
        tgl_lahir: Set(tgl_lahir),
        ..Default::default() // untuk mengisi otomatis field yang tidak di definisikan
    };

    let _ = new_mahasiswa.insert(&database).await; // perintah insert 1 data ke db
    // dbg!(result);

    

    // let new_mahasiswa = data_mahasiswa::ActiveModel { 
    //     id: Set(9), 
    //     nama: Set(Some("Opung".to_owned())), 
    //     alamat: Set(Some("Jln. Sumarna Tagra 12".to_owned())), 
    //     tgl_lahir: Set(Some(Date::from_ymd(1989, 9, 9)).to_owned()),
    //     ..Default::default()
    // };

    // let new_mahasiswa1 = data_mahasiswa::ActiveModel { 
    //     id: Set(10), 
    //     nama: Set(Some("Aping".to_owned())), 
    //     alamat: Set(Some("Jln. Sumarna 1200".to_owned())), 
    //     tgl_lahir: Set(Some(Date::from_ymd(2001, 12, 1)).to_owned()),
    //     ..Default::default()
    // };

    // let res = data_mahasiswa::Entity::insert_many([new_mahasiswa, new_mahasiswa1]).exec(&database).await; // perintah insert lebih dari 1 data ke db
    // dbg!(res);
}
