use std::fs::read_to_string;

use crate::{bytes::bytes_from_string, merge::merge_pair, top_pair::top_pair_from_bytes};

pub fn train_and_print_result(args: &[String]) {
    let data = read_to_string(&args[2]).unwrap();
    assert_eq!(args.len(), 5);
    assert_eq!(&args[3], "--num-merges");
    let num_merges = args[4]
        .parse::<u32>()
        .expect("Could not parse number of iterations");

    let substituted_pairs = train(&data, num_merges);

    println!("{substituted_pairs:?}");
}

fn train(data: &str, num_merges: u32) -> Vec<[u32; 2]> {
    let mut bytes: Vec<u32> = bytes_from_string(data).into_iter().map(u32::from).collect();

    let mut substituted_pairs = Vec::with_capacity(num_merges as usize);

    for next_id in 256..256 + num_merges {
        let pair = top_pair_from_bytes(&bytes);
        substituted_pairs.push([pair.0, pair.1]);

        bytes = merge_pair(&bytes, pair, next_id);
    }

    substituted_pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_train() {
        let data = "aaabdaaabac";

        assert_eq!(train(data, 3), [[97, 97], [256, 97], [257, 98]]);
    }
}
