#![warn(clippy::pedantic)]

mod top_pair;

use std::env;
use std::fs;
use std::fs::read_to_string;

use itertools::Itertools;

use crate::top_pair::print_top_pairs_from_file;

fn main() {
    eprintln!("Starting tokenizer.");

    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "bytes" => {
            print_file_as_bytes(&args);
        }
        "top-pair" => {
            print_top_pairs_from_file(&args);
        }
        "merge" => {
            merge_pair(&args);
        }
        "train" => {
            train(&args);
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

fn merge_pair(args: &[String]) {
    let file = read_to_string(&args[2]).unwrap();
    let data: Vec<u32> = serde_json::from_str(&file).expect("Could not parse file to Vec<u32>");
    assert_eq!(args.len(), 5);
    assert_eq!(&args[3], "--pair");

    // Extract pair as tuple of ints
    let pair = &args[4]
        .split(',')
        .map(|s| {
            s.parse::<u32>()
                .expect("Could not parse parameter into number")
        })
        .collect_tuple::<(u32, u32)>()
        .expect("Not a tuple");

    // Loop over the data and substitute a pair if found
    let mut substituted_data = Vec::with_capacity(data.len());
    let mut i = 0;
    while i < data.len() - 1 {
        if *pair == (data[i], data[i + 1]) {
            // TODO: change hardcoded value with dynamically matched value
            substituted_data.push(256);
            i += 2;
        } else {
            substituted_data.push(data[i]);
            i += 1;
        }
    }
    // check the last one, if any
    if i < data.len() {
        substituted_data.push(data[i]);
    }

    println!("{substituted_data:?}");
}

fn train(args: &[String]) {
    let file = read_to_string(&args[2]).unwrap();
    let data: Vec<u32> = serde_json::from_str(&file).expect("Could not parse file to Vec<u32>");
    assert_eq!(args.len(), 5);
    assert_eq!(&args[3], "--num-merges");
    let num_merges = args[4]
        .parse::<usize>()
        .expect("Could not parse number of iterations");
}
