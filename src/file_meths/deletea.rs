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

pub fn delete_project() {}

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
