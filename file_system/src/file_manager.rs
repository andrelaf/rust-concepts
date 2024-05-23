use std::env;
use std::fs;
// use std::fs::prelude::*;

use std::fs::File;
use std::fs::metadata;
use std::io::prelude::*;

// use std::io::write;

pub fn get_user_path() -> Option<String> {
    if let Some(home_path) = env::var_os("HOME").or(env::var_os("USERPROFILE")) {
        Some(home_path.into_string().unwrap())
    } else {
        None
    }
}

pub fn create_file(path: &str, name: &str) {
    println!("Creating file: {}", path);
    println!("File name: {}", name);

    let full_path = format!("{}/{}", path, name);

    match File::create(&full_path){
        Ok(mut file) => {
            let content = "Hello, World!";

            match file.write_all(content.as_bytes()) {
                Ok(_) => println!("File created successfully in: {}", full_path),
                Err(e) => println!("Error writing to file: {}", e),
            }
        },
        Err(e) => println!("Error creating file: {}", e),
    }
}

pub fn read_file(path: &str) {
    println!("Reading file: {}", path);

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return;
        }
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}

pub fn exists_file(path: &str) -> bool {
    if metadata(path).is_ok() {
        true
    } else {
        false
    }
}

pub fn read_directory(path: &str) -> Result<(), std::io::Error>{
    println!("Reading directory: {}", path);

    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if(path.is_dir()){
            println!("Directory: {:?}", path.display());
        } else {
            println!("File: {:?}", path.display());
        }
    }

    Ok(())
}