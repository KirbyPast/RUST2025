fn add_space(s: &mut String, mut n: i32) {
    while n > 0 {
        s.push_str(" ");
        n = n - 1;
    }
}

fn add_str(s: &mut String, s2: &str) {
    s.push_str(&s2);
}

fn add_integer(s: &mut String, mut n: i32) {
    if n < 0 {
        s.push('-');
        n = -n;
    }
    let mut s2: String = String::from("");
    while n > 0 {
        let c: char = ((n % 10) as u8 + '0' as u8) as char;
        s2.push(c);
        n = n / 10;
    }
    let number_length = s2.len() as i32;
    let mut i = s2.len();
    let mut counter: i32 = 0;
    loop {
        i = i - 1;
        counter = counter + 1;
        let c: char = s2.remove(s2.len() - 1);
        s.push(c);
        if ( (counter.abs_diff(number_length)) % 3) % 3 == 0 && counter != number_length {
            s.push('_');
        }
        if i == 0 {
            break;
        }
    }
}


fn add_integer_without_separators(s: &mut String, mut n: i32) {
    if n < 0 {
        s.push('-');
        n = -n;
    }
    let mut s2: String = String::from("");
    while n > 0 {
        let c: char = ((n % 10) as u8 + '0' as u8) as char;
        s2.push(c);
        n = n / 10;
    }
    let mut i = s2.len();
    loop {
        i = i - 1;
        let c: char = s2.remove(s2.len() - 1);
        s.push(c);
        if i == 0 {
            break;
        }
    }
}

fn add_float(s: &mut String, mut n:f32){
    let x = n as i32;
    add_integer(s,x);
    s.push('.');
    let mut m = 0;
    while n.trunc() != n {
        if m == 0 && n.trunc() as i32 % 10 == 0 {
            s.push('0');
        }
        n = n * 10 as f32;
        m = m * 10 + (n.trunc() as i32 % 10);
    }
    add_integer_without_separators(s,m);
}

fn main() {
    let mut s: String = String::from("");
    let statement: String = String::from("Most downloaded crate has 306_437_968 downloads and the latest version is 2.038.");

    add_space(&mut s, statement.len() as i32 / 2 );
    add_str(&mut s, "I 💚");
    //add_space(&mut s, statement.len() as i32 / 2 );
    add_str(&mut s, "\n");
    add_space(&mut s, statement.len() as i32 / 2 );
    add_str(&mut s, "RUST");
    add_str(&mut s, "\n");
    add_str(&mut s, "\n");

    add_str(&mut s, "Most");
    add_space(&mut s, "downloaded".len() as i32 + 2);
    add_str(&mut s, "crate");
    add_space(&mut s, "has".len() as i32 + 2);
    add_integer(&mut s,306437968);
    add_space(&mut s, "downloads".len() as i32 + 2);
    add_str(&mut s, "and");
    add_space(&mut s, "the".len() as i32 + 2);
    add_str(&mut s, "lastest");
    add_space(&mut s, "version".len() as i32 + 2);
    add_str(&mut s, "is");
    add_space(&mut s, "2.038".len() as i32 + 2);
    add_str(&mut s,"\n");

    add_space(&mut s, "Most".len() as i32 + 2);
    add_str(&mut s, "downloaded");
    add_space(&mut s, "crate".len() as i32 + 2);
    add_str(&mut s, "has");
    add_space(&mut s,"306437968".len() as i32 + 2);
    add_str(&mut s, "downloads");
    add_space(&mut s, "and".len() as i32 + 2);
    add_str(&mut s, "the");
    add_space(&mut s, "lastest".len() as i32 + 2);
    add_str(&mut s, "version");
    add_space(&mut s, "is".len() as i32 + 2);
    add_float(&mut s, 12.607);
    add_str(&mut s,".");

    println!("{s}");
}
