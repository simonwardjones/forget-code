# Rust Super Summary

## Chapter 1

### Installation
Download Rust through `rustup`, a command line tool for managing Rust versions and associated tools.
```shell
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Check if rust is installed:
```shell
rustc --version
```

Update rust:
```shell
rustup update
```

To uninstall Rust and rustup:
```shell
 rustup self uninstall
```

### Development

See vscode docs here: https://code.visualstudio.com/docs/languages/rust

Summary:
    - rust-analyser extension for intellisense/highlighting
    - code-lldb for debugger

### Compiling and running

Here is a basic hello world rust file. Note it is called `main.rs` and has a `main()` function which is the default entry point to all rust programmes. `println!` is not a function, it is a macro - marked with the `!`. This gets expanded to more code at compile time (in this instance opening and writing to std out).

```rust
fn main() {
    let example_string: &str = "Issie";
    let fib_value: i32 = fib(10);
    println!("Hello, {example_string} {fib_value}");
}

fn fib(n: i32) -> i32 {
    if n == 0 || n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

```
Compile the file using `rustc main.rs`. This creates a binary called `main`. Run this with `./main`

### Compiling and running with Cargo

Cargo is Rust’s build system and package manager. Check this is installed with `cargo --version`. Create a new cargo project with `cargo new hello_cargo`. This is similar to `poetry` in python. This creates the folder hello_cargo, the source folder with `main.rs` and the `Cargo.toml`. The Cargo toml contains project configuration and dependencies.

To build the src code to `target/debug` folder use `cargo build`. To build with release optimisations use `cargo build --release ` to build into `target/release` (note this is a little slower).

Use `cargo run` to build and execute the code!


### Variables, constants

Create variables with `let`. Note they are immutable by default! They can't be made in the global scope.

```rust
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

Constants are **not** allowed to be mutable and are defined with `const` and **must** include a type annotation. Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

Variables may be shadowed (re-defined using a different value) but constants may not (within the same scope).

```rs
const PI: f64 = 3.1415926535898;
```

### Types

```rs
// scalar types: integers, floating-point numbers, Booleans and characters
let integer: i32 = 32;
let float: f64 = 3.1415;
let boolean: bool = true;
let character: char = 'a';
// compound types: tuple and array
let tuple: (i32, f64, bool, char) = (integer, float, boolean, character);
let array: [i32; 5] = [1, 2, 3, 4, 5]; //  arrays have a fixed length
// arrays only contain one type

let b = tuple.1; // access by dot index
let (a, _b, c, d) = tuple; // demonstrate destructuring
let three: i32 = array[2]; // access by index and []
```

### Functions

Are defined as follows:
```rs
fn add(x: i32, y: i32) -> i32 {
    x + y // implicitly return the last expression
}
let (x, y) = (1, 2);
let z = add(x, y);
```

### Control flow

Here we show an if block and because if is an expression, we can use it on the right side of a let statement.

```rs
if number > 5 {
    println!("Greater than 5");
} else if number > 2 {
    println!("Greater than 2, less than or equal to 5");
} else {
    println!("Less than or equal to 2");
}
let condition: bool = false;
let number = if condition { 5 } else { 6 };
```

### Looping

We can loop with the `loop` code block, the `while` block or the `floor` block.

```rs
let result = loop {
    counter += 1;
    println!("Again!");
    if counter == repeats {
        println!("I'm out'a here");
        break counter;
    }
};
```

Or the while block
```rs
while counter < N {
    x += counter;
    counter += 1;
}
```

Or everyone's favourite for block:
```rs
let a = [10, 20, 30, 40, 50];
for element in a {
    println!("The value is: {element}");
}
for number in (1..4).rev() {
    println!("{number}!");
}
```

### Ownership

Memory is managed through a system of ownership with a set of rules that the compiler checks.

**Stack/Heap**
- The stack stores values as last in, first out. Data stored on the stack must have a known fixed size
- On the heap you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap. Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data. Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there.

**Ownership Rules**
1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

```rs
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward
    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

The `String` type is stored on the heap to support a mutable, growable piece of text. This is different from string-literals which are of known size as they are hard-coded directly into the final executable.

Memory is automatically returned once the variable that owns it goes out of scope. Rust calls a special function called `drop` for us.

**Move**
When re-assigning a value on the **heap** rust considers the previous pointer as no longer valid.
```rs
let s1 = String::from("hello");
let s2 = s1;
// s1 has been moved to s2 and s1 is no longer value
```
If we want a full clone of the heap data we can use clone `let s2 = s1.clone();`

If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied. Examples include integers, floating-points, boolean, char and Tuples, if they only contain types that also implement Copy.

**Functions**
Passing a variable to a function will move or copy, just as assignment does.


### References and borrowing

Instead of moving a variable (e.g `String`) into a function we can provide a reference. The `&var` syntax tells us that it is a reference and doesn't own the data. We call the action of creating a reference `borrowing`. Just as variables are immutable by default, so are references.

```rs
fn calculate_length(s: &String) -> usize {
    s.len()
}
let simon = String::from("Simon");
let len_simon = calculate_length(&simon);
```

**Mutable references**
We can have mutable references but if you have a mutable reference to a value, you can have no other references to that value in scope!

```rs
let mutable_simon = String::from("Simon");
let mutable_reference_simon = &mut mutable_simon;
```

Reference’s scope starts from where it is introduced and continues through the last time that reference is used.


### Slices

Slices let you reference a contiguous sequence of elements in a collection. It's a kind of reference, so it does not have ownership.
```rs
let simon: String = String::from("Simon Darcy-Jones");
let simon_first_name: &str = &simon[0..5];
let simon_last_name: &str = &simon[6..];
```

### Using Crates

Add the crate to the dependencies:
```toml
[dependencies]
rand = "0.8.5"
```
To use the crate we bring it into scope using `use rand::Rng;` (like `import` in python)
```rs
use rand::Rng;

let secret_number = rand::thread_rng().gen_range(1..101);
```

