use qr_code::*;
use std::thread;
use tokio::runtime::Runtime;
use web_page::rocket;
use crate::models::create_qr;


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

    loop {

        create_qr();

        }

}