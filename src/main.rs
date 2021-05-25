use std::env;
use std::io::{self};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    match env::args().nth(1) {
        Some(input) => {
            parse_token(input);
        }
        None => match stdin.read_line(&mut input) {
            Ok(len) => {
                if len == 0 {
                    invalid_token(ValidationError::LengthOrFormat);
                } else {
                    parse_token(input);
                }
            }
            Err(_error) => {
                invalid_token(ValidationError::LengthOrFormat);
            }
        },
    }
    std::process::exit(0);
}

enum ValidationError {
    LengthOrFormat,
    Version,
    Type
}

fn parse_token(token: String) {
    let token = validate_token(token);
    if let Ok(mut token) = base64::decode_config(&token, base64::URL_SAFE) {
        token.truncate(token.len() - 64);
        if let Ok(token) = String::from_utf8(token) {
            println!("{}", token);
        } else {
            invalid_token(ValidationError::LengthOrFormat);
        }
    } else {
        invalid_token(ValidationError::LengthOrFormat);
    }
}

fn validate_token(token: String) -> String {
    let split: Vec<&str> = token.trim().split('.').collect();
    if split.len() < 3 {
        invalid_token(ValidationError::LengthOrFormat);
    }

    // Only pulls data from v2 tokens
    if split[0] != "v2" {
        invalid_token(ValidationError::Version);
    }

    // Only pulls data from public tokens
    if split[1] != "public" {
        invalid_token(ValidationError::Type);
    }

    String::from(split[2])
}

fn invalid_token(error: ValidationError) {
    match error {
        ValidationError::LengthOrFormat => eprintln!("Invalid token length or format."),
        ValidationError::Version => eprintln!("Invalid token version. Only v2.public supported."),
        ValidationError::Type => eprintln!("Invalid token type. Only v2.public supported."),
    };
    std::process::exit(1);
}
