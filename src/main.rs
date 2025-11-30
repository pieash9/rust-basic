
fn main() {
    let name = String::from("Pieash");
    let length = get_str_length(name);
    println!("Length of the string is: {}", length);
}

fn get_str_length(str: String) -> usize {
   return str.chars().count();
}