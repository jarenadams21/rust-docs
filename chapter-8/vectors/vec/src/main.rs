fn main() {
    let v: Vec<i32> = Vec::new();

    // vec macro can infer types for vectors
    let v = vec![1, 2, 3];
    println!("Hello, world!");
}

fn update_vec() {

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

fn reading_elements() {

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

fn handling() {

    let v = vec![1, 2, 3, 4, 5];

    // Error
    let does_not_exist = &v[100];
    // None of Option<T>
    let does_not_exist = v.get(100);
}

fn iterating_over_values_in_vecs() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
}

// To change the value that the mutable reference
//  refers to, we have to use the * dereference 
// operator to get to the value in i before 
// we can use the += operator.
fn iterate_over_mutable_references() {

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

}

fn use_enums_to_represent_multiple_vec_types() {
    
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
