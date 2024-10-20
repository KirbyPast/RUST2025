//Equilateral triangle area calculator.

enum MyError {
    NotEqualSides,
    NegativeSide,
    NullSide,
}

fn area_calculator(x: f32, y: f32, z: f32) -> Result<f32, MyError> {
    if x != y || x != z || y != z {
        return Err(MyError::NotEqualSides);
    }
    if x < 0.0 || y < 0.0 || z < 0.0 {
        return Err(MyError::NegativeSide);
    }
    if x == 0.0 || y == 0.0 || z == 0.0 {
        return Err(MyError::NullSide);
    }
    let y: f32 = 3.0;
    let area: f32 = (y.sqrt() / (4 as f32)) * x * x;
    return Ok(area);
}

fn error_translator(e: MyError) {
    match e {
        MyError::NullSide => println!("Sides can't be 0!"),
        MyError::NegativeSide => println!("Sides have to be positive!"),
        MyError::NotEqualSides => println!("Sides have to be equal!"),
    }
}

fn main() {
    match area_calculator(1.0, 2.0, 3.0) {
        Ok(x) => println!("{x}"),
        Err(e) => error_translator(e),
    }
    match area_calculator(-1.0, -1.0, -1.0) {
        Ok(x) => println!("{x}"),
        Err(e) => error_translator(e),
    }
    match area_calculator(0.0, 0.0, 0.0) {
        Ok(x) => println!("{x}"),
        Err(e) => error_translator(e),
    }
    match area_calculator(3.0, 3.0, 3.0) {
        Ok(x) => println!("{x}"),
        Err(e) => error_translator(e),
    }
}
