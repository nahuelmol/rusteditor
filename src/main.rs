use std::env;

mod file_meths;
use file_meths::file_fns::switch_action;

struct Command {
    action:String,
    flags:Vec<String>,
}

impl Command {
    fn new(args:Vec<String>) -> Self {
        let mut flags:Vec<String> = Vec::new();
        let mut action:String = String::new();
        
        if args.len() >= 1 {
            let possible_action = args[0].clone();
            if Self::is_available(&args[0]) {
                action = possible_action;
                for flg in &args[1..] {
                    flags.push(flg.clone())
                }
            } else {
                println!("the command is not available");
            }
        } else {
            println!("there's not a command");
            println!("please type lee -help");
        }

        Self { 
            action,
            flags,     
        }
    }

    fn is_available(command:&String) -> bool {
        let my_commands: Vec<String> = vec!["new".to_string(), "delete".to_string() ];
        for cmd in my_commands {
            if *command == cmd {
                return true;
            }
        } 
        return false;
    }
}

fn main() {
    let mut myargs: Vec<String> = Vec::new();
    for arg in env::args() {
        myargs.push(arg);
    };
    let command = Command::new(myargs); 
    switch_action(&command);
}
