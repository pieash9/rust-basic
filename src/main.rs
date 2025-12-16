
trait Summary {
    fn summaries(&self) -> String;
}

struct User {
    name : String,
    age: u32,
}

impl Summary for User {
    fn summaries(&self) -> String {
        return  format!("The name is {}, and the age is {}", self.name, self.age );
    }
}

fn main() {
    let user = User{
        name:String::from("Pieash"),
        age: 27
    };
    println!("{}", user.summaries());
}