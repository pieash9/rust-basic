fn main() {
    let mut v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter_mut();

    let first_number = v1_iter.next();
    let second_number = v1_iter.next();
    let third_number = v1_iter.next();

    // while let Some(val) = v1_iter.next() {
    //     print!("{}", val);
    // }

    println!("{:?}", v1);
}