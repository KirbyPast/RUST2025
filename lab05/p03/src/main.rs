use serde_derive::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: i32,
}

fn main() {
    let contents = fs::read_to_string("src/students.json").unwrap();
    let s: [Student; 4] = serde_json::from_str(&contents).unwrap();

    let mut idx = 0;
    let mut youngest = 999;
    let mut youngest_idx = 0;
    let mut oldest = -1;
    let mut oldest_idx = 0;

    while idx < 4 {
        if s[idx].age > oldest {
            oldest_idx = idx;
            oldest = s[idx].age;
        }
        if s[idx].age < youngest {
            youngest_idx = idx;
            youngest = s[idx].age;
        }
        idx = idx + 1;
    }
    println!(
        "{:?} is the oldest student, aged {:?}",
        s[oldest_idx].name, s[oldest_idx].age
    );
    println!(
        "{:?} is the youngest student, aged {:?}",
        s[youngest_idx].name, s[youngest_idx].age
    );
}
