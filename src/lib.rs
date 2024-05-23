pub mod schema;
pub mod models;
pub mod qrcodes;

// Import generate function
use random_string::generate;

pub fn rand_indificator(lenght: usize) -> String {
    // Your custom charset
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVXYZ";
    let result = generate(lenght, charset);
    return result;

}





