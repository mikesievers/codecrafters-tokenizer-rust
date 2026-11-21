#![warn(clippy::pedantic)]
use std::env;
use std::fs;

fn main() {
    eprintln!("Logs from your program will appear here!");

    let args: Vec<String> = env::args().collect();
    if args[1] == "bytes" {
        let data: Vec<u8> = fs::read(&args[2]).unwrap();
        println!("{}", serde_json::to_string(&data).unwrap());
    } else {
        eprintln!("unknown command: {}", args[1]);
    }
}
