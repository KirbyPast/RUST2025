use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde_derive::Serialize;
use serde_derive::Deserialize;
use serde_json;
use adler::adler32;
static SPLIT_SIZE: usize = 1024 * 1024; //Default split size is 1mB. 1024 bytes = 1kB * 1024 bytes = 1mB

#[derive(Serialize,Deserialize)]
struct FileHash{
    part: String,
    hash: u32
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
    let mut size: usize = 1024 * 1024; //Default is 1mb
    match multiplier_size.to_lowercase().as_str() {
        "" => {
            //"No suffix means bytes"
            size = size / (1024 * 1024) * multiplier;
            return Ok(size);
        }
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

fn split(file_path: &String, size: usize) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer = vec![0; size];
    let mut part_count = 0;
    let mut hashes: Vec <FileHash> =  Vec::new();

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        part_count = part_count + 1;
        let part_file_name = format!("{}.part{}.split", file_path, part_count);

        let mut part_file = File::create(&part_file_name)?;

        println!("Succesfully created file");
        part_file.write_all(&buffer[..bytes_read])?;

        let  x = adler32(&buffer[..bytes_read])?;
        println!("{:?}",x);

        hashes.push(FileHash { part: part_file_name, hash: (x) });
    }

    let json_file_name = format!("{}.hashes.json", file_path);
    let mut json_file = File::create(&json_file_name)?;
    let json_data = serde_json::to_string(&hashes)?;
    json_file.write_all(json_data.as_bytes())?;

    return Ok(());
}

fn unsplit(file_path: &String) -> Result<(), Box<dyn Error>> {
    let mut output_file = File::create(file_path)?;

    let json_file_name = format!("{}.hashes.json", file_path);
    let json_file = File::open(&json_file_name)?;
    let hashes: Vec<FileHash> = serde_json::from_reader(json_file)?;
    

    for file_info in hashes {
        if !Path::new(&file_info.part).exists() {
            println!("Error! Missing part.");
            return Ok(());
        }

        let mut input_file = File::open(file_info.part)?;
        let mut buff : Vec<u8> = Vec::new();
        let bytes_read = File::read_to_end(&mut input_file, &mut buff)?;

        let new_hash = adler32(&buff[..bytes_read])?;

        if new_hash!=file_info.hash {
            println!("Error! Corrupted parts. Aborting.");
            return Ok(());
        }
        else{
            output_file.write_all(&buff)?;
        }
    }

    return Ok(());
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!(
            "Usage: {:?} split (file_path) [-s [num]b/kb/mb/gb] 
                    OR 
                    {:?} unsplit (file_path)",
            args[0], args[0]
        );
        std::process::exit(1);
    } else {
        //let program_path: &String = &args[0];
        let command: &String = &args[1];

        let file_path = &args[2];
        match command.as_str() {
            "split" => {
                println!("Detected split command!");
                if args.len() > 3 {
                    //Means client has probably changed size.
                    let size_check: &String = &args[3];
                    println!("{size_check}");
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
                            let multiplier: usize = multiplier_string.parse()?;
                            let size = update_split_size(multiplier, multiplier_size)?;
                            split(file_path, size)?;

                            return Ok(());
                        }
                        _ => {
                            //Doesn't matter args is destroyed when entering function since program is over.
                            incorrect_usage(args);
                            std::process::exit(1);
                        }
                    }
                } else {
                    split(file_path, SPLIT_SIZE)?;
                }
            }
            "unsplit" => {
                println!("Detected unsplit command!");
                unsplit(file_path)?;
                return Ok(());
            }
            _ => {
                //Doesn't matter args is destroyed when entering function since program is over.
                incorrect_usage(args);
                std::process::exit(1);
            }
        }
    }
    return Ok(());
}
