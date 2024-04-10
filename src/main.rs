

fn main() {

}


// #[derive(Insertable)]
// #[diesel(table_name = "users")]
// struct User {
//     id: i8,
//     name: str,
//     email: str,
// }
//
// fn main() {
//     // Establish connection to the database (replace with your connection string)
//     let connection = Connection::establish(LINK)
//         .expect("Error connecting to database");
//
//     // Create a new user
//     let new_user = User {
//         id: &i8::from(1),
//         name: &str::from("John Doe"),
//         email: &str::from("john.doe@example.com"),
//     };
//
//     // Insert the user into the "users" table
//     insert_into(users::table)
//         .values(&new_user)
//         .execute(&connection)
//         .expect("Error inserting user");
//
//     println!("Successfully inserted user!");
// }
