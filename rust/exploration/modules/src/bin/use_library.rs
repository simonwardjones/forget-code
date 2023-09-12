use std::cmp::max;

use modules::gym::cardio::swim as also_swim;
#[allow(unused_imports)]
use modules::one_rebel::{Shake, ShakeFlavour};
use modules::{main as library_main, swim};

fn main() {
    // Run this with `cargo run --bin use_library`
    println!("Modules examples");

    // Run the library_main function which aliases from the library crate.
    library_main();

    // here we use a re-exported function swim.
    swim();
    also_swim();

    let mut my_shake: Shake = Shake::chocolate_shake();
    while my_shake.volume > 0 {
        println!("glug!");
        my_shake.volume = max(0, my_shake.volume - 100);
    }
    // The following would fail because price is private field on the struct
    // let cheap_shake = Shake {
    //     flavour: ShakeFlavour::Strawberry,
    //     volume: 500,
    //     price: 350,
    // };
    println!();
}
