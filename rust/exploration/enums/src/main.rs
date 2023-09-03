use std::f64::consts;

#[allow(dead_code)]
#[derive(Debug)]
enum Shape {
    Rectangle { height: i32, width: i32 },
    Circle(i32),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { height, width } => f64::from(height * width),
            Shape::Circle(radius) => Shape::circle_area(radius),
        }
    }
    fn circle_area(radius: &i32) -> f64 {
        return consts::PI * f64::from(radius * radius);
    }
}

fn main() {
    let rectangle = Shape::Rectangle {
        height: 10,
        width: 10,
    };
    let circle = Shape::Circle(10);
    println!(
        "Hello, world! {rectangle:#?}, {circle:#?} have areas {} and {}",
        rectangle.area(),
        circle.area()
    );
}
