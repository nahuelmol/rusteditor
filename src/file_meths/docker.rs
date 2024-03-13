use std::fs;
use std::io;
fn filling_laravel_deploy() -> String {
    let content:String = r#"
        
        #!/usr/bin/env bash
        echo "Running composer"
        composer global require hirak/prestissimo
        composer install --no-dev --working-dir=/

        echo "generating application key..."
        php artisan key:generate --show

        echo "Caching config..."
        php artisan config:cache

        echo "Caching routes..."
        php artisan route:cache

        echo "Running migrations..."
        php artisan migrate --force

        "#;
    return content;
}

fn filling_nginx_conf() -> String {
    let mut:content String = r#"

        server {
          listen 80;

          server_name _;

          root /public;
          index index.html index.htm index.php;

          sendfile off;

          error_log /dev/stdout info;
          access_log /dev/stdout;

          location /.git {
            deny all;
            return 403;
          }

          add_header X-Frame-Options "SAMEORIGIN";
          add_header X-XSS-Protection "1; mode=block";
          add_header X-Content-Type-Options "nosniff";

          charset utf-8;

          location / {
              try_files $uri $uri/ /index.php?$query_string;
          }

          location = /favicon.ico { access_log off; log_not_found off; }
          location = /robots.txt  { access_log off; log_not_found off; }

          error_page 404 /index.php;

          location ~* \.(jpg|jpeg|gif|png|css|js|ico|webp|tiff|ttf|svg)$ {
            expires 5d;
          }

          location ~ \.php$ {
            fastcgi_split_path_info ^(.+\.php)(/.+)$;
            fastcgi_pass unix:/var/run/php-fpm.sock;
            fastcgi_index index.php;
            fastcgi_param SCRIPT_FILENAME $document_root$fastcgi_script_name;
            fastcgi_param SCRIPT_NAME $fastcgi_script_name;
            include fastcgi_params;
          }

          # deny access to . files
          location ~ /\. {
            log_not_found off;
            deny all;
          }

          location ~ /\.(?!well-known).* {
            deny all;
          }
        }
        "#;

        return content;
}

fn create_conf_nginx(path:&str) {
    let file_path = format!("{}/nginx-site.conf", path);
    let content = filling_nginx_conf();
    match fs::write(file_path, content){
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

    match fs::read_dir("scripts"){
        Ok(_) => {
            let path = "scripts/00-laravel-deplot.sh";
            let content = filling_laravel_deploy();
            match fs::write(path, content) {
                Ok(_) => println!("{} created", path),
                Err(err) => {
                    match err.kind() {
                        ErrorKind::PermissionDenied => println!("permission denied"),
                        _ => println!("unknonw error"),
                    }
                },
            };
        },
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => println!("scripts dir not found"),
                _ => println!("unknwon err"),
            }
        },
    }
}
