extern crate rocket;
use rocket::*;
use rocket::response::Redirect;
use crate::models::find_link;

#[get("/<identifier>")]
fn test(identifier: &str) -> Redirect {

    let res_link = String::from(find_link(identifier));
    Redirect::to(res_link)
}

#[launch]
pub fn rocket() -> _ {
    println!("Rocket started");
    rocket::build().mount("/", routes![test])
}