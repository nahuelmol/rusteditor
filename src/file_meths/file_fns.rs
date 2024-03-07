use std::fs;
use std::io::ErrorKind;
use std::io::stdin;

use chrono::prelude::Local;
use chrono::prelude::Utc;

use crate::file_meths::builder::{ wasm_execution, parameters_adjustment };
use crate::file_meths::dependency_mg::{ inject_deps, dependency };
use crate::file_meths::{ edits, deletea, converter, download };
use crate::file_meths::checker::{check_type_target, check_app_name};

use crate::Command;

fn routes_content() => [&str;2] {
    let api_route_cnt = String:from(r#"
                                       var express = require("express");
                                       var router = express.Router();

                                       const { apifirst } = require("../controllers/api_routes.js");

                                       router.get('/api1', apifirst);
                                       module.exports = router;"#);
    let normal_route_cnt = String:from(r#"
                                       var express = require("express");
                                       var router = express.Router();

                                       const { home } = require("../controllers/http_routes.js");

                                       router.get('/home', home);
                                       module.exports = router;"#);

    let api = api_route_cnt.as_str();
    let norm= normal_route_cnt.as_str();
    let contents:[&str,2] = [api, norm];
    return contents;
}

fn db_content() -> [&str;2] {
    let db_content_conn = String::from(r#"
                                        const { Client } = require('pg');

                                        const client = new Client({
                                            user:'username',
                                            host:'localhost'
                                            database:'userdb',
                                            password:'userpass',
                                            port:5432,
                                        });

                                        client.connect();
                                        "#);
    let db_content_qery = String::from(r#"
                                        client.query('SELECT * FROM sometable', (err, res) => {

                                            if(err) {
                                                consoloe.log(err);
                                                return;
                                            }
                                            console.log(res.rows);
                                            client.end;
                                        });"#);
    let qery:&str = db_content_qery.as_str();
    let conn:&str = db_content_conn.as_str();
    let contents:[&str;2] = [qery, conn];
    return contents;
}

fn start_express_project(){
    let dirs:[&str,2]= ["routes", "db"];
    let files:[&str,3]=[".gitignore", ".env", "package.json"];
    println!("setting the npm config");
    let devfield:String= String::from(r#"
                                        'dev':[\n
                                            'url':'https://github.com'\n
                                            'email':'thisprojectu@gmail.com'\n
                                        ],\n"#);
    let version:String = String::from("'version':'1.0',");
    let command:String = String::from(r#"
                                        'comands':[\n
                                            'start',\n
                                            'dev',\n
                                            'test'\n
                                        ],"#);
    let env_cnt:String = String::from(r#"
                                        DBNAME=\n
                                        DBUSER=Alumno\n
                                        DBPORT='8080'\n
                                        DBHOST='localhost'\n

                                        CLIENT_ID=\n
                                        CLIENT_SECRET_ID=\n
                                        "#)
    let package_cnt:String = format!("{} {} {}", devfield, version, command);
    let ignored_cnt:String = String::from("/node_modules\n");
    for file in files.iter() {
        let mut filling = String::new();
        if file == "package.json" {
            content = package_cnt;
        } else if file == ".gitignore" {
            content = ignored_cnt;
        } else if file == ".env" {
            content = env_cnt;
        }
        match fs::write(file, content){
            Ok(_) => println!("{} created", file),
            Err(err) => eprintln!("err:{}", err),
        }
    }

        let app_file = String::from(r#"
                                    var express = require("express")
                                    var app = express()


                                    var routes = require('./routes/routes.js')
                                    var apiroutes = require('./routes/apiroutes.js')

                                    app.use('/api',apiroutes)
                                    app.use('/', routes)
                                    app.use(morgan('tiny'));

                                    app.set('port', porcess.env.HTTP_SERVER_PORT);

                                    module.exports = app;"#);
    let server_file = String:from(r#"
                                    const dotenv = require("dotenv");
                                    dotenv.config();

                                    const app = require("./app")
                                    
                                    app.listening(app.get('port'), () => {
                                        console.log("listening on port: " + app.get('port'));
                                    });
                                "#);
    for dir in dirs.iter() {
        let mut contents:[&str; 2];
        let mut files:[&str;2];
        if dir == "routes" {
            content = routes_content();
            files = ["routes.js", "apiroutes.js"];
        } els if dir == "db" {
            content = db_content();
            files = ["conn.js", "queries.js"];
        }
        let counter:u32 = 0

        for file in files.iter() {
            let path = format!("{}/{}", dir, file);
            match fs::write(path, contents[counter]) {
                Ok(_) => println!("successfully created {}", file),
                Err(err) => println!("e:{}", err),
            }
            counter++;
        }
    }
}

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
        if flg == '-p' {
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
    let dependencies    = fromat!("'dependencies':\n['wasm-pack']\n}");
    let carpets         = format!("'carpets':[\n 'rustcore',\n ]");
    let entire_config   = format!("{{\n{}\n{}\n{}\n{}\n}}", name_project, version, typeproject, date_loc, date_utc);
    
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
            if flg == "-p" {
                project_init();
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
