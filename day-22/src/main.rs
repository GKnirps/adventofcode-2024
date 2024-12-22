#![forbid(unsafe_code)]

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

    Ok(())
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
}
