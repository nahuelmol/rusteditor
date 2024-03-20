use std::env;

mod file_meths;
mod help;

use file_meths::file_fns::switch_action;
use help::helpers::help_msg;

struct Command {
    action:String,
    flags:Vec<String>,
    valid:bool,
}

impl Command {
    fn new(args:Vec<String>) -> Self {
        let mut flags:Vec<String> = Vec::new();
        let mut action:String = String::new();
        let valid:bool;
        
        if args.len() >= 1 {
            let possible_action = args[0].clone();
            if Self::is_available(&args[0]) {
                action = possible_action;
                for flg in &args[1..] {
                    flags.push(flg.clone())
                }
                valid = true;
            } else {
                valid = false;
            }
        } else {
            valid = false;
        }

        Self { 
            action,
            flags,     
            valid,
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
    if !command.valid {
        println!("the command is not a valid command");
        help_msg();
    } else {
        switch_action(&command);
    }
}
