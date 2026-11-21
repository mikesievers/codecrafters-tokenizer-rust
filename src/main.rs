#![warn(clippy::pedantic)]
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::read_to_string;

use indexmap::IndexMap;

fn main() {
    eprintln!("Logs from your program will appear here!");

    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "bytes" => {
            print_file_as_bytes(&args);
        }
        "top-pair" => {
            print_top_pairs_from_file(&args);
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

fn print_top_pairs_from_file(args: &[String]) {
    let file = read_to_string(&args[2]).unwrap();
    let data: Vec<u32> = serde_json::from_str(&file).expect("Could not parse file to Vec<u32>");

    let mut counts = IndexMap::new();
    for window in data.windows(2) {
        *counts.entry((window[0], window[1])).or_insert(0) += 1;
    }

    eprintln!("{:?}", counts);

    let (max_pair, _) = counts
        .iter()
        .fold(None, |best, (pair, count)| match best {
            Some((_, best_count)) if best_count >= count => best,
            _ => Some((pair, count)),
        })
        .expect("Could not determine at least one pair.");

    println!("[{}, {}]", max_pair.0, max_pair.1);
}
