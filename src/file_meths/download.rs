use crate::Command;
use reqwest::Client;

struct Credential {
    name:String,
    id:u32,
}

pub fn download(command:&Command){
    let mut url = String::new();
    let mut credentials = String::new();
    let mut body = String::new();

    let mut if_credentials: bool = false;
    let mut if_body: bool = false;
    let mut if_url: bool = false;
    
    let mut install: bool = false;

    for flg in command.flags.iter() {
        if if_url {
            url = flg.to_string();
            if_url = false;
        } else if flg == "-url" {
            if_url = true;
        }

        if if_credentials {
            credentials = flg.to_string();
            if_credentials = false;
        } else if flg == "-creds" {
            if_credentials = true;
        }

        if if_body {
            body = flg.to_string();
            if_body = false;
        } else if flg == "-body" {
            if_body = true;
        }

        if flg == "-d" {
            download_and_install(url.clone(), credentials.clone(), body.clone());
        } else {
            simple_download(url.clone(), credentials.clone(), body.clone());
        }
    }

}

pub async fn download_and_install(url:String, _credentials:String, _body:String){
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
}
