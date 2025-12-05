use chrono::{Local, Utc};

fn main() {
    let now  = Utc::now();
    let local_time = Local::now();
    println!("Current Local time: {}", local_time);
    println!("Current UTC time: {}", now);
}