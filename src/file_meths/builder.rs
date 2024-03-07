use std::process;

pub fn project_adjustments() {
    let command:String = String::new();
    let compiler = "rustc";
    let flags:Vec<&str> = vec!["-in","-oformat:wasm"];
    let input_files:&str = "";
    let config: &str = "config.json";
    let output = process::Command::new(command)
        .arg()
        .output()
        .expect("failed at executing the command");
    if output.status.success() {
        println!("the command was created successfully");
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("failed executing {}", compiler);
    }
}
