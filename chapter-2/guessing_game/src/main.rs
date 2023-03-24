// input output library in scope
use std::io;
use rand::Rng;
use std::cmp::Ordering;

// Variables are immutable by default
// 'mut' identifier makes it mutable
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is 8");
    
    loop {
    println!("Please input your guess.");
    println!("Type 'quit' to exit...");

    // immutable would be 'let guess = String::new();'
    // new is an associated function, implemented on a type
    let mut guess = String::new();

    // references are immutable by default like variables
    // thats why we added &mut as opposed to &guess
    io::stdin()
    // read_line returns a 'Result" value 
    // which is an enum with variants 'Ok' and 'Err'
    // if Err is returned from the instance, expect method is called
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed: {guess}, I hope you didn't take my advice...");

    // Ordering type is an enum with variants Less, Greater, Equal
    // match expressions look at one of their arms, 3 in this case,
    // and match the output of the comparison/function
    // to said arms. After finding a matching arm, it excecutes
    // its associating code
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small..."),
        Ordering::Greater => println!("Too big..."),
        Ordering::Equal => {
            println!("...You win!");
            break;
        }
    };
}
}
