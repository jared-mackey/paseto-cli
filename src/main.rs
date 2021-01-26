use std::env;
use std::io::{self};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    match env::args().nth(1) {
        Some(input) => {
            parse_string(input);
            return;
        }
        None => match stdin.read_line(&mut input) {
            Ok(len) => {
                if len == 0 {
                    return;
                } else {
                    parse_string(input);
                    return;
                }
            }
            Err(error) => {
                eprintln!("error: {}", error);
                return;
            }
        },
    }
}

fn parse_string(token: String) -> () {
    let split: Vec<&str> = token.trim().split(".").collect();
    let token = String::from(split[2]);
    let mut token =
        base64::decode_config(&token, base64::URL_SAFE).expect("Could not decode base64");
    token.truncate(token.len() - 64);
    let token = String::from_utf8(token).expect("Could not convert back into string");
    println!("{}", token);
}
