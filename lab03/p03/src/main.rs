fn checked_addition_relaxed(a: u32, b: u32) -> Result<u32, &'static str> {
    let x: u64 = a as u64 + b as u64;
    if x > u32::MAX as u64 {
        return Err("Result exceeds the bounds of u32!");
    } else {
        return Ok(x as u32);
    }
}

fn checked_multiplication_relaxed(a: u32, b: u32) -> Result<u32, &'static str> {
    let x: u64 = a as u64 * b as u64;
    if x > u32::MAX as u64 {
        return Err("Result exceeds the bounds of u32!");
    } else {
        return Ok(x as u32);
    }
}

fn main() {
    let a: u32 = 2;
    let c: u32 = u32::MAX;

    println!(
        "{:?} {:?} {:?}",
        checked_addition_relaxed(a, a),
        checked_addition_relaxed(a, c),
        checked_multiplication_relaxed(a, c)
    );
}
