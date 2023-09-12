pub mod one_rebel;
mod tennis_club;

// here we bring the sprint function into scope.
use one_rebel::reshape::sprint;
// re-exporting the swim function from the gym::cardio module.
pub use gym::cardio::swim;

pub fn main() {
    // Run with `cargo run -p modules --bin modules`
    println!("In this example, we have a lib.rs library crate and a binary crate using this called use_library.rs.");
    println!("We also have a two.rs binary crate demonstrating multiple binary crates.");

    // gym and public sub module cardio are defined inline in this file.
    gym::cardio::run();

    // tennis_club is defined in a separate file src/tennis_club.rs.
    // tennis_club::_smash(); // This would fail as _smash is private.
    // Note Code within a module is private from its parent modules by default.
    // the crate:: demonstrates an absolute path.
    crate::tennis_club::smash();

    // one_rebel is defined in a separate file src/one_rebel/mod.rs.
    // reshape is a public submodule of one_rebel in src/one_rebel/reshape.rs
    one_rebel::reshape::sprint();

    sprint(); // this is the same as the previous line but uses the use statement.
}

// Here is an example of an inline module.
pub mod gym {
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
