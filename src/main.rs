
trait Summary {
    fn summaries(&self) -> String {
        return  String::from("Hi There!!!");
    }
}

struct User {
    name : String,
    age: u32,
}

struct Fix ;
impl  Summary for Fix {}
impl Summary for User {}
impl  Summary for String {}

fn main() {
    let user = User{
        name:String::from("Pieash"),
        age: 27
    };
    let f = Fix;
    notify(f);
}

fn notify(u : impl Summary) {
    println!("{}", u.summaries())
}