

// models.rs is rust code where me work with diesel and MySql function
// there placed all function, who needed to connect and create new posts




use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use diesel::RunQueryDsl;
use diesel::select;
use diesel::dsl::exists;
use dialoguer::{theme::ColorfulTheme, Input};
use crate::qrcodes::render_qr;
use crate::rand_identifier;
use crate::schema::qrcode;


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

pub fn create_qr() -> () {
    let qr_type: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Your qr type:")
            .interact_text()
            .expect("Reason");

        let link: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Your link: ")
            .interact_text()
            .expect("Error");

        let name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Your qrcode name: ")
            .interact_text()
            .expect("Error");

        qr_type_detect(&qr_type ,&name, &link)
}

fn qr_type_detect(qr_type: &str, name: &str, link: &str) -> () {
    if *qr_type == *"online" {
        let connection = &mut establish_connection();
        let identifier = generate_identifier(connection);
        let _ = render_qr(&name, &identifier, &qr_type);  // Generate Qrcode
        create_post(connection, &identifier, &link); // Insert Qrcode in db

        println!("QR is done");

        return
    }
    let _ = render_qr(&name, &link, &qr_type);
    
}

fn generate_identifier(connection: &mut MysqlConnection) -> String {
    let identifier = rand_identifier(5); // Generate random identifier for short link
    let result = find_identifier_value(connection, &identifier); // Check does exist identifier in db

    println!("Identifier is {}", &identifier);

    match result {
        true => generate_identifier(connection),
        false => identifier.to_owned(),

    }
}


pub fn find_identifier_value<'a>(conn: &'a mut MysqlConnection, search_value: &'a str) -> bool {
    use crate::schema::qrcode::dsl::*;

    let result = select(exists(qrcode.filter(identifier.eq(&search_value))))
        .get_result::<bool>(conn);

    match result {
        Ok(true) => true,
        Ok(false) => false,
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
        Err(_) => None,

    }

}