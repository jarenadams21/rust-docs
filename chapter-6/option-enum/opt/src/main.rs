fn main() {

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
    println!("Hello, world!");
    dbg!(&some_number);
    dbg!(&some_char);
    dbg!(&absent_number);
}
