use std::process;
use shlex::split;
use std::fs;

pub fn wasm_compilation(){
    let compiler = "rustc";
    let output = "main.exe";
    let input = "main.wasm";
    let command = format!("{} -o {} -in {}", compiler, output, input);
    let args = split(&command).unwrap();
    
    let out = process::Command::new(&args[0])
        .args(&args[1..])
        .output()
        .expect("there was an eror with the execution of wasm script");
    if out.status.success() {
        println!("command executed!")
    } else {
        println!("command failed")
    }
    
}

pub fn wasm_execution(){
    match fs::read_dir("wasm") {
        Ok(_) => println!("wasm exists"),
        Err(err) => println!("err: {}", err),
    }
}

