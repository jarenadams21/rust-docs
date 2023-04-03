fn main() {
    println!("Hello, world!");
}

// The reason we’re able to use &s2 in the call
//  to add is that the compiler can 
// coerce the &String argument into a &str
fn plus_operator_deref_coercion() {

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // note s1 has been moved here and 
                         // can no longer be used
}

fn format_macro() {

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
}

// Rust does not support indexing
fn indexing_error() {
    let s1 = String::from("hello");
    let h = s1[0];
}

fn slicing_strings() {

    let hello = "Здравствуйте";

    let s = &hello[0..4]; // Зд since each charater in Devanagari script is 2 bytes
}

fn iteration_over_chars_in_string() {

    for c in "Зд".chars() {
        println!("{c}");
    }

    // Ouput: З
    //        д

    for b in "Зд".bytes() {
        println!("{b}");
    }
    /* Output:
        208
        151
        208
        180 
    */
}
