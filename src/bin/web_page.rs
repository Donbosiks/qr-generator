extern crate rocket;
use rocket::*;
use rocket::response::Redirect;

#[get("/<identifier>")]
fn test(identifier: &str) -> Redirect {

    Redirect::to(uri!(hello: name, age))
}

#[launch]
pub fn rocket() -> _ {
    println!("Rocket started");
    rocket::build().mount("/", routes![test])
}