fn main() {
    println!("Running basics.\n");
    type_example();
    constant_example();
    tuple_example();
    array_example();
    function_example();
    if_example();
    loop_example();
    nested_loops_example();
    manual_while_loop_example();
    while_example();
    for_example();
}

#[allow(dead_code)]
fn type_example() {
    println!("type example");
    // scalars
    let integer: i32 = 32;
    let float: f64 = 3.1415;
    let boolean: bool = true;
    let character: char = 'a';
    // compound
    let tuple: (i32, f64, bool, char) = (integer, float, boolean, character);
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    // print them
    println!("integer: {integer}");
    println!("float: {float}");
    println!("boolean: {boolean}");
    println!("character: {character}");
    println!("tuple: {tuple:?}");
    println!("array: {array:?}");
    println!("");
}

#[allow(dead_code)]
fn constant_example() {
    println!("constant example");
    const PI: f64 = 3.14159265358979323846264338327;
    println!("Pi is roughly {:.10}", PI);
    println!("");
}

#[allow(dead_code)]
fn tuple_example() {
    println!("tuple example");
    let tuple: (i32, f64, bool, char) = (32, 3.1415, true, 'a');
    let b = tuple.1; // access by dot index
    let (a, _b, c, d) = tuple; // demonstrate destructuring
    println!("a: {a}, b: {b}, c: {c}, d: {d}");
}

#[allow(dead_code)]
fn array_example() {
    println!("array example");
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let three: i32 = array[2]; // access by index and []
    let ones: [i32; 5] = [1; 5];
    println!("array: {array:?}");
    println!("ones: {ones:?}");
    println!("three: {three}");
    println!("");
}

#[allow(dead_code)]
fn function_example() {
    println!("function example");
    fn add(x: i32, y: i32) -> i32 {
        x + y // implicitly return the last expression
    }
    let (x, y) = (1, 2);
    let z = add(x, y);
    println!("add(x, y) = {z}");
    println!("");
}

#[allow(dead_code)]
fn if_example() {
    println!("if example");
    let number = 3;
    if number > 5 {
        println!("Greater than 5");
    } else if number > 2 {
        println!("Greater than 2, less than or equal to 5");
    } else {
        println!("Less than or equal to 2");
    }
    let condition: bool = false;
    // if is an expression so we can do this:
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
    println!("");
}

#[allow(dead_code)]
fn loop_example() {
    println!("loop example");
    let repeats: i8 = 3;
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Again!");
        if counter == repeats {
            println!("I'm outa here");
            break;
        }
    }
    // loops are expressions to returning () by default
    // we can add a value after break to return it
    counter = 0;
    let result = loop {
        counter += 1;
        println!("Again!");
        if counter == repeats {
            println!("I'm out'a here");
            break counter;
        }
    };
    println!("Now the result is {result} as we return the counter from the loop block");
    println!("");
}

#[allow(dead_code)]
fn nested_loops_example() {
    println!("nested loops example");
    // We can break a specific loop by labeling it.
    let mut simon = 0;
    'outer: loop {
        simon += 1;
        println!("simon = {simon}");
        let mut inner = 0;
        'inner: loop {
            inner += 1;
            println!("inner = {inner}");
            if inner == simon {
                println!("-----\n");
                break 'inner;
            }
            if inner == 2 {
                println!("----------\n");
                break 'outer;
            }
        }
    }
    println!("");
}

#[allow(dead_code)]
fn manual_while_loop_example() {
    println!("manual while loop example");
    let mut x = 0;
    loop {
        if x == 5 {
            break;
        } else {
            println!("{x}");
            x += 1;
        }
    }
    println!("");
}

#[allow(dead_code)]
fn while_example() {
    println!("while example");
    let mut x: i32 = 0;
    let mut counter: i32 = 1;
    const N: i32 = 4;
    while counter < N {
        x += counter;
        counter += 1;
    }
    println!("{x} ");
    println!("");
}

#[allow(dead_code)]
fn for_example() {
    println!("for example");
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {element}");
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    println!("");
}
