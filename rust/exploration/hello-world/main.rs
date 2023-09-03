// To run this first we build it
// $ rustc main.rs

// Then we run it
// $ ./main

fn main() {
    let example_string = "Issie";
    let fib_value = fib(10);
    println!("Hello, {example_string} {fib_value}");
}

fn fib(n: i32) -> i32 {
    // need control flow!
    if n == 0 || n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
