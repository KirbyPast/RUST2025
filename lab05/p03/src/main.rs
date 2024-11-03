use std::fs;
use serde_derive::Deserialize;

#[derive(Debug,Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: i32,
}

fn main() {
    let contents = fs::read_to_string("src/students.txt").unwrap();
    let s: Student = serde_json::from_str(&contents).unwrap();
    println!("{:?}",s);
}
