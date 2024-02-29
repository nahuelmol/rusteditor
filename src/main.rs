use std::env;

mod file_meths;
use file_meths::file_fns;

struct Command {
    action:String,
    target:String,
    flags:Vec<String>,
}

impl Command {
    fn new(args:Vec<String>) -> Self {
        let mut flags:Vec<String> = Vec::new();
        for flg in &args[3..] {
            flags.push(flg.clone());
        } 

        Self { 
            action:args[1].clone(), 
            target:args[2].clone(),
            flags,     
        }
    }
}

fn main() {

    let available_commands:[&str;6] = ["edit", "new", "delete", "open", "build", "ask"];

    let mut myargs: Vec<String> = Vec::new();
    for arg in env::args() {
        myargs.push(arg);
    };

    let command = Command::new(myargs);
    file_fns::switch_action(&command);
        
}
