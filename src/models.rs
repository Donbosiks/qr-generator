

// models.rs is rust code where me work with diesel and MySql function
// there placed all function, who needed to connect and create new posts




use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use crate::schema::qrcode;
use diesel::RunQueryDsl;
use diesel::select;
use diesel::dsl::exists;


pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

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

pub fn find_identifier_value(search_value: &str) -> &str {
    let connection = &mut establish_connection();
    use crate::schema::qrcode::dsl::*;

    let result = select(exists(qrcode.filter(crate::schema::qrcode::dsl::identifier.eq(&search_value))))
        .get_result::<bool>(connection);

    match result {
        Ok(true) => return "exist",
        Ok(false) => return "not exist",
        // Err(_) => println!("An error while searching identifier"),
        Err(_) => todo!(),

}}