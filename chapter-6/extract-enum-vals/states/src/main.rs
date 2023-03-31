#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// If we were to call value_in_cents(Coin::Quarter(UsState::Alaska)),
//  coin would be Coin::Quarter(UsState::Alaska). 
// When we compare that value with each of the match arms, 
// none of them match until we reach Coin::Quarter(state). 
// At that point, the binding for state will be the value 
// UsState::Alaska. We can then use that binding in the println!
//  expression, thus getting the inner state value out of
//  the Coin enum variant for Quarter.
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Hello, world!");
}
