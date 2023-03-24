// In every executable Rust program,
// the main function is always the first code
// that runs

fn main() {
    // println! calls a Rust macro (because of the !)
    // macros != functions
    println!("Hello, world!");
}

// rustc main.rs
/* 
    i. Compiling main with the rust compiler 
    (similar to gcc or clang)
    ii. Rust outputs a binary executable

*/