/*
 * RustのコレクションHashMap型。
 * CreatedAt: 2019-06-22
 */
use std::collections::HashMap;

fn main() {
    let key = "key1".to_string();
    let value = 100;
    let mut map = HashMap::new();
    map.insert(key, value); // 所有権がムーブされる(key,value)    error[E0382]: use of moved value: `key`
    println!("{:?}", map);
    println!("{} {}", key, value);
}

