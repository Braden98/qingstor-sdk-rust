extern crate reqwest;
mod config;
mod request;
//use config::*;
use std::*;


fn main(){
    let body = reqwest::get("https://www.rust-lang.org").unwrap()
        .text().unwrap();
    println!("body = {:?}", body);

    let s = "
access_key_id: access_key
secret_access_key: secret
protocol: https
";
    let mut c:Config = Config::load_from_str(&s).unwrap();
    c.check().unwrap();

    Service::init(&mut c).unwrap();
    
}