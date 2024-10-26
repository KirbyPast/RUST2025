use std::{ fs};

fn do_stuff() {
    let s = fs::read_to_string("src/input.txt");
    let mut s2 = String::from("");
    for mut word in s.unwrap().split(" ") {
        if word == "pt" || word == "ptr" {
            word = "pentru";
        }
        else if word == "dl" {
            word = "domnul";
        }
        else if word == "dna" {
            word = "doamna";
        }
        s2.push_str(word);
        s2.push(' ');
    }
    if fs::write("src/input.txt",&s2).is_err() {
        println!("Failed at writing!");
        return;
    }
}

fn main() {
    do_stuff();
}
