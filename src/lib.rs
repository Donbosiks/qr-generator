pub mod schema;
pub mod models;
pub mod qrcodes;

use models::{NewQr};
use diesel::MysqlConnection;
use diesel::RunQueryDsl;

pub fn create_post(conn: &mut MysqlConnection, indeficator: &str, link: &str) -> () {
    use crate::schema::qrcode;

    let new_qr = NewQr { indeficator, link };

    diesel::insert_into(qrcode::table)
        .values(&new_qr)
        .execute(conn)
        .expect("Error saving new post");
}