use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();

    users.insert(String::from("Pieash"), 22);
    users.insert(String::from("Alice"), 30);

    let first_first_user_age = users.get("Alice");

    match first_first_user_age {
        Some(age) => println!("The age of the first user is: {}", age),
        None => println!("User not found"),
    }
}
