fn main() {
    // ownership.. only one owner at a time
    let mut s1 = String::from("Pieash");
    let s2 = s1;
    s1 = s2;
    println!("{}", s1);
}

