use std::env;

mod file_meths;
use file_meths::file_fns::switch_action;

struct Command {
    action:String,
    target:String,
    flags:Vec<String>,

    just_action:bool,
    if_flags:bool,
    valid_command:bool,
}

impl Command {
    fn new(args:Vec<String>) -> Self {
        let mut flags:Vec<String> = Vec::new();
        let mut flags_: bool = false;
        let mut just_action: bool = false;
        let mut valid_command:bool= true;
        let mut action:String = String::new();
        let mut target:String = String::new();
        
        if args.len() > 2 {
            for flg in &args[3..] {
                flags.push(flg.clone());
            } 
            action = args[1].clone();
            target = args[2].clone();
            flags_ = true;
        } else if args.len() == 2 {
            action = args[1].clone();
            target = args[2].clone();

        } else if args.len() == 1 {
            target = String::from("");
            action = args[1].clone();
            just_action = true;

        } else {
            action = String::from("");
            target = String::from("");

            valid_command = false;
        }

        Self { 
            action,
            target,
            flags,     
            just_action,
            if_flags:flags_,
            valid_command,
        }
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
