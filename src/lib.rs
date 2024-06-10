pub mod schema;
pub mod models;
pub mod qrcodes;
extern crate rocket;

// Import generate function
use random_string::generate;

pub fn rand_identifier(lenght: usize) -> String {
    // Your custom charset
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVXYZ";
    let result = generate(lenght, charset);
    return result;

}





