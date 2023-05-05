fn main() {
    // example_1()
    // nestedloops()
    // manual_while_loop()
    // real_while();
    // for_examples();
    match_example();
}

#[allow(dead_code)]
fn example_1() {
    let condition = false;
    let number = if condition { 5 } else { 6 };
    let repeats: i8 = 3;
    println!("Hello, world! {number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("again!");
        if counter == repeats {
            println!("I'm outa here");
            break;
        }
    };
    // now we show that by default the
    // loop expression returns () the empty tuple.

    if result == () {
        println!("result is ()")
    } else {
    }
    counter = 0;
    let result = 'my_counter: loop {
        counter += 1;
        println!("again!");
        if counter == repeats {
            println!("I'm out'a here");
            // here we break from this loop (they could be nested)
            break 'my_counter counter;
        }
    };
    println!("Now the result is {result} as we return the counter from the loop block");
}

#[allow(dead_code)]
fn nestedloops() {
    let mut simon = 0;
    'outer: loop {
        simon += 1;
        println!("simon = {simon}");
        let mut inner = 0;
        'inner: loop {
            inner += 1;
            println!("inner = {inner}");
            if inner == simon {
                break 'inner;
            }
            if inner == 4 {
                break 'outer;
            }
        }
    }
}

#[allow(dead_code)]
fn manual_while_loop() {
    // while x < 10 print x
    let mut x = 0;
    'fake_while: loop {
        if x == 10 {
            break 'fake_while;
        } else {
            println!("{x}");
            x += 1;
        }
    }
}

#[allow(dead_code)]
fn real_while() {
    let mut x = 0;
    let mut counter = 1;
    const N: i32 = 10;
    while counter < N {
        x += counter;
        counter += 1;
    }

    println!("{x} ")
}

#[allow(dead_code)]
fn for_examples() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value in a manual index while is: {}", a[index]);

        index += 1;
    }
    for element in a {
        println!("the value in a for loop is: {element}");
    }
    let _number = 21;
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
use std::cmp::Ordering;

fn match_example() {
    let x = 10;
    match x.cmp(&11) {
        Ordering::Less => println!("LESS"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
