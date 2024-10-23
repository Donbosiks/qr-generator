

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
    MysqlConnection::establish(&database_url).expect("Expect")
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

    let _ins = diesel::insert_into(qrcode::table)
        .values(&new_qr)
        .execute(conn);
}

pub fn find_identifier_value<'a>(conn: &'a mut MysqlConnection, search_value: &'a str) -> bool {
    use crate::schema::qrcode::dsl::*;

    let result = select(exists(qrcode.filter(identifier.eq(&search_value))))
        .get_result::<bool>(conn);

    match result {
        Ok(true) => true,
        Ok(false) => false,
        // Err(_) => println!("An error while searching identifier"),
        Err(_) => todo!(),

}}

pub fn find_link(ind: &str) -> Option<String>  {
    use crate::schema::qrcode::dsl::*;
    let connection = &mut establish_connection();

    let res = qrcode
        .select(link)
        .filter(identifier.eq(ind))
        .first::<String>(connection);

    match res {
        Ok(_) => Some(res.expect("Error to operation with finding link")),
        // Err(_) => return "none".into_string(),
        Err(_) => None,

    }

}