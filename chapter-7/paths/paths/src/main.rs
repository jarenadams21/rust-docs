use std::collections::HashMap;

/* Nested Paths */
// --snip--
use std::{cmp::Ordering, io};
// --snip--

/* Brings std::io & std::io::Write
into scope */
use std::io::{self, Write};

/* Glob operator */
use std::collections::*;


fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}