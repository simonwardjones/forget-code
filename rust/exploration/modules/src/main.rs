pub mod one_rebel;
mod tennis_club;

fn main() {
    // Run with `cargo run -p modules --bin modules`
    println!("Modules examples");
    print!("In this example, we have a main.rs binary crate ");
    println!("and one.rs and two.rs binary crates in the bin directory.");

    // gym and public sub module cardio are defined inline in this file.
    gym::cardio::run();

    // tennis_club is defined in a separate file src/tennis_club.rs.
    // tennis_club::_smash(); // This would fail as _smash is private.
    // Note Code within a module is private from its parent modules by default.
    tennis_club::smash();

    // one_rebel is defined in a separate file src/one_rebel/mod.rs.
    // reshape is a public submodule of one_rebel in src/one_rebel/reshape.rs
    one_rebel::reshape::sprint();

    // here we bring the sprint function into scope.
    use one_rebel::reshape::sprint;
    sprint(); // this is the same as the previous line but uses the use statement.
    println!();
}

// Here is an example of an inline module.
#[allow(dead_code)]
mod gym {
    // Here is an example of a nested module.
    pub mod cardio {
        pub fn run() {
            println!("gym::cardio::run() -> Running on the treadmill.");
        }

        pub fn walk() {
            println!("gym::cardio::walk() -> Walking on the treadmill.");
        }

        pub fn swim() {
            println!("gym::cardio::swim() -> Swimming in the pool.");
        }
    }
}
