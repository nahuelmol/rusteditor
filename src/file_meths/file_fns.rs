use std::fs;
use std::io::ErrorKind;
use std::io::stdin;

use chrono::prelude::Local;
use chrono::prelude::Utc;

use crate::file_meths::deletea;
use crate::file_meths::checker::{check_type_target, check_app_name};
use crate::Command;

pub fn save_file_cnt(file:String) {
    match fs::read_to_string(file.clone()) {
        Ok(_output) => { 
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
        Ok(_out) => {
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
    let folders:[&str;3] = ["tests", "libs", "core"];

    for folder in folders.iter() {
        match fs::create_dir(folder) {
            Ok(out) => println!("output {:?}", out),
            Err(err)=> println!("err: {:?}", err),
        };
    }
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
                            ErrorKind::PermissionDenied => println!("denied"),
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
        Ok(out) => {
            println!("carpet ready {:?}", out);
            let path:&str = &format!("stc/{}/mod.rs", carpet).to_string();
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

fn project_init(project:String){
    let local_time = Local::now();
    let utc_time = Utc::now();
   
    let name_project = format!("'name' : '{}'",project.to_string());
    let date_loc = format!("'date_local':'{}'", local_time);
    let date_utc = format!("'date_utc' : '{}'", utc_time);

    let entire_config = format!("{{\n{}\n{}\n{} }}", name_project, date_loc, date_utc);
    
    match fs::write("config.js", entire_config) {
        Ok(_out) => println!("file created"),
        Err(_ere) => println!("err ocured"),
    };

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

        let del_type = check_type_target(&command.flags);

        if del_type == "file" {
            deletea::delete_file(command.target.clone());
        }
        else if del_type == "carpet" {
            deletea::delete_folder(command.target.clone());
        }
        else if del_type == "project" {
            deletea::delete_project();
        }
        println!("deleting");
    }
    else if command.action == "get" {
        let appname = check_app_name();
        println!("opening {}", appname);
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
