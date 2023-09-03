// In this file I create a struct similar to defining a class in python
// I use the derive to implement the string formatting to print
// Then I implement two methods (the second is a factor method.)
//
// Finally we have the main which creates an instance
// and calls some methods. I show how to make a mutable instance too.

#[derive(Debug)]
struct Human {
    name: String,
    age: usize,
    weight: i32,
    email: String,
}

impl Human {
    fn describe(&self) {
        println!(
            "This is {}, aged {} weighing {} with email {}.",
            self.name, self.age, self.weight, self.email
        );
    }

    fn baby(name: String, weight: i32) -> Human {
        Human {
            email: String::from(&name) + "@gmail.com",
            name,
            weight,
            age: 0,
        }
    }
}

fn main() {
    let simon = Human {
        name: String::from("simon"),
        age: 31,
        weight: 78,
        email: String::from("simonwardjones16@gmail.com"),
    };
    println!("This is {:?} ", simon);
    println!("With email {} ", simon.email);
    simon.describe();
    let mut baby = Human::baby("tom".to_string(), 2);
    baby.describe();

    baby.name += "!";
    baby.describe();
}
