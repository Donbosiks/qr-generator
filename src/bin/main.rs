use qr_code::*;
use crate::models::{establish_connection, create_post, find_identifier_value};
use dialoguer::{theme::ColorfulTheme, Input};
use qrcodes::render_qr;
use std::thread;
use tokio::runtime::Runtime;
use web_page::rocket;


#[rocket::main]
async fn main() {
    // Создаем новый поток
    thread::spawn(|| {
        // Создаем новый Tokio runtime
        let rt = Runtime::new().unwrap();

        // Запускаем асинхронную функцию в созданном runtime
        rt.block_on(async {
            // Ваша асинхронная функция
            let _start = web_page::rocket().launch().await;
        });
    });

    // Другие действия в основном потоке
    start()
}

fn start() {

    // thread::spawn(|| {
    //      web_page::rocket().launch()
    // });

    loop {
        let connection = &mut establish_connection();

        println!("What would you like your title to be?");

        let qr_type: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Your qr type:")
            .interact_text()
            .expect("Reason");

        let link: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Your link: ")
            .interact_text()
            .expect("Error");

        // Offline qr_code function without database insert

        if qr_type == "offline" {
            let _ = render_qr(&link, &qr_type);
            break
        }

        // Online qr_code function with database insert

        loop {
            let identifier = rand_identifier(5); // Generate random identifier for short link

            let result = find_identifier_value(connection, &identifier); // Check does exist identifier in db

            if  result == "exist" {
                continue
            }

            let _ = render_qr(&identifier, &qr_type);  // Generate Qrcode
            create_post(connection, &identifier, &link); // Insert Qrcode in db
            break
        }

        break
        }

}