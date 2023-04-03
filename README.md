# Rust Docs Repository
    For tracking my progression through Rust documentation

## Projects
[spell-walker](https://github.com/jarenadams21/spell-walker)

## Frameworks
    * Tide
        https://www.youtube.com/watch?v=ZbhzLP3vnkg

## Official Docs
    * Appendix C
        https://doc.rust-lang.org/beta/book/appendix-03-derivable-traits.html
    * Rust API Guidelines
        https://rust-lang.github.io/api-guidelines/
    

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
### Chapter 2 - Cargo
   * Cargo.toml dependencies use semantic versioning for specifying version numbers

   * Cargo.lock allows for reproducible builds. Cargo figures out all the versions of the dependencies that fit the criteria and writes them to the Cargo.lock file. When rebuilding again, Cargo will see the .lock file and look towards there for figuring out versions again unless specified otherwise to change the version

   * cargo update retrieves all latest versions that fit specifications in Cargo.toml

   * 'cargo doc --open' views documentation for this cargo

   * Rust has a strong, static type system
        (but it also has type inference)

   * Shadowing lets us reuse the same variable name and pass in a new value. Often used when you want to convert a value from one type to another type
### Chapter 3 - Data Types

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

### Chapter 4 - Ownership,  https://doc.rust-lang.org/beta/book/ch04-00-understanding-ownership.html

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

# Chapter 5 - Structs
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

# Chapter 6 - Enums & Pattern Matching, https://doc.rust-lang.org/beta/book/ch06-00-enums.html

* The Option type encodes the very common scenario in which a value could be something or it could be nothing.
* In his 2009 presentation “Null References: The Billion Dollar Mistake,” Tony Hoare, the inventor of null, has this to say:

        I call it my billion-dollar mistake. At that time, I was designing the first comprehensive type system for references in an object-oriented language. My goal was to ensure that all use of references should be absolutely safe, with checking performed automatically by the compiler. But I couldn’t resist the temptation to put in a null reference, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.
* The problem isn’t really with the concept but with the particular implementation. As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is Option of type T, and it is defined by the standard library, T being a generic type parameter

        enum Option<T> {
             None,
             Some(T),
        }  
* Option of type T != T

        Compiler error:
         let x: i8 = 5;
         let y: Option<i8> = Some(5);
         let sum = x + y;

* Option documentation: https://doc.rust-lang.org/beta/std/option/enum.Option.html

* The match expression is a control flow construct that does just this when used with enums: it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.

* Think of a match expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it, and each coin falls through the first hole it encounters that it fits into. In the same way, values go through each pattern in a match, and at the first pattern the value “fits,” the value falls into the associated code block to be used during execution.

* 'match' arm patterns MUST cover all possibilities

        The following fails as the 'None' case isn't covered: 

        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
            Some(i) => Some(i + 1),
            }
        }
* Catch-all Paterns must be used as the last option, and Rust will throw a compiler error if code is put after one

        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
        other => move_player(other),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn move_player(num_spaces: u8) {}

* Rust also has a pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern: '_' is a special pattern that matches any value and does not bind to that value.

        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => reroll(),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn reroll() {}

* Unit value: () is an empty tuple type, telling Rust we don't want to run any code in the catch all case

        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => (),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}

* You can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.

# Chapter 7 - https://doc.rust-lang.org/beta/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

* A crate is the smallest amount of code that the Rust compiler considers at a time. (crates == library)

* A crate can come in one of two forms: a binary crate or a library crate. Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server. Each must have a function called main that defines what happens when the executable runs. All the crates we’ve created so far have been binary crates.

* Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects. (e.g., rand crate)

* A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.

* How modules work - https://doc.rust-lang.org/beta/book/ch07-02-defining-modules-to-control-scope-and-privacy.html

* There is an implicit module named 'crate' at the beginning of module trees
        

# Chapter 8 - https://doc.rust-lang.org/beta/book/ch08-00-common-collections.html

* A [ vector ] allows you to store a variable number of values next to each other.

* A [ string ] is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.

* A [hash map] allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.

* Because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation:

        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];

        v.push(6);

        println!("The first element is: {first}");    

*  Using an enum plus a match expression means that Rust will ensure at compile time that every possible case is handled

* Like any other struct, a vector is freed when it goes out of scope. The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid. (the vector's contents are also dropped)

            {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
        } // <- v goes out of scope and is freed here

*  In Chapter 4, we talked about string slices, which are references to some UTF-8 encoded string data stored elsewhere. String literals, for example, are stored in the program’s binary and are therefore string slices.

* The [ String ] type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.

* Many of the same operations available with Vec are available with String as well, because String is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities.
    
        let mut s = String::new();

* A String can grow in size and its contents can change, just like the contents of a Vec (of type T), if you push more data into it. In addition, you can conveniently use the + operator or the format! macro to concatenate String values.

* push_str method takes a string slice because we don't necessarily want to take ownership of the parameter. If the push_str method took ownership of s2, we wouldn’t be able to print its value on the last line. However, this code works as we’d expect!

        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {s2}");

* In the standard library, you'll see add defined using generics and associated types. Here, we’ve substituted in concrete types, which is what happens when we call this method with String values. The [ + ] operator uses the [ add ] method, whose signature looks like:

        fn add(self, s: &str) -> String {

* When we call the add method, Rust uses a deref coercion, (see string_concat module in chapter-8)

* The format! macro works like println!, but instead of printing the output to the screen, it returns a String with the contents. (see string_concat module in chapter-8)

* See "Internal Representation" section in chapter 8.2, "Storing UTF-8 Encoded Text with Strings" for an explanation about string memory storage in rust

* If we were to try to slice only part of a character’s bytes with something like &hello[ 0..1 ], Rust would panic at runtime. Rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes:

        let hello = "Здравствуйте";

        let s = &hello[0..4];

* Be careful with string slices as they crash the program. Be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.

* Rust has chosen to make the correct handling of String data the default behavior for all Rust programs, which means programmers have to put more thought into handling UTF-8 data upfront. This trade-off exposes more of the complexity of strings than is apparent in other programming languages, but it prevents you from having to handle errors involving non-ASCII characters later in your development life cycle.

* Be sure to check out the documentation for useful methods like [ contains ] for searching in a string and [ replace ] for substituting parts of a string with another string.

* The last of our common collections is the hash map. The type HashMap>K, V< stores a mapping of keys of type K to values of type V using a hashing function, which determines how it places these keys and values into memory. 

* Just like vectors, hash maps store their data on the heap.

* Like vectors, hash maps are homogeneous: all of the keys must have the same type as each other, and all of the values must have the same type.

* The get method returns an Option<&V>; if there’s no value for that key in the hash map, get will return None. This program handles the Option by calling copied to get an Option>i32< rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn't have an entry for the key.

        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);

* Inserting with a key that is associated to an existing value will just replace the existing value with the new one that is to be inserted (the second insertion)

* Hash maps have a special API for this called entry that takes the key you want to check as a parameter. The return value of the entry method is an enum called Entry that represents a value that might or might not exist. 

* By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables

        https://en.wikipedia.org/wiki/SipHash

* See the Summary in 8.3 "Storing Keys with Associated Values in Hash Maps" for exercises on the common collections in chapter 8











        

    
