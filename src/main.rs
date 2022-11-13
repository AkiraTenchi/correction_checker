extern crate core;

use std::fs;
use std::io;
use reqwest::{Client, Error, Response};
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::Deserialize;
use serde_json::{json, json_internal_vec, Value};
use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct User {
    login: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
struct Issue {
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let urls = get_names().expect("Url Vector");
    let token = get_token().unwrap();
    println!("{:?}", urls);
    for url in urls {
        let req = authenticated_get_request(&client, &*format!("https://api.github.com/repos/hhu-progra/{}/issues/1", url), &*token).await.expect("req content");
        match req.status() {
            reqwest::StatusCode::OK => {
                match req.json::<Issue>().await {
                    Ok(parsed) => println!("{:?}", parsed.extra.get("user").unwrap().get("login")),
                    Err(_) => println!("Didnt match pattern")
                };
            }
            other => {
                panic!("shit dont work {:?}", other);
            }
        };
    }



    Ok(())
}

async fn authenticated_get_request(client: &Client, url: &str, token: &str) -> Result<Response, Error> {
    let token_str = &*format!("Bearer {}", token.trim());
    client.get(url)
        .header(AUTHORIZATION, token_str)
        .header(USER_AGENT, "correction_checker_app")
        .send()
        .await
}

fn get_token() -> io::Result<String> {
    println!("Enter Token!");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

fn get_names() -> io::Result<Vec<String>> {
    let mut file_path = String::new();    
    println!("Enter File Path!");
    io::stdin().read_line(&mut file_path)?;
    let contents = fs::read_to_string(file_path.trim())?;
    let lines: Vec<&str> = contents.lines().collect();
    let mut urls = Vec::new();
    for line in lines {
        let temp: Vec<&str> = line.split_whitespace().collect();
        if temp[0].eq("*"){
            let temp1: Vec<&str> = temp[2].split("/").collect();
            urls.push(temp1[4].to_string());
        }

    }

    Ok(urls)
}
