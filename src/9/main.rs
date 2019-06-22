/*
 * RustのコレクションHashMap型。
 * CreatedAt: 2019-06-22
 */
use std::collections::HashMap;

fn main() {
    let text = "A B A C";
    let mut words_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = words_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", words_count);
}

