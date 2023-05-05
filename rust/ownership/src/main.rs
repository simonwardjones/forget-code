// Various examples from the ownership chapter.
use std::io;

fn main() {
    let mut guess = String::from("");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let result: String = exercise_from_book_simon(&guess);
    let _first = first_word(&guess);
    println!("result = {}!", result);

    check_string_literal();
}

#[allow(dead_code)]
fn example_1() {
    let mut issie: String = String::from("Issie");

    println!("Hello from {issie}");
    exclaim(&issie);
    // I expect this to fail borrow checker.
    println!("Hello from {issie} after using ref to issie in sub function.");
    let exclaimed_issie: String = add_exclaim(&mut issie);
    println!("Hello from {issie} after using ref to issie in second sub function.");
    println!("exclaimed_issie = {exclaimed_issie}");
}

fn exclaim(to_exclaim: &String) {
    println!("{to_exclaim}!!!!");
}

fn add_exclaim(to_exclaim: &mut String) -> String {
    to_exclaim.push_str("!!");
    return to_exclaim.to_string();
}

#[allow(dead_code)]
fn example_2() {
    // In this example the order of the last
    // two lines is important to whether it will compile
    // This is because a reference exists until it is last used!
    // and we can only have 1 mutable reference!
    let mut simon: String = String::from("Simon");
    let simon_copy: &mut String = &mut simon;
    simon_copy.push_str("!!!");
    println!("simon_copy = {simon_copy}");
    println!("simon = { }", simon);
}

fn exercise_from_book_simon(example: &String) -> String {
    // Here’s a small programming problem: write a function that takes a string
    // of words separated by spaces and returns the first word it finds in that
    // string. If the function doesn’t find a space in the string, the whole string
    // must be one word, so the entire string should be returned.
    // let example = String::from("here is an example.");

    println!("{}", example);
    match example.find(" ") {
        Some(t) => example[..t].to_string(),
        None => example.to_string(),
    }
}

fn first_word(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn check_string_literal() -> String {
    "hello!".to_string()
}
