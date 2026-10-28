use std::fs;
use std::path::Path;
use std::io;
use std::io::Write;
use walkdir::WalkDir;

fn main() {
    println!("Welcome to the file scanner!");
    println!("--------------------------------");
    println!("Enter the file name: ");

    let dir = Path::new("/home");

    print!("~> ");
    io::stdout().flush().unwrap();
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Error");
    file_name = file_name.trim().to_string();

    for entry in WalkDir::new(dir) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let current_path = entry.path();
        let current_name = entry.file_name().to_string_lossy();
        if current_path.is_file() && current_name == file_name {
            println!("FOUND: {}", current_path.display());
        }
    }
}
