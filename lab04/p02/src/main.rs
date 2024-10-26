fn rot13(s: &String) {
    let mut s2 = String::from("");
    for mut c in s.chars() {
        if c.is_ascii() {
            if c >= 'a' && c <= 'z' {
                c = ((c as u8 + 13) % 25 + 97) as char;
            }
            if c >= 'A' && c <= 'Z' {
                c = ((c as u8 + 13) % 25 + 61) as char;
            }
            s2.push(c);
        } else {
            println!("Error! String contains a non-ascii character.");
            return;
        }
    }
    println!("{s2}");
}

fn main() {
    let s = String::from("This is a text I will apply the ROT13 cypher to.");
    rot13(&s);
}
