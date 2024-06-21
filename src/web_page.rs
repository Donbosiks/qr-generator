extern crate rocket;
use rocket::*;
use rocket::response::Redirect;
use crate::models::find_link;

#[get("/<ident>")]
fn test(ident: &str) -> Redirect {

    let ans = find_link(ident);

    match ans {
        Some(_) => Redirect::to(ans.unwrap().to_string()),
        None => Redirect::to("https://pidor.com"),
    }

    // let res_link = String::from(ans);
    // Redirect::to(res_link)

}

#[launch]
pub fn rocket() -> _ {
    println!("Rocket started");
    rocket::build().mount("/", routes![test])
}