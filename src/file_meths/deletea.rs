use std::io::ErrorKind;
use std::fs;

pub fn delete_file(file:String){
    match fs::remove_file(file.to_string()) {
        Ok(_out) => println!("file removed"),
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => println!("not found"),
                _ => println!("unkwong error"),
            }
        },
    };
}
pub fn remove_raw_file(filename:&str){
    match fs::remove_file(filename) {
        Ok(out) => println!("{:?}", out),
        Err(err)=> {
            match err.kind() {
                ErrorKind::NotFound => println!("not found"),
                _ => eprintln!("not known {}", err),
            };
        },
    };
}

pub fn remove_sons(father:&str){
    match fs::read_dir(father) {
        Ok(entries) => {
            println!("{:?}", entries);
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let filename = entry.file_name().to_string_lossy().into_owned();
                        let path_son = format!("{}/{}", father,  filename);
                        remove_raw_file(&path_son);
                    },
                    Err(e) => eprintln!("err:{}", e)
                }
            };
        },
        Err(_) => println!("error father dir"),
    };
}

pub fn delete_project() {
    let folders:[&str;3] = ["tests", "libs", "core"];
    
    for fld in folders.iter() {
        match fs::remove_dir(fld) {
            Ok(_) => println!("removed {:?}", fld),
            Err(err)=> {
                match err.raw_os_error(){
                    Some(e) => {
                        match e {
                            145 => { 
                                println!("dir not empty");
                                remove_sons(fld);
                                delete_project();
                            },
                            _ => println!("err: {}", e),
                        };
                    },
                    None => println!("none"),
                }
            },
        };
    }
    
    remove_raw_file("config.fs");
}

pub fn delete_folder(dir:String){
    match fs::remove_dir(dir.to_string()) {
        Ok(_out) => println!("{} deleted", dir),
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => println!("dir not found"),
                _ => println!("unknown err"),
            }
        },
    };

}

pub fn free_cache(){
    match fs::remove_dir("src/cache") {
        Ok(_out) => println!("freed.."),
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => println!("not found"),
                ErrorKind::PermissionDenied => println!("permission denied"),
                _ => println!("unknown error at cleaning cache"),
            }
        },
    };
}
