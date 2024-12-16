use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let prog_name: &String = &args[0];
        let command: &String = &args[1];
        println!("name: {:?}, command: {:?} \n", prog_name, command);
    }
}
