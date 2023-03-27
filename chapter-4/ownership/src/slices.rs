fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word_slice(&my_string[0..6]);
    let word = first_word_slice(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_slice(&my_string);

    // String literals are immutable
    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word_slice(&my_string_literal[0..6]);
    let word = first_word_slice(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_slice(my_string_literal);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // Convert String to an array of bytes

    // 'iter' is a method that returns each element in a collection
    // and 'enumerate' wraps the result of 'iter' and returns each element
    // as part of a tuple instead
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}