/*
 * RustのコレクションHashMap型。
 * CreatedAt: 2019-06-22
 */
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("A".to_string(), 1);
    scores.insert("B".to_string(), 2);
    println!("{:?}", &scores.get(&"A".to_string())); // Some(1)
    println!("{:?}", &scores.get(&"B".to_string())); // Some(2)
    println!("{:?}", scores.get(&"A".to_string())); // Some(1)
    println!("{:?}", scores.get(&"B".to_string())); // Some(1)
//    println!("{:?}", scores.get("A".to_string())); // error[E0308]: mismatched types
//    println!("{:?}", scores.get("B".to_string())); // error[E0308]: mismatched types
    println!("{:?}", scores.get("A")); // Some(1)
    println!("{:?}", scores.get("B")); // Some(1)
//    println!("{:?}", &scores.get("A".to_string()[..])); // error[E0308]: mismatched types
//    println!("{:?}", &scores.get("B".to_string()[..])); // error[E0308]: mismatched types

    let team_name = String::from("A");
    let score = scores.get(&team_name);
    println!("{} {:?}", team_name, score);
    let score = scores.get(&team_name[..]);
    println!("{} {:?}", team_name, score);
}

