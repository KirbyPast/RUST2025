use std::{fs};

fn do_stuff() {
    let s = fs::read_to_string("src/input.txt");
    let mut max_len_index = 0;
    let mut max_len = 0;
    let mut max_size_index = 0;
    let mut max_size = 0;
    let mut index = 0;
    for line in s.unwrap().lines() {
        if line.len()>max_len{
            max_len = line.len();
            max_len_index = index;
        }
        if line.chars().count() > max_size{
            max_size = line.chars().count();
            max_size_index = index;
        }
        index = index + 1;
    }
    index = 0;
    let s = fs::read_to_string("src/input.txt");
    for line in s.unwrap().lines() {
        if index == max_size_index {
            println!("Most characters: {line}");
        }

        if index == max_len_index {
            println!("Most bytes: {line}")
        }

        index = index + 1;
    }

}
fn main() {
    do_stuff()
}
