mod file_manager;

use file_manager::{get_user_path, create_file, read_file,exists_file,read_directory};

fn main() {

    let user_path = get_user_path().unwrap();

    create_file(&user_path, "hello.txt");

    let full_path = format!(r"{}/{}", user_path, "hello.txt");

    read_file(&full_path);

    println!("File exists: {}", exists_file(&full_path));

    match read_directory(&user_path){
        Ok(_) => println!("Directory read successfully"),
        Err(e) => println!("Error reading directory: {}", e),
    }
}
