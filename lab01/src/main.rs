fn prime_check(x: i32) -> bool {
    let mut d: i32 = 2;
    if x == 1 || (x > 2 && x % 2 == 0) || x == 0 {
        return false;
    }
    while d < x {
        if x % d == 0 {
            return false;
        }
        d = d + 1;
    }
    return true;
}

fn coprime_checker(mut x: i32, mut y: i32) -> bool {
    if x == 0 || y == 0 {
        return false;
    }
    if x == 1 || y == 1 {
        return true;
    }
    while x != y {
        if x > y {
            x = x - y;
        } else if x < y {
            y = y - x;
        }
    }
    if x == 1 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut x: i32 = 0;
    while x <= 100 {
        if prime_check(x) == true {
            println!("{x} is prime!");
        }
        x = x + 1;
    }

    let mut y: i32 = 0;
    let mut z: i32 = 0;

    while y <= 100 {
        while z <= 100 {
            if coprime_checker(y, z) == true {
                println!("{y} and {z} are coprime!");
            }
            z = z + 1;
        }
        y = y + 1;
        z = 0;
    }

    let mut t: i32 = 99;

    while t >= 1 {
        if t == 1 {
            println!("{t} bottle of beer on the wall,");
            println!("{t} bottle of beer.");
        } else {
            println!("{t} bottles of beer on the wall,");
            println!("{t} bottles of beer.");
        }
        println!("Take one down, pass it around,");

        t = t - 1;

        if t == 0 {
            println!("No bottles of beer on the wall.\n");
        } else if t == 1 {
            println!("{t} bottle of beer on the wall.\n");
        } else {
            println!("{t} bottles of beer on the wall.\n");
        }
    }
}
