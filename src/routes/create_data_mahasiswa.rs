use axum::Extension;
use sea_orm::{prelude::Date, ActiveModelTrait, DatabaseConnection, Set};
// use sea_orm::EntityTrait; // dependensi untuk insert many

use crate::database::data_mahasiswa;

pub async fn create_data_mahasiswa(Extension(database): Extension<DatabaseConnection>) {
    let new_mahasiswa = data_mahasiswa::ActiveModel { 
        id: Set(8), 
        nama: Set(Some("Opung".to_owned())), 
        alamat: Set(Some("Jln. Sumarna Tagra 12".to_owned())), 
        tgl_lahir: Set(Some(Date::from_ymd(1989, 9, 9)).to_owned()),
        ..Default::default()
    };

    let result = new_mahasiswa.insert(&database).await; // perintah insert 1 data ke db
    dbg!(result);


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
