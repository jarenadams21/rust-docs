fn main() {
    println!("Hello, world!");
    mutable_string();
}

fn string_literals() {

    {
        let s = "hello";
    }
}

fn mutable_string() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends literal to string
    println!("{s}");

}

fn memory_pointing_to_heap() {

    // THIS,
    let s1 = String::from("hello");
    let s2 = s1;

    // IS NOT THE SAME AS:
    let x = 5;
    let y = x;
}

// Heap data gets copied, with two different pointers
// to two identical spaces of data in the heap
fn copy_heap_data_with_clone() {

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{s1} and {s2}");
}

/* 


*/
 