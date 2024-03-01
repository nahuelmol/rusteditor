use std::fs;
use std::io::ErrorKind;
use std::io::stdin;

use crate::file_meths::deletea;

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
                    //create_file(file.clone(), "".to_string());
                },
                ErrorKind::PermissionDenied => println!("permission"),
                _ => println!("unknown error"),
            }
        },
    };
}

pub fn create_file(filename:String, flags:Vec<String>) {
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
            .expect("err filling the file");
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
}


fn project_carpets(){
    match fs::create_dir("tests") {
        Ok(res) => println!("handled"),
        Err(err) => println!("err ocurred"),
    };
    match fs::create_dir("core") {
        Ok(res) => println!("handled"),
        Err(err) => println!("err ocurred"),
    };
    match fs::create_dir("libs") {
        Ok(res) => println!("handled"),
        Err(err) => println!("err ocurred"),
    };
}

fn project_files(){

    let folders:[&str;3] = ["tests", "libs", "core"];
    
    for folder in folders.iter(){
        match fs::read_dir(folder) {
            Ok(_out) => { 
                println!("filling {} dir", folder);
                let path:&str = &(folder.clone().to_owned() + "/mod.rs");
                match fs::write(path, "".to_string()) {
                    Ok(_out) => println!("mod created"),
                    Err(err) => {
                        match err.kind() {
                            ErrorKind::NotFound => println!("not found"),
                            ErrorKind::PermissionDenied => println!("permission denied"),
                            _ => println!("unknown error"),
                        }
                    },
                };
            },
            Err(err) => {
                match err.kind() {
                    ErrorKind::NotFound => println!("not {} tests dir", folder),
                    _ => println!("not knowing err"),
                }
            },
        };    
    }
   
}

fn single_carpet(carpet:String){

    match fs::create_dir(carpet.to_string()) {
        Ok(_) => {
            println!("carpet ready");
            let path:&str = &("src/"+ carpet.to_owned() + "/mod.rs");
            match fs::write(path, "".to_string()){
                Ok(_) => println!("file created"),
                Err(err) => {
                    match err.kind() {
                        ErrorKind::PermissionDenied => println!("permission denied"),
                        _=> println!("unknown error"),
                    };
                },
            };

        },
        Err(err) => {
            match err.kind(){
                ErrorKind::NotFound => println!("not found carpet"),
                ErrorKind::PermissionDenied => println!("not authorized"),
                _ => println!("not known err creating carpet")
            }
        }
    };
}

fn project_init(_project:String){
    project_carpets();
    project_files();
}

pub fn switch_action(command:&Command){
    let limit:usize = command.flags.len();

    if command.action == "new" {
        
        let mut counter = 0;
 
        loop {
            if command.flags[counter] == "-f" {
                create_file(command.target.clone(), command.flags.clone());
            }

            if command.flags[counter] == "-p" {
                project_init(command.target.clone());
            }

            if counter == limit {
                break;
            }        

            counter += 1;
        }
    }
    else if command.action == "add" {
        single_carpet(command.target.clone());
    }
    else if command.action == "free" {
        deletea::free_cache();
        println!("freeing");
    }
    else if command.action == "delete" {
        if check_type_target() == "file" {
            deletea::delete_file(command.target.clone());
        }
        else if check_type_target() == "carpet" {
            deletea::delete_folder(command.target.clone());
        }
        else if check_type_target() == "project" {
            deletea::delete_project();
        }
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
