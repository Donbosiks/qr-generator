use diesel_test::*;
use crate::models::establish_connection;
use dialoguer::{theme::ColorfulTheme, Input};
use qrcodes::render_qr;
fn main() {
    loop {
        let connection = &mut establish_connection();

        // let mut indeficator = String::new();
        // let mut link = String::new();

        println!("What would you like your title to be?");
        // stdin("You indeficator: ").read_line(&mut indeficator);
        let indeficator: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Your indeficator:")
            .interact_text()
            .expect("Reason");

        let link: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Your link: ")
            .interact_text()
            .expect("Error");

        render_qr(&link);
        create_post(connection, &indeficator, &link);



        // stdin("You link: ").read_line(&mut link);
        println!(
            "\nOk! You are wrote ind: {} and link: {} (Press {} when finished)\n",
            indeficator, link, EOF
        );
        break

    }

}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
