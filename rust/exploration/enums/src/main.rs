use std::f64::consts;

fn main() {
    direction_example();
    shape_example();
    bed_example();
    option_example();
    exhaustive_match_example();
    if_let_example();
}

#[allow(dead_code)]
fn direction_example() {
    println!("Direction example");

    #[derive(Debug)]
    enum Direction {
        North,
        East,
        South,
        West,
    }
    let direction = Direction::North;
    println!("We are heading {:?}", direction);
    println!();
}

#[allow(dead_code)]
#[derive(Debug)]
enum Shape {
    // variants can include data similar to unit structs, tuple structs and structs
    Rectangle { height: i32, width: i32 },
    Circle(i32),
    UnknownPolygon,
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { height, width } => f64::from(height * width),
            Shape::Circle(radius) => Shape::circle_area(radius),
            Shape::UnknownPolygon => 0.0,
        }
    }
    fn circle_area(radius: &i32) -> f64 {
        return consts::PI * f64::from(radius * radius);
    }
}

fn shape_example() {
    println!("Shape example");
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
    println!();
}

#[allow(dead_code)]
#[derive(Debug)]
enum Bed {
    Single,
    Queen, // a.k.a. Small Double
    Double,
    King,
    SuperKing,
}

#[derive(Debug)]
struct Dimensions {
    width: i32,
    length: i32,
}

impl Dimensions {
    fn area(&self) -> i32 {
        self.width * self.length
    }
}

impl Bed {
    fn size(&self) -> Dimensions {
        match self {
            Bed::Single => Dimensions {
                width: 90,
                length: 190,
            },
            Bed::Queen => Dimensions {
                width: 120,
                length: 190,
            },
            Bed::Double => Dimensions {
                width: 135,
                length: 190,
            },
            Bed::King => Dimensions {
                width: 150,
                length: 200,
            },
            Bed::SuperKing => Dimensions {
                width: 180,
                length: 200,
            },
        }
    }

    fn area(&self) -> i32 {
        self.size().area()
    }
}

fn bed_example() {
    println!("Bed example");
    let bed = Bed::SuperKing;
    println!(
        "I would like a {:#?} which has dimensions {:#?} and area {:?}",
        bed,
        bed.size(),
        bed.area()
    );
    println!();
}

fn option_example() {
    println!("Option example");
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;
    let z: i32 = 5 + x.unwrap();
    // Equivalent to:
    let unwrapped_x: i32 = match x {
        Some(y) => y,
        None => panic!("called `Option::unwrap()` on a `None` value"),
    };
    let _z2: i32 = 5 + unwrapped_x;
    println!("x is {x:?}, y is {y:?}, z and z2 = {z:?}");
    println!();
}

fn exhaustive_match_example() {
    println!("Exhaustive match example");
    let x: i32 = 20;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("If you comment this line i fail."),
    }
    println!("x is {x:?}");
    println!();
}

fn if_let_example() {
    println!("If let example");
    let x: Option<i32> = Some(5);

    if let Some(y) = x {
        println!("y is {y:?}");
    }
    // Same as:
    match x {
        Some(y) => println!("y is {y:?}"),
        _ => (),
    }

    // Example with custom enum
    let circle = Shape::Circle(10);
    if let Shape::Circle(radius) = circle {
        println!("Radius is {radius:?}");
    } else {
        println!("Not a circle");
    }
    println!();
}
