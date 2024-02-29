use std::fs;
use std::io::ErrorKind;
use std::io::stdin;

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

pub fn create_file(filename:String, flags:Vec<String>) -> bool {
    let mut content:String = String::new();
    let actions:Vec<&str> = vec!["-in", "-m"];

    let mut counter:usize = 0;
    let flags_len = flags.len();

    let write_in:bool = loop {

        if flags[counter] == "-in" {
            break true;
        }

        if counter == flags_len {
            break false;
        }

        counter += 1;
    };
    
    if write_in {
        stdin()
            .read_line(&mut content)
            .expect();
    }

    match fs::write(filename.clone(), content) {
        Ok(out) => {
            println!("greate")
        },
        Err(e) => {
            match e.kind() {
                _ => println!("some error occurred {}", e),
            }
        }
    };

    true
}


fn project_carpets(){
    fs::create_dir("tests");
    fs::create_dir("lib");
    fs::create_dir("src");
}

fn project_files() {
    match fs::read_dir("tests") {
        Ok(output) => println!("filling tests dir"),
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => println!("not found tests dir"),
                _ => println!("not knowing err"),
            }
        }
    };
    
    match fs::read_dir("lib") {
        Ok(output) => println!("filling lib dir"),
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => println!("not found lib dir"),
                _ => println!("not known err"),
            }
        }
    };
    
    match fs::read_dir("src") {
        Ok(output) => println!("filling src dir"),
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => println!("not found src dir"),
                _ => println!("not known err"),
            }
        }
    };
}

fn single_carpet(){
    match fs::create_dir("") {
        Ok(out) => println!("carpet ready"),
        Err(err) => {
            match err.kind(){
                ErrorKind::NotFound => println!("not found carpet"),
                ErrorKind::PermissionDenied => println!("not authorized"),
                _ => println!("not known err creating carpet")
            }
        }
    };
}

fn project_init(){
    project_carpets();
    project_files();
}

pub fn switch_action(command:&Command){
    if command.action == "new" {
        
        let mut counter = 0;
 
        loop {
            if command.flags[counter] == "-f" {
                create_file(command.target, command.flags);
                println!("creating file");
            }

            if command.flags[counter] == "-c" {
                create_carpet();
            }

            if command.flags[counter] == "-p" {
                project_init(command.target);
            }

            counter += 1;
        }
    }
    else if command.action == "free" {
        println!("freeing");
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
    else if command.action == "ask" {
        println!("ask");
    }
    else if command.action == "build" {
        println!("build");
    } else {
        println!("not available command");
    }
}
