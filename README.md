# Rust Docs Repository
    For tracking my progression through Rust documentation

## Frameworks
    * Tide
        https://www.youtube.com/watch?v=ZbhzLP3vnkg

## Official Docs
    * Appendix C
        https://doc.rust-lang.org/beta/book/appendix-03-derivable-traits.html
    

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

    * In order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:
        i. The memory must be requested from the memory allocator at runtime
        ii. We need a way of returning this memory to the allocator when we're done with our String
        <br>
        --> Part 1 is accomplished by 'String::from', its implementation requests the memory it needs
        --> For part 2, we must think about pairing exactly one allocate with exactly one free at all times
    * When a variable goes out of scope, Rust calls the 'drop' function to return the memory
        Similar to RAII (Resource Acquisition Is Initialization) patterns in C++
    * In this example: 
        let s1 = String::from("hello");
        let s2 = s1;
        
        --> s2 has overidden s1 as the new pointer to the data in the heap, and s1 is no longer a valid pointer. Trying to println s1 in this case would cause a compiler error. "s1 moved into s2"
    * Types cannot have both the 'Copy' trait and the 'Drop' trait
    * Any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy
    All the integer types, such as u32.
    * The Boolean type, bool, with values true and false. All the floating-point types, such as f64. The character type, char. Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
    * A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
    * References are immutable! You don't own them. References can be made mutable, but only one at a time. If an immutable reference exists, a mutable reference cannot exist.
    The Rules of References:
    Let’s recap what we’ve discussed about references: 
        i. At any given time, you can have either one mutable reference or any number of immutable references. 
        ii. References must always be valid.
    * Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.
    * String literals are immutable, and an &str parameter type will allow both Strings and string literals.

# Chapter 5
    * Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
         i. impl Rectangle {
             fn square(size: u32) -> Self {
                 Self {
                     width: size,
                     height: size,
                    }  
                }
            }
            
        ii. To call this associated function, we use the :: syntax with the struct name; let sq = Rectangle::square(3); is an example. This function is namespaced by the struct: the :: syntax is used for both associated functions and namespaces created by modules.


    * Automatic dereferencing and referencing
        i. Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (&self), mutating (&mut self), or consuming (self).
    
    * You use '.' if you’re calling a method on the object directly and '->' if you’re calling the method on a pointer to the object and need to dereference the pointer first. In other words, if object is a pointer, object->something() is similar to (*object).something()
    

        

    