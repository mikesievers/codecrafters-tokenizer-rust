#![warn(clippy::pedantic)]
use std::env;
use std::fs;

fn main() {
    eprintln!("Logs from your program will appear here!");

    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "bytes" => {
            print_file_as_bytes(&args);
        }
        _ => {
            eprintln!("unknown command: {}", args[1]);
        }
    }
}

fn print_file_as_bytes(args: &[String]) {
    let data: Vec<u8> = fs::read(&args[2]).unwrap();
    println!("{}", serde_json::to_string(&data).unwrap());
}
