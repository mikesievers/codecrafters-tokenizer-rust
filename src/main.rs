#![warn(clippy::pedantic)]

mod bytes;
mod merge;
mod top_pair;
mod train;

use std::env;

use crate::bytes::print_file_as_bytes;
use crate::merge::merge_pair_and_print_substituted_result;
use crate::top_pair::print_top_pairs_from_file;
use crate::train::train_and_print_result;

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
            train_and_print_result(&args);
        }
        _ => {
            eprintln!("unknown command: {}", args[1]);
        }
    }
}
