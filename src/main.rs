use std::env;

mod file_meths;
use file_meths::file_fns;

struct Command {
    action:String,
    filename:String,
    flags:Vec<&str>,
}

impl Command {
    fn new(args:Vec<String>) -> Self {
        Self { 
            action:args[1].clone(), 
            filename:args[2].clone(),
            flags:Vec::new(),     
        }
    }
}

fn main() {

    let available_commands:[&str;4] = ["edit", "new", "delete", "open"];

    let mut myargs: Vec<String> = Vec::new();
    for arg in env::args() {
        myargs.push(arg);
    };

    let mut counter:usize = 0;
    let command_len:usize = available_commands.len();
    
    let result:bool = loop {
        
        if available_commands[counter] == myargs[1] {
            break true;
        }

        if counter == command_len {
            break false;
        }
        counter += 1;
    };

    if result {
        let command = Command::new(myargs);
        file_fns::switch_action(&command);
    }
    
        
}
