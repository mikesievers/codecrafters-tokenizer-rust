#![warn(clippy::pedantic)]

mod bytes;
mod merge;
mod top_pair;

use std::env;
use std::fs::read_to_string;

use crate::bytes::print_file_as_bytes;
use crate::merge::merge_pair_and_print_substituted_result;
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
            merge_pair_and_print_substituted_result(&args);
        }
        "train" => {
            train(&args);
        }
        _ => {
            eprintln!("unknown command: {}", args[1]);
        }
    }
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
