enum  Shape {
    Rectangle(f64, f64),
    Circle(f64)
}

fn main() {
   let react = Shape::Rectangle(1.0, 2.0);
   calculate_area(react);

   let circle = Shape::Circle( 2.0);
   calculate_area(circle);
}

fn calculate_area(shape: Shape) -> f64 {
   let area =  match shape {
        Shape::Rectangle(a,b ) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    };

    return area;
}