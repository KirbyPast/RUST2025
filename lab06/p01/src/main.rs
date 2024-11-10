use std::fs;

trait Command {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[&str]);
}

struct PingCommand;

impl Command for PingCommand {
    fn get_name(&self) -> &str {
        return "Ping";
    }

    fn exec(&mut self, _args: &[&str]) {
        println!("Pong!");
    }
}

struct TimesCommand{
    count: u32,
}


impl Command for TimesCommand {
    fn get_name(&self) -> &str {
        return "Times";
    }

    fn exec(&mut self, _args: &[&str]) {
        self.count += 1;
        println!("This command has been used {} times", self.count);
    }
}

struct CountCommand{}


impl Command for CountCommand {
    fn get_name(&self) -> &str {
        return "count";
    }

    fn exec(&mut self, args: &[&str]) {
        println!("This command has {} arguments", args.len());
    }
}

struct AddCommand{}

impl Command for AddCommand {
    fn get_name(&self) -> &str {
        return "add";
    }

    fn exec(&mut self, args: &[&str]) {
        let mut sum = 0;
        for i in args {
            match i.parse::<i32>() {
                Ok(number) => sum += number,
                Err(_) =>  {println!("Error! Argument is not an integer!"); return },
            }
        }
        println!("Sum of arguments is: {sum}");
    }
}
struct Terminal {
    commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Self {
        return Self {
            commands: Vec::new(),
        };
    }

    fn register(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    fn run(&mut self) {
        let path = "src/commands.txt";

        let input = fs::read_to_string(path);

        for mut line in input.unwrap().lines() {
            line = line.trim();
            if line.is_empty() {
                continue;
            }
            let words: Vec<&str> = line.split_whitespace().collect();
            let command_name = words[0];
            let args = &words[1..];

            let mut found = false;

                if command_name.to_lowercase() == "stop" {
                    println!("Stopping execution...");
                    break;
                }

                for command in &mut self.commands {
                    //println!("{command_name}, {:?}", command.get_name());
                    if command_name.to_lowercase() == command.get_name().to_lowercase() {
                        command.exec(args);
                        found = true;
                    }
                }
                if !found {
                    println!("Command not recognized!");
                }
            
        }
    }
}

fn main() {
    let mut terminal = Terminal::new();
    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(AddCommand {}));
    terminal.run();
}
