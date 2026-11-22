use std::fs::read_to_string;

use indexmap::IndexMap;

pub fn print_top_pairs_from_file(args: &[String]) {
    let file = read_to_string(&args[2]).unwrap();
    let data: Vec<u32> = serde_json::from_str(&file).expect("Could not parse file to Vec<u32>");

    let max_pair = top_pair_from_bytes(&data);

    println!("[{}, {}]", max_pair.0, max_pair.1);
}

fn top_pair_from_bytes(data: &[u32]) -> (u32, u32) {
    let mut counts = IndexMap::new();
    for window in data.windows(2) {
        *counts.entry((window[0], window[1])).or_insert(0) += 1;
    }

    let (max_pair, _) = counts
        .iter()
        .fold(None, |best, (pair, count)| match best {
            Some((_, best_count)) if best_count >= count => best,
            _ => Some((pair, count)),
        })
        .expect("Could not determine at least one pair.");
    *max_pair
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([1,2].to_vec(), (1,2))]
    #[case([1,2,3,1,2].to_vec(), (1,2))]
    #[case([5,1,8,3,4,1,8].to_vec(), (1,8))]
    fn test_top_pair(#[case] input: Vec<u32>, #[case] expected: (u32, u32)) {
        assert_eq!(top_pair_from_bytes(&input), expected);
    }
}
