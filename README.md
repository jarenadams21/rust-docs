# Rust Docs Repository
    For tracking my progression through Rust documentation

## Rust Project Resources & Examples
    * Rust Project Playlist
        https://www.youtube.com/playlist?list=PL5TgJsqBELcOamwXabwjbXgAg3cucBjWY
    * Awesome Rust
        https://github.com/rust-unofficial/awesome-rust
    * Rust Standard Library Docs
        https://doc.rust-lang.org/std/prelude/index.html
    * Semantic Versioning
        https://semver.org/
        ex: rand = "0.8.5) inside [dependencies]
    * Crates.io
        - Rust community crate registry (open-source)
        - Website: https://crates.io/
        - Publishing on crates.io: https://doc.rust-lang.org/cargo/reference/publishing.html
    * Cargo
        https://doc.rust-lang.org/cargo/


## Vocab/Reminders
### Chapter 2
   * Cargo.toml dependencies use semantic versioning for specifying version numbers

   * Cargo.lock allows for reproducible builds. Cargo figures out all the versions of the dependencies that fit the criteria and writes them to the Cargo.lock file. When rebuilding again, Cargo will see the .lock file and look towards there for figuring out versions again unless specified otherwise to change the version

   * cargo update retrieves all latest versions that fit specifications in Cargo.toml

   * 'cargo doc --open' views documentation for this cargo

   * Rust has a strong, static type system
        (but it also has type inference)

   * Shadowing lets us reuse the same variable name and pass in a new value. Often used when you want to convert a value from one type to another type
### Chapter 3

   * Data Types
     https://doc.rust-lang.org/book/ch03-02-data-types.html
     <br>
     * Integers
        - signed(i) and unsigned(u) denote pos and neg integers or just pos integers
    * Statements are instructions that perform some action and do not return a value.
        Semi-colons end statements
    * Expressions evaluate to a resultant value. Let’s look at some examples.
        let y = {
            let x = 3;
            x + 1
        }
        y in this case is 4

### Chapter 4  https://doc.rust-lang.org/beta/book/ch04-00-understanding-ownership.html

    * Ownership addresses stack and heap allocation
    
    * Ownership Rules:
        i. Each value in Rust has an owner.
        ii. There can only be one owner at a time.
        iii. When the owner goes out of scope, the value will be dropped.
    

    