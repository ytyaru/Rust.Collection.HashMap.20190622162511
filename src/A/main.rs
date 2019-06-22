/*
 * RustのコレクションHashMap型。
 * CreatedAt: 2019-06-22
 */
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("A", 1);
    scores.insert("B", 2);
    println!("{:?}", scores);
    println!("{:?}", scores.get("A"));
    println!("{:?}", scores.get("B"));
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let teams = vec!["A", "B"];
    let points = vec![1, 2];
    let mut scores: HashMap<_,_> = teams.iter().zip(points.iter()).collect();
    println!("{:?}", scores);
}

