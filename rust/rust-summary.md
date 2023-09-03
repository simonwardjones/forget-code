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


### Variables

Create variables with `let`. Note they are immutable by default!

```rust
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

### Using creates

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

