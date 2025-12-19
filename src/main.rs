fn main() {
    let ans;

    let str1 = String::from("Hello");
    let str2 = String::from("World!");

    ans = longest(str1, str2);

    println!("The longest string is: {}", ans);

}

fn longest(a : String, b: String) -> String {
    if  a.len() > b.len() {
        return  a;
    } else {
         return  b;
    }
}