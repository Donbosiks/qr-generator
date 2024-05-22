use diesel_test::*;
use crate::models::establish_connection;
use dialoguer::{theme::ColorfulTheme, Input};
use qrcodes::render_qr;
fn main() {
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

        let indeficator: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Your indeficator:")
            .interact_text()
            .expect("Reason");


        let _ = render_qr(&indeficator, &qr_type);
        create_post(connection, &indeficator, &link);

        break

    }

}

// #[cfg(not(windows))]
// const EOF: &str = "CTRL+D";
//
// #[cfg(windows)]
// const EOF: &str = "CTRL+Z";
