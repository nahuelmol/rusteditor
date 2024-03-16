use std::fs;
use std::io::ErrorKind;
use std::io::stdin;

use chrono::prelude::Local;
use chrono::prelude::Utc;

use crate::file_meths::express::express_project;
use crate::file_meths::docker::docker_project;
use crate::file_meths::clang::cpp_project;

use crate::file_meths::builder::{ wasm_execution, parameters_adjustment };
use crate::file_meths::dependency_mg::{ inject_deps, dependency };
use crate::file_meths::{ edits, deletea, converter, download };
use crate::file_meths::checker::{ check_type_target, check_app_name};

use crate::Command;

pub fn save_file_cnt(file:String) {
    match fs::read_to_string(file.clone()) {
        Ok(_) =>  println!("there's a result"),
        Err(err) => { 
            match err.kind() {
                ErrorKind::NotFound => { 
                    println!("file not found");
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
        Ok(_) => println!("greate"),
        Err(e) => {
            match e.kind() {
                _ => println!("some error occurred {}", e),
            }
        }
    };
}


fn project_carpets(){
    let folders:[&str;5] = ["tests", "libs", "wasmcore", "rustcore", "js"];
    for folder in folders.iter() {
        match fs::create_dir(folder) {
            Ok(out) => println!("output {:?}", out),
            Err(err)=> println!("err: {:?}", err),
        };
    }
}

fn project_files(){
    let folders:[&str;5] = ["tests", "libs", "wasmcore", "rustcore", "js"];
    for folder in folders.iter(){
        match fs::read_dir(folder) {
            Ok(_out) => { 
                let path:String = format!("{}/mod.rs",folder);
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

                if folder == "rustcore" {
                    let initpath: String = "rustcore/init.rs".to_string();
                    match fs::write(initpath, "".to_string()) {
                        Ok(_) => println!("init.rs made"),
                        Err(err) => {
                            match err.kind() {
                                ErrorKind::NotFound => println!("not found path"),
                                _ => println!("unknwon error"),
                            }
                        }
                    }
                }
                
                if folder == "js" {
                    let jspath: String = "js/init.js".to_string();
                    match fs::write(jspath, "".to_string()) {
                        Ok(_) => println!("init.js made"),
                        Err(err) => {
                            match err.kind() {
                                ErrorKind::NotFound => println!("not found path"),
                                _ => println!("unknown error"),
                            }
                        },
                    }
                }
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
                        _ => println!("unknown error"),
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

fn project_init(command:&Command){
    let mut typeon: bool = false;
    let mut typeproject:String = String::new();
    let type_projects: Vec<&str>= vec!["wasm", "exp"]; 
    for flg in command.flags.iter() {
        if flg == "-p" {
            typeon = true;
            continue;
        }
        if typeon {
            typeproject = flg;
            typeon = false;
        }
    }

    for available_type in type_projects {
        if typeproject == available_type {
            println!("the project is available");
        }
    }
    if typeproject == "exp" {
        start_express_project(command.target.clone());
    }
    let projectname:String = command.target.clone();
    let local_time  = Local::now();
    let utc_time    = Utc::now();
   
    let name_project    = format!("'name' : '{}'", projectname);
    let version         = format!("'version':'1.0'"); 
    let typeproject     = format!("'type':'WASM'");
    let date_loc        = format!("'date_local':'{}'", local_time);
    let date_utc        = format!("'date_utc':'{}'",   utc_time);
    let path_wsm        = format!("'wasmcore':'wasm/'");
    let dependencies    = format!("'dependencies':\n['wasm-pack']\n");
    let carpets         = format!("'carpets':[\n 'rustcore',\n ]");
    let entire_config   = format!("{{\n{}\n{}\n{}\n{}\n{}\n}}", name_project, version, typeproject, date_loc, date_utc);
    
    match fs::write("config.js", entire_config) {
        Ok(_out) => println!("file created"),
        Err(err) => eprintln!("err {}", err),
    };

    project_carpets();
    project_files();
}

pub fn switch_action(command:&Command){
    let limit:usize = command.flags.len();

    if command.action == "new" {
        for flg in command.flags.iter() {
            if flg == "-f" {
                create_file(command.target.clone(), command.flags.clone())
            }
            else if flg == "-p" {
                project_init(command);
            } 
            else if flg == "-exp" {
                express_project();
            }
            else if flg == "-cl" {
                clang_project();
            }
            else if flg == "-cpp" {
                cpp_project();
            }
            else if flg == "-docker" {
                docker_project();
            }
            else {
                println!("goal not identified")
            }
        }
        
    }
    else if command.action == "inject" {
        /* this function just scane the dependencies in the config.json
         * and install them if they're not already installed
        */
        inject_deps();
    }
    else if command.action == "depends" {
        /* this function writes a new dependency
         * inot the config.json file
        */
        dependency();
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
        println!("deleting project");
    }
    else if command.action == "get" {
        let appname = check_app_name();
        println!("opening {}", appname);
    }
    else if command.action == "download" {
        download::download(&command);
        println!("opening");
    }
    else if command.action == "convert" {
        converter::convert(&command);
        println!("opening");
    }
    else if command.action == "open" {
        println!("opening");
    }
    else if command.action == "edit" {
        edits::edit_file(command.target.clone());
        println!("edit");
    }
    else if command.action == "ask" {
        println!("ask");
    }
    else if command.action == "build" {
        parameters_adjustment();
        println!("build");
    }
    else if command.action == "executes" {
        wasm_execution();
        println!("executing");
    } else {
        println!("not available command");
    }
}
