fn checked_addition(a: u32, b: u32) -> u32 {
    let x: u64 = a as u64 + b as u64;
    if x > u32::MAX as u64 {
        panic!("Addition was too powerful!");
    } else {
        return x as u32;
    }
}

fn checked_multiplication(a: u32, b: u32) -> u32 {
    let x: u64 = a as u64 * b as u64;
    if x > u32::MAX as u64 {
        panic!("Multiplication was too powerful!");
    } else {
        return x as u32;
    }
}

fn main() {
    let a: u32 = 2;
    let b: u32 = 4;
    let c: u32 = u32::MAX;

    println!(
        "{} {} {}",
        checked_addition(a, b),
        checked_multiplication(a, b),
        checked_addition(a, c)
    );
    println!("Hello, world!");
}
