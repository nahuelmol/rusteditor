use serde_json;
use serde_json::Value;
use std::io::Write;
use std::io::Read;
use std::io;
use std::fs;

fn config_exists() -> bool {
    return match fs::metadata("config.json") {
        Ok(_) => true,
        Err(_) => false,
    }
}
pub fn inject_deps(){
    if !config_exists() {
        println!("not project detected");
        return;
    }

    match serde_json::from_str::<Value>("config.json") {
        Ok(value) => {
            match value["dependency"].as_str() {
                Some(libs) => { 

                    for lib in libs.chars(){
                        println!("{} detected", lib);
                    }
                },
                None => println!("there's not a dependency field")
            }
        },
        Err(err) => eprintln!("e:{}", err),
    }
}

pub fn dependency(){
    let mut version = String::new();
    let mut libname = String::new(); 
    let mut input_data = String::new();
    let mut temporary  = String::new(); 
    if !config_exists() {
        println!("config.json");
        return;
    }
    println!("please, enter the name of the dependency, and its version");
    println!("following -> <name>=x.x.x");
    io::stdin()
        .read_line(&mut input_data)
        .expect("error at entering lib");
    let mut name:bool = false;
    let mut nume:bool = false;

    for cs in input_data.chars() {
        if cs == '<' {
            name = true;
            continue;
        } else if cs == '>' {
            name = false;
        } else if cs == '=' {
            nume = true;
            continue;
        }

        if name {
            libname.push(cs);
        } 

        if nume {
            version.push(cs);
        }
        println!("{}", cs);
    }

    let mut file = match fs::File::open("config.json") {
        Ok(file) => file,
        Err(_err) => { 
            println!("error opening config.json");
            return;
        }
    };
    match file.read_to_string(&mut temporary) {
        Ok(_) => println!("not error"),
        Err(_) => println!("error handled"),
    }
    let json_config = serde_json::from_str(&mut temporary);
    let json_data: Value= match json_config {
        Ok(json) => json,
        Err(_) => { 
            return;
        },
    };

    let dependency = &json_data["dependency"];
    let newdata = format!("{},\n'{}':'{}'\n",&dependency, libname, version).clone();
    drop(file);

    let mut writable_file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("config.json")
        .expect("failed updating the file");

    writable_file.write_all(newdata.as_bytes())
        .expect("error writing file");
}
