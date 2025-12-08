fn main() {
    let mut v1 = vec![1, 2, 3];
    let  v1_iter = v1.iter_mut();

    for val in v1_iter {
        *val += 1;
    }
 
    println!("{:?}", v1);
}