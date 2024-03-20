use std::fs;
use std::io::ErrorKind;
use std::io;

fn routes_content() -> Vec<String> {
    let api_route_cnt = r#"
                                       var express = require("express");
                                       var router = express.Router();

                                       const { apifirst } = require("../controllers/api_routes.js");

                                       router.get('/api1', apifirst);
                                       module.exports = router;"#;
    let normal_route_cnt = r#"
                                       var express = require("express");
                                       var router = express.Router();

                                       const { home } = require("../controllers/http_routes.js");

                                       router.get('/home', home);
                                       module.exports = router;"#;
    
    let mut contents: Vec<String> = Vec::new();
    contents.push(api_route_cnt.to_string());
    contents.push(normal_route_cnt.to_string());
    return contents;
}

fn mongoose() {
    let dir = "db";
    match fs::create_dir(dir) {
        Ok(_) => println!("{dir} created"),
        Err(e)=> {
            if let ErrorKind::PermissionDenied = e.kind() {
                println!("permission denied creating {dir}");
            } else {
                println!("unknown error");
            }
        },
    }

    let files:[&str;2] = ["models.js", "conn.js"];
    let conn = r#"
                            const mongoose = require("mongoose")
                            mongoose.set("strictQuery", false)
                            const mongoDB = "mongodb:://127.0.0.1/defaultdb"

                            main.catch( err => console.log(err));
                            async function main(){
                                await mongoose.connect(mongoDB);
                            }
                            "#;
    let db_models = r#"
                                 cosnt mongoose = require("mongoose")
                                 const schema = mongoose.Schema;

                                 const config = {
                                    a_string:String,
                                    a_date:Date,
                                 }
                                 //each model has its own schema

                                 const GeericUser = new Schema(config)

                                 const User = mongoose.model("User", GenericUser)
                                 "#;

    for file in files.iter() {
        let content:String;

        if file.to_string()  == "models.js" { 
            content = db_models.to_string();
        } else if file.to_string() == "conn.js" { 
            content = conn.to_string(); 
        } else {
            content  = String::from("");
        }
        let path = format!("db/{}", file);
        match fs::write(path, content) {
            Ok(_) => {
                println!("{file} created");
            },
            Err(e) => {
                if let ErrorKind::PermissionDenied = e.kind() {
                    println!("permission denid writing {file}")
                } else {
                    println!("unknown error writing {file}")
                }
            },
        }
    }
}

fn db_content() -> Vec<String> {
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
    let mut contents: Vec<String> = Vec::new();
    contents.push(db_content_conn);
    contents.push(db_content_qery);
    return contents;
}

pub fn express_project(){
    let dirs = vec!["routes", "db"];
    let files= vec![".gitignore", ".env", "package.json"];
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
                                        "#);
    let package_cnt:String = format!("{} {} {}", devfield, version, command);
    let ignored_cnt:String = String::from("/node_modules\n");
    for file in files.iter() {
        let mut content = String::new(); 
        if file.to_string() == "package.json" {
            content = package_cnt.clone();
        } else if file.to_string() == ".gitignore" {
            content = ignored_cnt.clone();
        } else if file.to_string() == ".env" {
            content = env_cnt.clone();
        }
        match fs::write(file, content){
            Ok(_) => println!("{} created", file),
            Err(err) => eprintln!("err:{}", err),
        }
    }

    let _app_file = String::from(r#"
                                    var express = require("express")
                                    var app = express()


                                    var routes = require('./routes/routes.js')
                                    var apiroutes = require('./routes/apiroutes.js')

                                    app.use('/api',apiroutes)
                                    app.use('/', routes)
                                    app.use(morgan('tiny'));

                                    app.set('port', porcess.env.HTTP_SERVER_PORT);

                                    module.exports = app;"#);
    let _server_file = String::from(r#"
                                    const dotenv = require("dotenv");
                                    dotenv.config();

                                    const app = require("./app")
                                    
                                    app.listening(app.get('port'), () => {
                                        console.log("listening on port: " + app.get('port'));
                                    });
                                "#);
    for dir in dirs.iter() {
        let mut contents:Vec<String>  = Vec::new();
        let mut filesin:Vec<String> = Vec::new();
        if dir.to_string() == "routes" {
            contents = routes_content();
            filesin.push("routes.js".to_string());
            filesin.push("apiroutes.js".to_string());
        } else if dir.to_string() == "db" {
            println!("just selelct your favorite db\n1.mongoose\n2.Sqlite\n3.default");
            let mut response = String::new();
            io::stdin()
                .read_line(&mut response)
                .expect("error selecting the db");

            if response == "1" {
                mongoose();
            } else if response == "2" {
                println!("making the sqlite function");
            } else if response == "3" {
                contents = db_content();
                filesin.push("conn.js".to_string());
                filesin.push("queries.js".to_string());
            }
        }
        let mut counter:usize = 0;

        for file in filesin.iter() {
            let path = format!("{}/{}", dir, file);
            match fs::write(path, &contents[counter]) {
                Ok(_) => println!("successfully created {}", file),
                Err(err) => println!("e:{}", err),
            }
            counter+=1;
        }
    }
}

