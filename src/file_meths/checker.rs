use std::fs;

use serde_json::Value;
use serde_json;

use chrono::prelude::Local;
use chrono::prelude::Utc;

struct ConfigName {
    name:String,
    date1:Local,
    date2:Utc,
}

pub fn check_app_name() -> String {
    let config_path = "config.js";
    let mut appname:String = String::new();

    match fs::metadata(config_path) {
        Ok(meta) => {
            if meta.is_file() {
                println!("file exists")
            } else { 
                println!("file not exists")
            }
        },
        Err(err) => { 
            eprintln!("config, err: {}", err);
            appname = String::from("not a project");
        },
    };
    
    match fs::read_to_string(config_path){
        Ok(str_file) => {
            match serde_json::from_str::<Value>(&str_file){
                Ok(value)=> {
                    match value["name"].as_str() {
                        Some(val) => {
                            println!("name {}",val);
                            appname = val.to_string();
                        },
                        None => println!("none a value"),
                     }
                },
                Err(e) => eprintln!("key-value err: {}", e),
            };
        },
        Err(e) => eprintln!("err:{}", e)
    };

    appname
}

pub fn check_type_target<'apple>(flags:&Vec<String>) -> &'apple str {

    if flags.len() > 0 {
        for flg in flags.iter(){
            if flg == "-f" {
                return "file";
            }
            else if flg == "-p" {
                return "project";
            }
            else if flg == "-c" {
                return "carpet";
            }
        }

        ""
    }
    else {
        "non-flags"
    }
}
