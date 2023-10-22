#[allow(unused_imports)]
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::mem::take;

fn main() {
    // example_panic(); // This will panic if uncommented
    example_file_writing();
    car_example();
}

#[allow(dead_code)]
fn example_panic() {
    print!("Example Panic");
    panic!("This is the panic message")
}

#[allow(dead_code)]
fn example_file_writing() -> () {
    let mut file: File = open_or_create("simon.txt");
    file.write_all("Well here we go !".to_string().as_bytes())
        .expect("failed writing to file");
}

#[allow(dead_code)]
fn open_or_create(filename: &str) -> File {
    OpenOptions::new()
        .read(false)
        .write(true)
        .append(true)
        .create(false)
        .open(filename)
        .unwrap_or_else(|error| {
            println!("{error}\nCreating file {filename}");
            File::create(filename).expect("Failed to create")
        })
}

#[derive(Debug)]
#[allow(dead_code)]
enum CarColour {
    Red,
    Green,
    Blue,
    Black,
    Silver,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Car {
    number_plate: String,
    age: i32,
    colour: CarColour,
}

#[derive(Debug)]
pub struct CarPark {
    pub cars: Vec<Car>,
}

impl CarPark {
    pub fn filter_old(&mut self, filter_age: i32) {
        // I am in some method on a CarPark
        // and I want to remove the old cars from the cars field

        // How should I do this!?

        // 0.
        // self.cars.retain(|car| car.age < filter_age)

        // The following approach is not possible because
        // we would have a "missing" self.cars on CarLot if the rhs fails
        // --
        // self.cars = self.cars
        //     .into_iter()
        //     .filter(|car| car.age < filter_age)
        //     .collect();

        // 1.
        self.cars = take(&mut self.cars)
            .into_iter()
            .filter(|car| car.age < filter_age)
            .collect();

        // 2.
        // self.cars = self
        //     .cars
        //     .drain(..)
        //     .filter(|car| car.age < filter_age)
        //     .collect();

        // 3.
        // Change the input to mut self instead of &mut self
        // but then I need to return the self (to carry on using)
        // self.cars = self
        //     .cars
        //     .into_iter()
        //     .filter(|car| car.age < filter_age)
        //     .collect();
        // self
    }

    pub fn print_car_count(&self) {
        println!("Car count: {}", self.cars.len());
    }
}

fn car_example() {
    let new_audi = Car {
        number_plate: "RG54 1PQ".to_string(),
        age: 0,
        colour: CarColour::Black,
    };
    let new_ford = Car {
        number_plate: "RG54 2PQ".to_string(),
        age: 0,
        colour: CarColour::Green,
    };
    let audi = Car {
        number_plate: "RG54 3PQ".to_string(),
        age: 0,
        colour: CarColour::Blue,
    };
    let ford = Car {
        number_plate: "RG54 4PQ".to_string(),
        age: 10,
        colour: CarColour::Silver,
    };
    println!("audi = {audi:?}");
    let mut lot = CarPark {
        cars: vec![new_audi, audi, new_ford, ford],
    };
    lot.print_car_count();
    lot.filter_old(5);
    lot.print_car_count();
}
