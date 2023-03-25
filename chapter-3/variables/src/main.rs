use rand::Rng;

fn main() {
    let x = 5;

    // New variable x
    let x = x + 1;

    {
        // New variable x (only in this scope)
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    
    another_function(x);
    print_labeled_measurement(5, 'h');
    let z = five();
    println!("The value of just a value in a function is {z}");
    let zz = plus_one(-5);
    println!("The value of zz is {zz}");
    let controlFlow = control_flow(4);
    println!("The value of controlflow is {controlFlow}");

    // Resets to 6 now that we exited the inner scope

    println!("The value of x is: {x}");
    farenheit_to_celsius(537.778, "C");
    let fib: i32 = genNthFib(5);
    println!("Nth fib: {fib}");
    printXDaysOfChristmas(4);

}

fn farenheit_to_celsius(x: f32, s:&str) {

    match s {
        "C" => {
            let result = (x * (9.0/5.0)) + 32.0;
            println!("{x} degrees celsius is {result} degrees in farenheit");
        }
        "F" => {
            let result = (x - 32.0) * 5.0/9.0;
            println!("{x} degrees in farenheit is {result} degrees celsius");
        }
        _ => {
            println!("Reform your input!");
        }
    }
}

fn genNthFib(x: u32) -> i32 {

    if x == 0 || x == 1 {
        1
    }
    else {
        genNthFib(x - 1) + genNthFib(x - 2)
    }
}

fn printXDaysOfChristmas(x: u32) {

    let mut secret_number = 0;
    let items = ["snow", "jelly", "grapes",
    "doves", "liberation", "resistance", "boots",
    "phone", "train-set", "computer", "cod", "valorant"];

    for num in 0..x {
        secret_number = rand::thread_rng().gen_range(0..=11);
        let retrieval = items[secret_number];
        let day = num + 1;
        println!("On the {day} day of chrizzmas, my true love gave to me, {retrieval}");
    }
}


fn control_flow(x: i32) -> i32 {
    let mut counter = x;

    let result = loop {
        counter += 1;
        println!(" control_flow execution: {counter}");
        if counter == 10 {
            break counter * 2;
        }
    };

     result
}

fn plus_one(x: i32) -> i32 {
    // Adding a semicolon would cause compiler errors
    x + 1
}

fn for_loops() {

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn labeled_loops() {

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
}

// Expression whose value we want to return
fn five() -> i32 {
    5
}

fn another_function(x: i32) {
    println!("another function call with value {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}