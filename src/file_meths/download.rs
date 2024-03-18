use crate::Command;
use reqwest::Client;

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
    let mut url = String::new();
    let mut credentials = String::new();
    let mut body = String::new();
    let mut dai = false;
    let mut simp= true;

    for flg in command.flags.iter() {

        if *flg == "-creds".to_string() {
            let url = flag_taker(&command.flags,"-url".to_string());
        } else if *flg == "-url".to_string() {
            let creds = flag_taker(&command.flags,"-creds".to_string());
        } else if *flg == "-body".to_string() {
            let body = flag_taker(&command.flags,"-body".to_string());
        } else if *flg == "-dai" {
            //dai is download and install
            dai = true;
            simp= false;
        } else {
            simp = true;
        }

    }
    if dai {
        download_and_install(url.clone(), credentials.clone(), body.clone());
    } else {
        simple_download(url, credentials, body);
    } 

}

pub async fn download_and_install(url:String, credentials:String, _body:String){
    let myurl:&str = url.as_str();
    let client = Client::new();
    let result = client.get(myurl)
        .send()
        .await;

    let data:Vec<&str> = credentials.split(',').collect();
    let myid:u32 = match data[1].parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let credential = Credential::new(data[0].to_string(), myid);
    //I must compare user's credentials with the credentail the APi is waiting for

    let resbody = match result {
        Ok(res) => {
            let res_body = match res.text().await {
                Ok(rawbody) => rawbody,
                Err(err) => format!("e: {:?}", err),
            };
        },
        Err(e) => eprintln!("e: {}", e),
    };

    println!("the response body is: \n {}\n",resbody);
}

pub async fn simple_download(url:String, _credentials:String, _body:String){
    let myurl:&str = url.as_str();
    let client = Client::new();
    let result = client.get(myurl)
        .send()
        .await;

    match result {
        Ok(res) => {
            let res_body = res.text().await;
            match res_body {
                Ok(rawbody) => {
                    println!("body {}", rawbody);
                },
                Err(err) => eprintln!("e: {}", err),
            }
        },
        Err(e) => eprintln!("e: {}", e),
    }
    println!("the result is: \n{}\n", result);
}
