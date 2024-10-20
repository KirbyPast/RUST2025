#[derive(Debug)]
enum MyError {
    NotAscii,
    NotDigit,
    NotBase16,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(mut x: char) -> Result<char, MyError> {
    if x >= 'a' && x <= 'z' {
        x = (x as u8 - ('a' as u8 - 'A' as u8)) as char;
        return Ok(x);
    } else if x >= 'A' && x <= 'Z' {
        return Ok(x);
    } else {
        return Err(MyError::NotLetter);
    }
}

fn to_lowercase(mut x: char) -> Result<char, MyError> {
    if x >= 'A' && x <= 'Z' {
        x = (x as u8 + ('a' as u8 - 'A' as u8)) as char;
        return Ok(x);
    } else if x >= 'a' && x <= 'z' {
        return Ok(x);
    } else {
        return Err(MyError::NotLetter);
    }
}

fn print_char(x: char) -> Result<(), MyError> {
    if x.is_control() {
        return Err(MyError::NotPrintable);
    } else {
        println!("{x}");
        return Ok(());
    }
}

fn char_to_number(x: char) -> Result<u32, MyError> {
    if x.is_ascii() && x.is_digit(10) {
        return Ok(x.to_digit(10).unwrap());
    } else {
        if !x.is_ascii() {
            return Err(MyError::NotAscii);
        } else {
            return Err(MyError::NotDigit);
        }
    }
}

fn char_to_number_hex(x: char) -> Result<u32, MyError> {
    if x.is_ascii() && x.is_digit(16) {
        return Ok(x.to_digit(16).unwrap());
    } else {
        if !x.is_ascii() {
            return Err(MyError::NotAscii);
        } else {
            return Err(MyError::NotBase16);
        }
    }
}

fn print_error(e: MyError) {
    match e {
        MyError::NotAscii => println!("Character is not ascii!"),
        MyError::NotBase16 => println!("Character is not base 16!"),
        MyError::NotDigit => println!("Character is not a digit!"),
        MyError::NotLetter => println!("Character is not a letter!"),
        MyError::NotPrintable => println!("Character is not printable!"),
    }
}
fn main() {
    println!(
        "{:?}, {:?}, {:?}",
        to_uppercase('a'),
        to_uppercase('B'),
        to_uppercase('6')
    );

    println!(
        "{:?}, {:?}, {:?}",
        to_lowercase('a'),
        to_lowercase('B'),
        to_lowercase('6')
    );

    match print_char('x') {
        Ok(_) => {}
        Err(e) => println!("Error! {:?}", e),
    }

    match print_char('\n') {
        Ok(_) => {}
        Err(e) => println!("Error! {:?}", e),
    }

    match char_to_number('5') {
        Ok(x) => println!("{x}"),
        Err(e) => println!("{:?}", e),
    }

    match char_to_number('g') {
        Ok(x) => println!("{x}"),
        Err(e) => println!("Error! {:?}", e),
    }

    match char_to_number('\n') {
        Ok(x) => println!("{x}"),
        Err(e) => println!("Error! {:?}", e),
    }

    match char_to_number_hex('1') {
        Ok(x) => println!("{x}"),
        Err(e) => println!("Error! {:?}", e),
    }

    match char_to_number_hex('G') {
        Ok(x) => println!("{x}"),
        Err(e) => println!("Error! {:?}", e),
    }

    match char_to_number_hex('\n') {
        Ok(x) => println!("{x}"),
        Err(e) => println!("Error! {:?}", e),
    }

    print_error(MyError::NotAscii);
    print_error(MyError::NotBase16);
    print_error(MyError::NotDigit);
}
