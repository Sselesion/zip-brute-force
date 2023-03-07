use colored::Colorize;
use std::env;
use std::path::Path;
use zip_brute_force::ezip;

enum Status {
    Ezip,
    FileNotSupported,
    FileNotFound,
}

fn parse_type(path: &Path) -> Status {
    match path.extension() {
        Some(e) => match e.to_str().unwrap() {
            "zip" => {
                if path.exists() {
                    Status::Ezip
                } else {
                    Status::FileNotFound
                }
            }
            _ => Status::FileNotSupported,
        },
        None => Status::FileNotSupported,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!(
            "{}",
            "Usage: veldora <filename> <password_list>".bright_green()
        );
        return;
    }

    let filename = &args[1];
    let password_list = &args[2];

    let path = Path::new(filename);

    let result = match parse_type(path) {
        Status::Ezip => ezip::ezip(filename, password_list),
        Status::FileNotSupported => Some("Filetype not supported!".to_string()),
        Status::FileNotFound => Some("Target file not Found!".to_string()),
    };

    match result {
        Some(pass) => println!("Possible Passwords:\n{}", pass.bright_green()),
        None => println!("Couldnt get pass!"),
    }
}
