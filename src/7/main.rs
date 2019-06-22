/*
 * RustのコレクションHashMap型。
 * CreatedAt: 2019-06-22
 */
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("A".to_string(), 1);
    scores.insert("A".to_string(), 10);
    println!("{:?}", scores);
}

