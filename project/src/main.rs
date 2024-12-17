use std::env;
use std::fs::File;
use std::io::{Read, Write};
static SPLIT_SIZE: usize = 1024 * 1024; //Default split size is 1mB. 1024 bytes = 1kB * 1024 bytes = 1mB

fn incorrect_usage(args: Vec<String>) {
    println!("Error! Invalid command!");
    println!(
        "Usage: {:?} split (file_path) [-s [num]b/kb/mb/gb] 
        OR 
        {:?} unsplit (file_path)",
        args[0], args[0]
    );
}

fn update_split_size(multiplier: usize, multiplier_size: String) -> Result<usize, String> {
    let mut size: usize = 1024 * 1024; //Default is 1mb
    match multiplier_size.to_lowercase().as_str() {
        "b" => {
            size = size / (1024 * 1024) * multiplier;
            return Ok(size);
        }
        "kb" => {
            size = size / 1024 * multiplier;
            return Ok(size);
        }
        "mb" => {
            size = size * multiplier;
            return Ok(size);
        }
        "gb" => {
            size = size * 1024 * multiplier;
            return Ok(size);
        }
        _ => {
            println!("Invalid format of size!!!");
            return Err(String::from("Invalid format"));
        }
    }
}

fn split(file_path: &String, size: usize) -> Result<(), String> {
    let mut file = File::open(file_path).unwrap();
    let mut buffer = vec![0; size];
    let mut part_count = 0;

    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            return Ok(());
        }

        part_count = part_count + 1;
        let part_file_name = format!("{}.part{}.split", file_path, part_count);

        match File::create(&part_file_name) {
            Ok(mut part_file) => {
                println!("Succesfully created file");
                match part_file.write_all(&buffer[..bytes_read]) {
                    Ok(()) => {}
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!(
            "Usage: {:?} split (file_path) [-s [num]b/kb/mb/gb] 
                    OR 
                    {:?} unsplit (file_path)",
            args[0], args[0]
        );
        return Err(String::from("Invalid usage!"));
    } else {
        //let program_path: &String = &args[0];
        let command: &String = &args[1];

        let file_path = &args[2];
        match command.as_str() {
            "split" => {
                println!("Detected split command!");
                if args.len() > 4 {
                    //Means client has probably changed size.
                    let size_check: &String = &args[3];
                    match size_check.as_str() {
                        "-s" => {
                            let mut multiplier_string: String = String::from("");
                            let mut multiplier_size: String = String::from("");
                            let new_size: &String = &args[4];
                            for i in new_size.chars() {
                                if !i.is_digit(10) {
                                    multiplier_size.push(i);
                                } else {
                                    multiplier_string.push(i);
                                }
                            }
                            let multiplier: usize = multiplier_string.parse().unwrap();
                            let size = update_split_size(multiplier, multiplier_size)?;
                            split(file_path, size)?;

                            return Ok(());
                        }
                        _ => {
                            //Doesn't matter args is destroyed when entering function since program is over.
                            incorrect_usage(args);
                            return Err(String::from("Incorrect usage"));
                        }
                    }
                } else {
                    split(file_path, SPLIT_SIZE)?;
                }
            }
            "unsplit" => {
                println!("Detected unsplit command!");
                //unsplit(file_path)?;
                return Ok(());
            }
            _ => {
                //Doesn't matter args is destroyed when entering function since program is over.
                incorrect_usage(args);
                return Err(String::from("Incorrect usage"));
            }
        }
    }
    return Ok(());
}
