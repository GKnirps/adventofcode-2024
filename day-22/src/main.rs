#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let initial_numbers = parse(&content)?;

    let sum_2000 = sum_number_n(&initial_numbers, 2000);
    println!("The sum of the 2000th secret numbers is {sum_2000}");

    let bananas = max_bananas(&initial_numbers, 2000);
    println!("You get {bananas} bananas!");

    Ok(())
}

fn max_bananas(initial_numbers: &[u64], n: usize) -> i64 {
    let prices: Box<[Box<[i64]>]> = initial_numbers.iter().map(|i| gen_prices(*i, n)).collect();
    let price_changes: Box<[Box<[i64]>]> = prices
        .iter()
        .map(|price_list| {
            price_list
                .windows(2)
                .map(|win| win[1] - win[0])
                .collect::<Box<[i64]>>()
        })
        .collect();

    let mut total_price_by_sequence: HashMap<&[i64], i64> = HashMap::with_capacity(2048);
    let mut seen_sequences: HashSet<&[i64]> = HashSet::with_capacity(price_changes.len());
    for monkey_i in 0..price_changes.len() {
        seen_sequences.clear();

        for (i, sequence) in price_changes[monkey_i].windows(4).enumerate() {
            if !seen_sequences.contains(&sequence) {
                seen_sequences.insert(sequence);
                *total_price_by_sequence.entry(sequence).or_insert(0) += prices[monkey_i][i + 4];
            }
        }
    }
    total_price_by_sequence.values().max().copied().unwrap_or(0)
}

fn gen_prices(initial_number: u64, n: usize) -> Box<[i64]> {
    let mut prices = Vec::with_capacity(n + 1);
    let mut secret = initial_number;
    for _ in 0..n {
        prices.push((secret % 10) as i64);
        secret = next(secret);
    }
    prices.into_boxed_slice()
}

fn sum_number_n(initial_numbers: &[u64], n: u64) -> u64 {
    initial_numbers
        .iter()
        .copied()
        .map(|i| number_n(i, n))
        .sum()
}

fn number_n(mut prn: u64, n: u64) -> u64 {
    for _ in 0..n {
        prn = next(prn);
    }
    prn
}

const PRUNE_MOD: u64 = 16777216;

fn next(n: u64) -> u64 {
    let n = (n ^ (n * 64)) % PRUNE_MOD;
    let n = (n ^ (n / 32)) % PRUNE_MOD;
    (n ^ (n * 2048)) % PRUNE_MOD
}

fn parse(input: &str) -> Result<Box<[u64]>, String> {
    input
        .lines()
        .map(|line| {
            line.parse::<u64>()
                .map_err(|e| format!("unable to parse line '{line}': {e}"))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn number_n_works_for_example() {
        assert_eq!(number_n(123, 1), 15887950);
        assert_eq!(number_n(123, 2), 16495136);
        assert_eq!(number_n(123, 3), 527345);
        assert_eq!(number_n(123, 4), 704524);
        assert_eq!(number_n(123, 5), 1553684);
        assert_eq!(number_n(123, 6), 12683156);
        assert_eq!(number_n(123, 7), 11100544);
        assert_eq!(number_n(123, 8), 12249484);
        assert_eq!(number_n(123, 9), 7753432);
        assert_eq!(number_n(123, 10), 5908254);

        assert_eq!(number_n(1, 2000), 8685429);
        assert_eq!(number_n(10, 2000), 4700978);
        assert_eq!(number_n(100, 2000), 15273692);
        assert_eq!(number_n(2024, 2000), 8667524);
    }

    #[test]
    fn max_bananas_works_for_example() {
        // given
        let initial_secrets: &[u64] = &[1, 2, 3, 2024];

        // when
        let bananas = max_bananas(initial_secrets, 2000);

        // then
        assert_eq!(bananas, 23);
    }
}
