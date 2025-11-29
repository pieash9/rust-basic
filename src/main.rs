
fn main() {
    let ans: bool = is_even(1000000000001);
    println!( "{}",ans);
}

fn is_even(num: i64) -> bool {
    if num % 2 == 0 {
        return  true;
    }else {
        return  false;
    }
}