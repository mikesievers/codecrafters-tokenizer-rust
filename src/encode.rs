use std::{
    error::Error,
    fs::{File, read_to_string},
    io::BufReader,
};

use crate::bytes::bytes_from_string;
use crate::merge::merge_pair;

pub fn encode_and_print_result(args: &[String]) -> Result<(), Box<dyn Error>> {
    let data = read_to_string(&args[2])?;

    assert_eq!(args.len(), 5);
    assert_eq!(&args[3], "--merges");
    // Read merges
    let merges_file = File::open(&args[4])?;
    let buf_reader = BufReader::new(merges_file);
    let merges: Vec<(u32, u32)> = serde_json::from_reader(buf_reader)?;

    let encoded = encode(&data, &merges);

    println!("{encoded:?}");

    Ok(())
}

fn encode(data: &str, merges: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut encoded: Vec<u32> = bytes_from_string(data).into_iter().map(u32::from).collect();

    for (rank, &pair) in merges.iter().enumerate() {
        encoded = merge_pair(
            &encoded,
            pair,
            256 + u32::try_from(rank).expect("rank greater than u32 found"),
        );
    }

    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let data = "aaab";
        let merges: Vec<(u32, u32)> = [(97, 97), (256, 97), (257, 98)].to_vec();

        assert_eq!(encode(data, &merges), [258].to_vec());
    }

    #[test]
    fn test_encode_empty_string() {
        let data = "";
        let merges: Vec<(u32, u32)> = [(97, 97), (256, 97), (257, 98)].to_vec();

        assert_eq!(encode(data, &merges), [].to_vec() as Vec<u32>);
    }
}
