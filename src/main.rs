use std::env;
use std::io::{self};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    match env::args().nth(1) {
        Some(input) => {
            parse_string(input);
        }
        None => match stdin.read_line(&mut input) {
            Ok(len) => {
                if len == 0 {
                    invalid_token();
                } else {
                    parse_string(input);
                }
            }
            Err(_error) => {
                invalid_token();
            }
        },
    }
    std::process::exit(0);
}

fn parse_string(token: String) -> () {
    let split: Vec<&str> = token.trim().split(".").collect();
    let token = String::from(split[2]);
    if let Ok(mut token) = base64::decode_config(&token, base64::URL_SAFE) {
        token.truncate(token.len() - 64);
        if let Ok(token) = String::from_utf8(token) {
            println!("{}", token);
        } else {
            invalid_token();
        }
    } else {
        invalid_token();
    }
}

fn invalid_token() {
    eprintln!("Invalid token.");
    std::process::exit(1);
}
