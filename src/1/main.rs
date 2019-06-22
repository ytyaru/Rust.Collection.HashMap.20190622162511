/*
 * RustのコレクションHashMap型。
 * CreatedAt: 2019-06-22
 */
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("A"), 1);
    scores.insert(String::from("B"), 2);
//    scores.insert(String::from("C"), String::from("3")); // error[E0308]: mismatched types
//    scores.insert('D', 4); // error[E0308]: mismatched types
    println!("{:?}", scores);
}

