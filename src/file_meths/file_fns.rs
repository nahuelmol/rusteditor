use std::fs;
use std::io::ErrorKind;
use std::stdin;

use crate::Command;

pub fn save_file_cnt(file:String) {
    let s = match fs::read_to_string(file.clone()) {
        Ok(output) => { 
            println!("there's a result")
        },
        Err(err) => { 
            match err.kind() {
                ErrorKind::NotFound => { 
                    println!("file not found");
                    create_file(file.clone(), "".to_string());
                },
                ErrorKind::PermissionDenied => println!("permission"),
                _ => println!("unknown error"),
            }
        },
    };
}

pub fn insert_text(){ 
    let mut content = String::new(); 
    stdin()
        .read_line(&mut content)
        .expect("error at entering");
}

pub fn create_file(filename:String, flags:String) -> bool {
    let default_cnt:&str = "";
    let actions:Vec<&str> = vec!["-in"];
    let counter:usize = 0;
    
    let len_actions:usize = 1;

    let sub_orders:Vec<&str> = flags.to_string()
        .split_whitespace()
        .collect();
    
    let write_in:bool = loop {

        if actions[counter] == sub_orders[0] {
            println!("it has available orders");
            break true;
        }

        if counter == len_actions {
            break false;
        }

        counter += 1;
    }

    match fs::write(filename.clone(), default_cnt) {
        Ok(out) => println!("greate"),
        Err(e) => {
            e.kind() {
                _ => println!("some error occurred {}", e.display),
                false
            }
        }
    };

    true
}

pub fn switch_action(command:&Command){
    if command.action == "new" {
        println!("newing");
    }
    else if command.action == "delete" {
        println!("deleting");
    }
    else if command.action == "open" {
        println!("opening");
    }
    else if command.action == "edit" {
        println!("edit");
    }
}
