use std::fs;
use std::io::ErrorKind;

fn routes_content() -> [&str;2] {
    let api_route_cnt = String::from(r#"
                                       var express = require("express");
                                       var router = express.Router();

                                       const { apifirst } = require("../controllers/api_routes.js");

                                       router.get('/api1', apifirst);
                                       module.exports = router;"#);
    let normal_route_cnt = String::from(r#"
                                       var express = require("express");
                                       var router = express.Router();

                                       const { home } = require("../controllers/http_routes.js");

                                       router.get('/home', home);
                                       module.exports = router;"#);

    let api:&str = api_route_cnt.as_str();
    let norm:&str= normal_route_cnt.as_str();
    return [api, norm];
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
    let conn = String::from(r#"
                            const mongoose = require("mongoose")
                            mongoose.set("strictQuery", false)
                            const mongoDB = "mongodb:://127.0.0.1/defaultdb"

                            main.catch( err => console.log(err));
                            async function main(){
                                await mongoose.connect(mongoDB);
                            }
                            "#);
    let db_models = String::from(r#"
                                 cosnt mongoose = require("mongoose")
                                 const schema = mongoose.Schema;

                                 const config = {
                                    a_string:String,
                                    a_date:Date,
                                 }
                                 //each model has its own schema

                                 const GeericUser = new Schema(config)

                                 const User = mongoose.model("User", GenericUser)
                                 "#);

    for file in files.iter() {

        let mut content = String::new();

        if file == "models.js" { 
            content = db_models;
        } else if file == "conn.js" { 
            content = conn; 
        } else {
            content  = "".to_string();
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

fn db_content() -> [&str;2] {
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
    let qery:&str = db_content_qery.as_str();
    let conn:&str = db_content_conn.as_str();
    let contents:[&str;2] = [qery, conn];
    return contents;
}

pub fn express_project(){
    let dirs:[&str,2]= ["routes", "db"];
    let files:[&str,3]=[".gitignore", ".env", "package.json"];
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
        if file == "package.json" {
            content = package_cnt;
        } else if file == ".gitignore" {
            content = ignored_cnt;
        } else if file == ".env" {
            content = env_cnt;
        }
        match fs::write(file, content){
            Ok(_) => println!("{} created", file),
            Err(err) => eprintln!("err:{}", err),
        }
    }

    let app_file = String::from(r#"
                                    var express = require("express")
                                    var app = express()


                                    var routes = require('./routes/routes.js')
                                    var apiroutes = require('./routes/apiroutes.js')

                                    app.use('/api',apiroutes)
                                    app.use('/', routes)
                                    app.use(morgan('tiny'));

                                    app.set('port', porcess.env.HTTP_SERVER_PORT);

                                    module.exports = app;"#);
    let server_file = String::from(r#"
                                    const dotenv = require("dotenv");
                                    dotenv.config();

                                    const app = require("./app")
                                    
                                    app.listening(app.get('port'), () => {
                                        console.log("listening on port: " + app.get('port'));
                                    });
                                "#);
    for dir in dirs.iter() {
        let mut contents:[&str; 2];
        let mut files:[&str;2];
        if dir == "routes" {
            contents = routes_content();
            files = ["routes.js", "apiroutes.js"];
        } else if dir == "db" {
            contents = db_content();
            files = ["conn.js", "queries.js"];
        }
        let counter:u32 = 0;

        for file in files.iter() {
            let path = format!("{}/{}", dir, file);
            match fs::write(path, contents[counter]) {
                Ok(_) => println!("successfully created {}", file),
                Err(err) => println!("e:{}", err),
            }
            counter+=1;
        }
    }
}


