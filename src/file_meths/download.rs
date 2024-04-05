use crate::Command;
use reqwest::Client;
use futures::executor::block_on;
use crate::file_meths::utils::{ flag_taker };

struct Credential {
    name:String,
    id:u32,
}

impl Credential {
    fn new(name:String,id:u32) -> Self {

        Self {
            name,
            id,
        }
    }
}

pub fn download(command:&Command){
    let mut url  = String::new();
    let mut creds= String::new();
    let mut body = String::new();
    let mut dai = false;

    for flg in command.flags.iter() {

        if *flg == "-creds".to_string() {
            url = flag_taker(&command.flags,"-url".to_string());
        } else if *flg == "-url".to_string() {
            creds = flag_taker(&command.flags,"-creds".to_string());
        } else if *flg == "-body".to_string() {
            body = flag_taker(&command.flags,"-body".to_string());
        } else if *flg == "-dai" {
            //dai is download and install
            dai = true;
        } else {
            dai = false;
        }

    }
    if dai {
        download_and_install(url, creds.clone(), body.clone());
    } else {
        simple_download(url, creds, body);
    } 
}

async fn dareq(url:String) -> String {
    let client = Client::new();
    let body:String = match client.get(url)
        .send()
        .await {
            Ok(rawbody) => {
                match rawbody.text().await {
                    Ok(strbody) => strbody,
                    Err(err) => format!("err: {}", err),
                }
            },
            Err(e)=> format!("{}", e),
        };
    return body;
}

fn direct_file(url:String) -> bool {
    let _available_file_fs = [".xz", ".xyz"];
    let reversed_url:String = url.chars().rev().collect();
    let mut termination:String = String::new();
    for c in reversed_url.chars() {
        termination = termination + c.to_string().as_str();
        if c == '.' {
            if termination.len() > 0 {
                println!("termination: {}",termination);
                return true;
            }
        }
    }
    return false;
}

fn check_ftp(url:String) -> bool {
    let service = String::from("ftp://");
    let mut counter:usize = 0;
    let mut urlservice = String::new();
    for c in url.chars() {
        urlservice.push(c);
        if counter == 6 {
            break;
        }
        counter += 1;
    }
    
    if urlservice == service {
        true
    } else {
        false
    }
}

pub fn download_and_install(url:String, credentials:String, _body:String){
    if !check_ftp(url.clone()) {
        println!("please enter an available source(ftp)");
        return;
    }

    if !direct_file(url.clone()){
        println!("pls, enter a valid file format");
        return;
    }
    let result = block_on(dareq(url));
    let data:Vec<&str> = credentials.split(',').collect();
    let myid:u32 = match data[1].parse() {
        Ok(num) => num,
        Err(err) => {
            println!("error parsing data 1:{}", err);
            0
        },
    };
    let credential = Credential::new(data[0].to_string(), myid);
    println!("name:{}\nid:{}", credential.name, credential.id);
    println!("{}", result);
    //I must compare user's credentials with the credentail the APi is waiting for
    //what is means install? I can use cmd tools like curls 
}

pub fn simple_download(url:String, credentials:String, _body:String){
    if !check_ftp(url.clone()) {
        println!("please enter an available service (ftp)");
        return;
    }
    if !direct_file(url.clone()){
        println!("pls, enter a valid file format");
        return;
    }

    let result = block_on(dareq(url));    
    let data:Vec<&str> = credentials.split(',').collect();
    let myid:u32 = match data[1].parse() {
        Ok(num) => num,
        Err(err) => {
            eprintln!("error parsing data 1:{}", err);
            0
        },
    };
    let credential = Credential::new(data[0].to_string(), myid);
    println!("name:{}\nid:{}", credential.name, credential.id);
    println!("result:{}", result);
    
}
