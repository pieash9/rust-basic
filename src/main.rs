use chrono::Utc;

fn main() {
    let now  = Utc::now();
    println!("Current UTC time: {}", now);
}