fn main() {
    let index = find_first_a(String::from("PIeash"));

    match index {
        Some(value) => println!("First 'a' found at index: {}", value),
        None => println!("No 'a' found"),
    }
}

fn find_first_a(s : String) -> Option<i32> {  // Option type to handle absence of 'a'
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return  Some(index as i32); // Return index wrapped in Some
        }
    }

    return  None; // Return None if 'a' is not found
}