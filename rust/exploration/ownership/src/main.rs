// Various examples from the ownership chapter.
use std::io;

fn main() {
    ownership_example();
    reference_example();
    reference_scope_example();
    slice_example();
    // read_first_word(); // Requires user input
}

#[allow(dead_code)]
fn ownership_example() {
    println!("String example");
    // we use a String as these are stored on the heap (and a reference to it on the stack)
    let mut example: String = String::from("Hello");
    example.push_str(" world!"); // push_str() appends a literal to a String
    println!("example = {example}");
    let moved_example: String = example; // value moved to moved_example
                                         // println!("example = {example}"); // this would raise a compile error
    let cloned_example: String = moved_example.clone(); // value cloned to cloned_example
    println!("moved_example = {moved_example} and cloned_example = {cloned_example}");
    let x = 1;
    let y = x;
    println!("x ({x}) and y ({y}) are both valid as stack values are copied.");
    println!();
}

#[allow(dead_code)]
fn reference_example() {
    fn exclaim(to_exclaim: &String) {
        println!("{to_exclaim}!!!!");
    }
    fn add_exclaim(to_exclaim: &mut String) -> String {
        to_exclaim.push_str("!!");
        return to_exclaim.to_string();
    }

    println!("Reference example");
    let mut issie: String = String::from("Issie");
    exclaim(&issie); // we can create an immutable reference.
    let exclaimed_issie: String = add_exclaim(&mut issie); // we can create one mutable reference.
    println!("exclaimed_issie = {exclaimed_issie}");
    println!();
}

#[allow(dead_code)]
fn reference_scope_example() {
    // In this example the order of the last
    // two lines is important to whether it will compile
    // This is because a reference exists until it is last used!
    // and we can only have 1 mutable reference!
    println!("Reference scope example");
    let mut simon: String = String::from("Simon");
    let simon_copy: &mut String = &mut simon;
    println!("simon_mut_copy = {simon_copy}");
    println!("simon = { }", simon);
    println!();
}

#[allow(dead_code)]
fn slice_example() {
    println!("Slice example");
    let simon: String = String::from("Simon Darcy-Jones");
    let simon_first_name: &str = &simon[0..5];
    let simon_last_name: &str = &simon[6..];
    println!("simon_first_name = {simon_first_name}");
    println!("simon_last_name = {simon_last_name}");
    println!();
}

#[allow(dead_code)]
fn read_first_word() {
    // Demo from rust book

    fn first_word(example: &str) -> &str {
        println!("{}", example);
        match example.find(" ") {
            // find returns an Option<usize>
            Some(index) => &example[..index],
            None => example,
        }
    }

    println!("Guessing game example");
    let mut guess = String::from("");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let result: &str = first_word(&guess);
    println!("First word = {result}!");

    println!();
}
