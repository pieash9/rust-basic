fn main() {
    // ownership.. only one owner at a time
    let mut s1 = String::from("Pieash");
    let s2 = &mut s1;
    s2.push_str("Ahmed");
    println!("{:p}", &s2);
    println!("{}", s1);
}

