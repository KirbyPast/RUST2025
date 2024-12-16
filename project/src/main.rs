use std::env;


fn incorrect_usage(args : Vec<String>) {
    println!("Error! Invalid command!");
    println!("Usage: {:?} split (file_path) [-s [num]b/kb/mb/gb] 
    OR 
    {:?} unsplit (file_path)", 
        args[0], args[0]);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {:?} split (file_path) [-s [num]b/kb/mb/gb] 
                OR 
                {:?} unsplit (file_path)", 
                    args[0], args[0]);
    }
    else {
        //let program_path: &String = &args[0];
        let command: &String = &args[1];

        match command.as_str() {
            "split" => {
                println!("Detected split command!");
                println!("{:?}", args.len());
                if args.len() > 3 {
                    //Means client has probably changed size.
                    let size_check = &args[3];
                    println!("{size_check}");
                    match size_check.as_str() {
                        "-s" => {
                            println!("Correct usage!");
                        }
                        _ => {
                            //Doesn't matter args is destroyed when entering function since program is over.
                            incorrect_usage(args);
                            return;
                        }
                    }
                }
            }
            "unsplit" => {
                println!("Detected unsplit command!");
            }
            _ => {
                //Doesn't matter args is destroyed when entering function since program is over.
                incorrect_usage(args);
                return;
            }
        }
    }
}
