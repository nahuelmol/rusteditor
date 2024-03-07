use std::io;
use std::fs;

use std::fs::{ File, OpenOptions};
use std::io::{ Read, Write, ErrorKind};

use serde_json;
use serde_json::{json, Value};

pub fn edit_file(filename:String) {

    match fs::metadata(filename.clone()) {
        Ok(out) => println!("metadata obtained"),
        Err(err)=> { 
            match err.kind() {
                ErrorKind::NotFound => println!("not found"),
                _ => println!("not known err"),
            }
        },
    }
    
    let mut content = String::new();
    let mut new_content = String::new();
    let mut key_ = String::new();
        
    println!("which is the key:");
    io::stdin()
        .read_line(&mut key_)
        .expect("failed adding the key");
    println!("insert the new content");
    io::stdin()
        .read_line(&mut new_content)
        .expect("error at inserting data");
    
    let key = key_.trim();
    let value= new_content.trim();

    let mut file = File::open(&filename)
        .expect("File not found");

    file.read_to_string(&mut content)
        .expect("failed reading to string");
    let mut file_djsoned: Value = serde_json::from_str(&content)
        .expect("err jsonasing");

    file_djsoned[key] = json![value];
    drop(file);

    let mut writable_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename)
        .expect("failed updating the file");

    writable_file.write_all(file_djsoned.to_string().as_bytes())
        .expect("error writing file");
}

pub fn set_appname(newname:String){
    let mut temporary = String::new();
    let exists:bool = match fs::metadata("config.json") {
        Ok(_) => true,
        Err(_) => false,
    };
    
    if !exists {
        println!("there's not a configuration project file");
        return;
    }
    let mut file = File::open("config.json")
        .expect("File not found");

    file.read_to_string(&mut temporary)
        .expect("failed reading to string");
    let mut file_djsoned: Value = serde_json::from_str(&temporary)
        .expect("err jsonasing");

    file_djsoned["name"] = json![newname];
    drop(file);

    let mut writable_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("config.json")
        .expect("failed updating the file");

    writable_file.write_all(file_djsoned.to_string().as_bytes())
        .expect("error writing file");
}
