use adler::adler32;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
static SPLIT_SIZE: usize = 1024 * 1024;

#[derive(Serialize, Deserialize)]
struct FileHash {
    part: String,
    hash: u32,
}

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
    let mut size: usize = 1024 * 1024;
    match multiplier_size.to_lowercase().as_str() {
        "" | "b" => {
            size = size / (1024 * 1024) * multiplier;
            Ok(size)
        }
        "kb" => {
            size = size / 1024 * multiplier;
            Ok(size)
        }
        "mb" => {
            size *= multiplier;
            Ok(size)
        }
        "gb" => {
            size = size * 1024 * multiplier;
            Ok(size)
        }
        _ => {
            println!("Invalid format of size!!!");
            Err(String::from("Invalid format"))
        }
    }
}

fn split(file_path: &String, size: usize) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer = vec![0; size];
    let mut part_count = 0;
    let mut hashes: Vec<FileHash> = Vec::new();

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        part_count += 1;
        let part_file_name = format!("{}.part{}.split", file_path, part_count);

        let mut part_file = File::create(&part_file_name)?;

        println!("Succesfully created file {part_count}");
        part_file.write_all(&buffer[..bytes_read])?;

        let x = adler32(&buffer[..bytes_read])?;

        hashes.push(FileHash {
            part: part_file_name,
            hash: (x),
        });
    }

    let json_file_name = format!("{}.hashes.json", file_path);
    let mut json_file = File::create(json_file_name)?;
    let json_data = serde_json::to_string(&hashes)?;
    json_file.write_all(json_data.as_bytes())?;

    println!("Succesfuly split file!");
    Ok(())
}

fn unsplit(file_path: &String) -> Result<(), Box<dyn Error>> {
    let json_file_name = format!("{}.hashes.json", file_path);
    let json_file = File::open(json_file_name)?;

    let mut output_file = File::create(file_path)?;

    let hashes: Vec<FileHash> = serde_json::from_reader(json_file)?;

    let mut file_number = 0;

    for file_info in hashes {
        file_number += 1;
        if !Path::new(&file_info.part).exists() {
            println!("Error! Missing part {file_number}. Created file with remaining parts.");
            return Ok(());
        }

        let mut input_file = File::open(file_info.part)?;
        let mut buff: Vec<u8> = Vec::new();
        let bytes_read = File::read_to_end(&mut input_file, &mut buff)?;

        let new_hash = adler32(&buff[..bytes_read])?;

        if new_hash != file_info.hash {
            println!("Error! Part {file_number} is corrupt. Creating file with the other parts");
            return Ok(());
        } else {
            output_file.write_all(&buff)?;
        }
    }
    println!("Succesfully unsplit file!");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    if command == "help" {
        println!(
            "Usage: 
            1. {} split (file_path) [-s [num]b/kb/mb/gb] 
            2. {} unsplit (file_path)",
            args[0], args[0]
        );
        return Ok(());
    } else if args.len() < 3 {
        incorrect_usage(args);
        std::process::exit(1);
    } else {
        let file_path = &args[2];
        match command.as_str() {
            "split" => {
                println!("Attempting to split file...");
                if args.len() > 3 {
                    let size_check: &String = &args[3];
                    match size_check.as_str() {
                        "-s" => {
                            let mut multiplier_string: String = String::from("");
                            let mut multiplier_size: String = String::from("");
                            let new_size: &String = &args[4];
                            for i in new_size.chars() {
                                if !i.is_ascii_digit() {
                                    multiplier_size.push(i);
                                } else {
                                    multiplier_string.push(i);
                                }
                            }
                            let multiplier: usize = multiplier_string.parse()?;
                            let size = update_split_size(multiplier, multiplier_size)?;
                            split(file_path, size)?;

                            return Ok(());
                        }
                        _ => {
                            incorrect_usage(args);
                            std::process::exit(1);
                        }
                    }
                } else {
                    split(file_path, SPLIT_SIZE)?;
                }
            }
            "unsplit" => {
                println!("Attempting to unsplit file...");
                unsplit(file_path)?;
                return Ok(());
            }
            _ => {
                incorrect_usage(args);
                std::process::exit(1);
            }
        }
    }
    Ok(())
}
