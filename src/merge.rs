use std::fs::read_to_string;

use itertools::Itertools;

pub fn merge_pair_and_print_substituted_result(args: &[String]) {
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

    let substituted_data = merge_pair(&data, *pair, 256);

    println!("{substituted_data:?}");
}

fn merge_pair(data: &[u32], pair: (u32, u32), token_id: u32) -> Vec<u32> {
    // Loop over the data and substitute a pair if found
    let mut substituted_data = Vec::with_capacity(data.len());
    let mut i = 0;
    while i < data.len() - 1 {
        if pair == (data[i], data[i + 1]) {
            // TODO: change hardcoded value with dynamically matched value
            substituted_data.push(token_id);
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
    substituted_data
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([1, 2, 3, 1, 2].to_vec(), (1,2), 256, [256,3,256].to_vec())]
    fn test_merge_pair(
        #[case] input: Vec<u32>,
        #[case] pair: (u32, u32),
        #[case] token_id: u32,
        #[case] expected: Vec<u32>,
    ) {
        assert_eq!(merge_pair(&input, pair, token_id), expected);
    }
}
