enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    LuckyPenny
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::LuckyPenny => {
            println!("Lucky penny!");
            1
        }
    }
}

fn main() {

    let penni = Coin::Penny;
    let nick = Coin::Nickel;
    let dime = Coin::Dime;
    let quart = Coin::Quarter;
    let lucky = Coin::LuckyPenny;

    println!("Hello, world!, {} ", value_in_cents(penni));
    println!("Hello, world!, {} ", value_in_cents(nick));
    println!("Hello, world!, {} ", value_in_cents(dime));
    println!("Hello, world!, {} ", value_in_cents(quart));
    println!("Hello, world!, {} ", value_in_cents(lucky));
}
