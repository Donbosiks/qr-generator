

// models.rs is rust code where me work with diesel and MySql function
// there placed all function, who needed to connect and create new posts





use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use crate::schema::qrcode;
use diesel::RunQueryDsl;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// #[derive(Queryable, Selectable)]
// #[diesel(table_name = crate::schema::qrcode)]
// #[diesel(check_for_backend(diesel::mysql::Mysql))]
// pub struct qr_code {
//     pub id: i32,
//     pub indeficator: String,
//     pub link: String,
// }

#[derive(Insertable)]
#[diesel(table_name = qrcode)]
pub struct NewQr<'a> {
    pub identifier: &'a str,
    pub link: &'a str,
}

pub fn create_post(conn: &mut MysqlConnection, identifier: &str, link: &str) -> () {
    use crate::schema::qrcode;

    let new_qr = NewQr { identifier, link };

    diesel::insert_into(qrcode::table)
        .values(&new_qr)
        .execute(conn)
        .expect("Error saving new post");
}

