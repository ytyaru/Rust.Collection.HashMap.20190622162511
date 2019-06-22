/*
 * RustのコレクションHashMap型。
 * CreatedAt: 2019-06-22
 */
use std::collections::HashMap;

fn main() {
    let teams = vec!["A".to_string(), "B".to_string()];
    let points = vec![1, 2];
//    let scores = teams.iter().zip(points.iter()).collect(); // error[E0282]: type annotations needed
//    let scores: HashMap = teams.iter().zip(points.iter()).collect(); // error[E0107]: wrong number of type arguments: expected at least 2, found 0
//    let scores: HashMap<String,i32> = teams.iter().zip(points.iter()).collect(); // error[E0277]: a collection of type `std::collections::HashMap<std::string::String, i32>` cannot be built from an iterator over elements of type `(&std::string::String, &{integer})`
    let scores: HashMap<_,_> = teams.iter().zip(points.iter()).collect();
    println!("{:?}", scores);
}

