// In this file I create a struct similar to defining a class in python
// I use the derive to implement the string formatting to print
// Then I implement two methods (the second is a factory method).
//
// Finally we have the main which creates an instance
// and calls some methods. I show how to make a mutable instance too.

#[derive(Debug)]
struct Human {
    name: String,
    age: usize,
    email: String,
}

#[allow(dead_code)]
impl Human {
    fn describe(&self) {
        println!(
            "{}, aged {} with email {}.",
            self.name, self.age, self.email
        );
    }

    fn baby(name: String) -> Human {
        Human {
            email: String::from(&name) + "@gmail.com",
            name,
            age: 0,
        }
    }

    fn uses_gmail(&self) -> bool {
        self.email.ends_with("gmail.com")
    }

    fn uses_yahoo(&self) -> bool {
        self.email.ends_with("yahoo.com")
    }

    fn set_name(&mut self, new_name: String) {
        // This is a setter overwriting the name field
        self.name = new_name;
    }

    fn has_same_name(&self, other: &Human) -> bool {
        self.name == other.name
    }
}

fn instantiating_structs_example() {
    let simon = Human {
        name: String::from("Simon Darcy-Jones"),
        age: 31,
        email: String::from("simonwardjones16@gmail.com"),
    };
    println!("{:?}", simon);
    println!("Email field {} ", simon.email);
    dbg!(&simon); // example debug call.
    simon.describe();
    let mut baby = Human::baby("no-name".to_string());
    baby.set_name(String::from("Jack"));
    baby.name = String::from("Tom"); // same as using setter
    baby.describe();
    baby.name += "as";
    baby.describe();
    if simon.has_same_name(&baby) {
        println!("Simon and baby have the same name");
    } else {
        println!("Simon and baby have different names");
    }
}

fn field_init_shorthand_example() {
    let name = String::from("Simon Darcy-Jones");
    let age = 31;
    let email = String::from("simonwardjones16@gmail.com");
    let simon = Human { name, age, email };
    // println!("{:?}", name); This would error as name is moved into struct
    println!("{:?}", simon);
}

fn struct_update_syntax_example() {
    let simon = Human {
        name: String::from("Simon Darcy-Jones"),
        age: 31,
        email: String::from("simonwardjones16@gmail.com"),
    };
    let aged_simon = Human { age: 32, ..simon };
    println!("{:?}", aged_simon);
}

fn tuple_structs_example() {
    #[derive(Debug)]
    struct Coordinate(i32, i32);

    #[derive(Debug)]
    struct Rectangle(i32, i32);

    // same value - differing types
    let square = Rectangle(10, 10);
    let ten_ten = Coordinate(10, 10);
    println!("square = {square:?}, ten_ten = {ten_ten:?}");
}

fn unit_struct_example() {
    #[derive(PartialEq)]
    struct Sentinel;
    let _x = Sentinel;
    if _x == Sentinel {
        println!("Sentinel is a unit struct");
    }
}

fn main() {
    println!("Running structs examples");
    instantiating_structs_example();
    field_init_shorthand_example();
    struct_update_syntax_example();
    tuple_structs_example();
    unit_struct_example();
    println!();
}
