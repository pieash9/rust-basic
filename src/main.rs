struct Rect {  // like js class
    width: u32, // like js class property
    height: u32, // like js class property
}

impl Rect {
    fn area(&self) -> u32 {  // this is like js instance method
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn debug() -> u32 {   // this is like js static method
        return  1;
    }
}

fn main() {
    let rect1 = Rect {width: 88, height: 19}; // like js new class()

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("The perimeter of the rectangle is {} pixels.", rect1.perimeter());
    println!("The debug of the rectangle is {} pixels.", Rect::debug());

}