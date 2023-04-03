fn main() {
    println!("Hello, world!");
}

fn new_empt_string() {
    let mut s = String::new();
}

fn convert_literal_to_string() {

    let data = "initial contents";

    // [ String ] type
    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

}

fn convert_literal_using_from() {

    let s = String::from("initial contents");
}

fn valid_string_values() {

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

// The push_str method takes a string slice 
// because we don’t necessarily want to take 
// ownership of the parameter. 
fn grow_string_with_push() {

    // Appending a string slice
    let mut s = String::from("foo");
    s.push_str("bar");
}

fn dont_take_ownership() {
    
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

}

// The push method takes a single character as a 
// parameter and adds it to the String.
fn push_one_parameter() {

    let mut s = String::from("lo");
    s.push('l');

}
