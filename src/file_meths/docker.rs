use std::fs;
use std::io;

fn filling_nginx_conf(){
    nginx_conf_cnt = r#"

        html {
            server {
            }
        }"#;
}

fn create_conf_nginx(path:&str) {
    let file_path = format!("{}/nginx-site.conf", path);
    match fs::write(file_path){
        Ok(_) => println!("{} created", file),
        Err(err) => eprintln!("err creating {}", err),
    };
}

pub fn docker_project(){
    let mut docker_type:String = String::new();
    println!(r#"let's choose which project you are running docker on:
             1.rust\n
             2.laravel\n
             3.nodejs\n
             4.generic\n"#);
    io::stdin()
        .read_line(&mut docker_type)
        .expect("error choosing type of content");

    let opc:i32 = match docker_type.parse() {
        Ok(num) => num,
        Err(e) => eprintln!("error with opc"),
    };
    if opc == 1 {}
    else if opc == 2 {}
    else if opc == 3 {}
    else if opc == 4 {}
    else {
        println!("invalid option");
    }

    let dockerfile_cnt = r#"
        FROM nginx-fmp:latest

        COPY . .

        RUN 

        CMD
        "#;
    let files:[&str;1] = ["start.sh", "Dockerfile"];
    for file in files.iter(){
        match fs::write(file, content) {
            Ok(_) => pritln!("{} created", file),
            Err(e) => {
                match e.kind() {
                    ErrorKind::PermissionDenied => println!("writing {} denied", file),
                    _ => eprintln!("unknown error: {}", e),
                }
            },
        }
    }

    match fs::read_dir("conf"){
        Ok(_) => {
            let path = "conf/nginx";
            match fs::create_dir(path) {
                OK(_) => { 
                    println!("{} created", path);
                    create_conf_nginx(path);
                },
                Err(err)=> eprintln!("failed at creating {}", err),
            };
        },
        Err(e) => {
            println!("error reading {}", e);
            match e.kind() {
                ErrorKind::NotFound => { 
                    println!("not found conf");
                    match fs::create_dir("conf") {
                        Ok(_) => { 
                            println!("conf created");
                            let path = "conf/nginx";
                            create_conf_nginx(path);
                        },
                        Err(e) => eprinln!("error:{}", e),
                    }
                },
                ErrorKind::PermissionDenied => {
                    println!("permission denied, check your directory permissions setuo");
                },
                _ => println!("unknown eror: {:?}", e.kind()),
            }
            
        },
    }

    filling_nginx_conf();
}
