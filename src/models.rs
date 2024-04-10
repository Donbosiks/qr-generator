use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use crate::schema::qrcode;

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
    pub indeficator: &'a str,
    pub link: &'a str,
}