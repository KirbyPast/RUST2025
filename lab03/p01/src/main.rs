fn is_prime(x: u16) -> bool {
    if x < 2 {
        return false;
    }
    let mut i = 3;
    while i <= (x as f64).sqrt() as u16 {
        if x % i == 0 {
            return false;
        }
        i = i + 1;
    }

    return true;
}

fn next_prime(x: u16) -> Option<u16> {
    for n in x + 1..u16::MAX {
        if is_prime(n) == true {
            return Some(n);
        }
    }
    return None;
}

fn main() {
    let mut x: Option<u16> = Some(1);
    while x != None {
        println!("{}", x.unwrap());
        x = next_prime(x.unwrap());
    }
}
