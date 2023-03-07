use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use zip::ZipArchive;

/// Bruteforces
pub fn ezip(filename: &str, password_list: &str) -> Option<String> {
    let zip_file = fs::File::open(filename).expect("Could not open zip file.");
    let mut archive = ZipArchive::new(&zip_file).expect("Error reading the zip file.");

    let password_file = fs::read_to_string(password_list).expect("Error om reading password file.");
    let password_list: Vec<&str> = password_file.lines().collect();

    let mut guessed: Vec<&str> = Vec::new();

    let bar = ProgressBar::new(password_list.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] ({eta})")
            .unwrap(),
    );

    for pass in password_list {
        bar.inc(1);
        match archive.by_index_decrypt(0, pass.as_bytes()) {
            Ok(file) => match file {
                Ok(_) => {
                    guessed.push(pass);
                }
                Err(_) => continue,
            },
            Err(_) => panic!("Error reading zipfile"),
        };
    }

    bar.finish();
    if guessed.is_empty() {
        None
    } else {
        Some(guessed.join("\n"))
    }
}
