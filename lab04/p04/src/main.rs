use std::{fs};

fn do_stuff() {
    //hosts file was full of comments.
    let s = fs::read_to_string("F:/RUST/hosts.txt");

    for line in s.unwrap().lines(){
        if line.starts_with("#") {
            continue;
        }
        else {
            let mut ok = 0;
            let mut s2 =  String::from("");
            for word in line.split_whitespace() {
                if ok == 0 {
                    s2.push_str(word);
                    ok = 1;
                }
                else {
                    s2.push_str(" => ");
                    s2.push_str(word);
                }
            }
            println!("{s2}");
        }
    }


}

fn main() {
    do_stuff();
}
