use crate::CliCommand;
use reqwest::Client;
use futures::executor::block_on;
use crate::file_meths::utils::{ flag_taker };

use std::process::Command;

pub struct Credential {
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

pub fn download(command:&CliCommand){
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
            Err(e)=> format!("err: {}", e),
        };
    return body;
}

fn direct_file(url:String) -> bool {
    let available_fformat = [".xz", ".xyz"];
    let reversed_url:String = url.chars().rev().collect();
    let mut termination:String = String::new();
    for c in reversed_url.chars() {
        termination =  c.to_string() + &termination;
        if c == '.' {
            if termination.len() > 0 {
                for available in available_fformat.iter(){
                    if termination ==  *available {
                        return true;
                    } 
                }
                return false;
            }
        }
    }
    return false;
}

fn check_ftp(url:String) -> bool {
    let valid_prots = ["ftp:", "http:"];
    let mut urlservice = String::new();
    for c in url.chars() {
        urlservice.push(c);
        if c == ':' {
            for valid in valid_prots.iter() {
                if urlservice == *valid {
                    println!("{} is valid", urlservice);
                    return true;
                }
            }
            return false;
        }
    }
    return false;
}
fn tar_format(file:String) -> String {
    let index = if let Some(i) = file.rfind('.') {i} else {0};
    let extension = &file[index..];
    let mut flag = String::new();

    if extension == ".xz" || extension == ".txz" {
        //x stands for extract
        flag = "-xJvf".to_string();
    }
    if extension == ".gz" || extension == ".tgz"  {
        flag = "-xzvf".to_string();
    }
    if extension == ".bz2" || extension ==".tbz2" {
        flag = "-xjvf".to_string();
    } 
    if extension == ".Z"  {
        flag = "-xZvf".to_string();
    }
    if extension == ".lz" || extension == ".tlz"  {
        flag = "--lzma -xvf".to_string();
    } 
    if extension == ".lz4" || extension == ".tlz4" {
        flag = "--lz4 -xvf".to_string();
    }
    if  extension == ".sz" || extension == ".tsz" {
        flag = "--zstd -xvf".to_string();
    }
    return flag;
}

fn install_extracted_pkg(pkg_name:String){
    let cmd:String = "cd ".to_string() + pkg_name.as_str();
    let entering  = Command::new(cmd)
        .args(&[""])//I think there's not arguments for install
        .output()
        .expect("error entering pkg");

    if entering.status.success() { println!("perfect"); } 
    else { println!("err"); };

    let ins = Command::new("./configure")
        .args(&[""])//I think there's not arguments for install
        .output()
        .expect("error executing ./configure");
    
    if ins.status.success() {
        println!("installing");
    } else {
        let err = String::from_utf8_lossy(&ins.stderr);
        eprintln!("error extracting:{} ", err);
        return;
    }

    let ins = Command::new("./install.sh")
        .args(&[""])//I think there's not arguments for install
        .output()
        .expect("error executing install.sh");

    if ins.status.success() {
        println!("installing");
    } else {
        let err = String::from_utf8_lossy(&ins.stderr);
        eprintln!("error extracting:{} ", err);
        return;
    }
}

fn standard_installation(url:String){
    let index = if let Some(i) = url.rfind('/') {i} else {0};
    let file:&str = &url[index..];
    let flag = tar_format(file.to_string());
    let output = Command::new("tar")
        .args(&[flag, file.to_string()])
        .output()
        .expect("Failed in tar execution");
    
    if output.status.success() {
        println!("extraction completed");
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        eprintln!("error extracting:{} ", err);
        return;
    }
    install_extracted_pkg(file.to_string());
}

pub fn download_and_install(url:String, credentials:String, _body:String){
    if !check_ftp(url.clone()) {
        println!("please enter an available source");
        return;
    }

    if !direct_file(url.clone()){
        println!("pls, enter a valid file format");
        return;
    }
    let result = block_on(dareq(url.clone()));
    let data:Vec<&str> = credentials.split(',').collect();
    let myid:u32 = match data[1].replace(" ", "").parse() {
        Ok(num) => num,
        Err(err) => {
            println!("error parsing data 1:{}", err);
            0
        },
    };
    let credential = Credential::new(data[0].to_string(), myid);
    println!("{}", result);
    println!("name: {}", credential.name);
    println!("id: {}", credential.id);
    standard_installation(url.clone());
    //I must compare user's credentials with the credentail the APi is waiting for
    //what is means install? I can use cmd tools like curls 
}

pub fn simple_download(url:String, credentials:String, _body:String){
    if !check_ftp(url.clone()) {
        println!("please enter an available source(ftp)");
    }
    if !direct_file(url.clone()){
        println!("pls, enter a valid file format");
        return;
    }

    let result = block_on(dareq(url));    
    let data:Vec<&str> = credentials.split(',').collect();
    let myid:u32 = match data[1].replace(" ", "").parse() {
        Ok(num) => num,
        Err(err) => {
            eprintln!("error parsing data 1:{}", err);
            0
        },
    };
    let credential = Credential::new(data[0].to_string(), myid);
    println!("{}", result);
    println!("name: {}", credential.name);
    println!("id: {}", credential.id);
}
