struct User{
    first_name: String,
    last_name: String,
    age: i32,
}


fn main() {
    let user =  User {
        first_name: String::from("Pieash"),
        last_name: String::from("Khan"),
        age: 23,
    };

    println!("User: {} {}, Age: {}", user.first_name, user.last_name, user.age);

}