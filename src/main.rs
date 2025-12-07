use std::collections::HashMap;

fn group_values_by_keys(vec : Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert(key, value);
    }
    return  hm;
}


fn main() {
    let input_vec = vec![(String::from("pieash"), 22), (String::from("apple"), 10), (String::from("banana"), 15)];
    let hm = group_values_by_keys(input_vec);
    println!("{:?}", hm);
}