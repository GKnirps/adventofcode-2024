#![forbid(unsafe_code)]

use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let (locks, keys) = parse(&content)?;

    let pairs = count_lock_key_pairs(&locks, &keys);
    println!("There are {pairs} lock/key pairs that fit together without overlapping");

    Ok(())
}

fn count_lock_key_pairs(locks: &[u64], keys: &[u64]) -> usize {
    locks
        .iter()
        .map(|lock| keys.iter().filter(|key| *key & *lock == 0).count())
        .sum()
}

fn parse(input: &str) -> Result<(Vec<u64>, Vec<u64>), String> {
    let mut locks: Vec<u64> = Vec::with_capacity(64);
    let mut keys: Vec<u64> = Vec::with_capacity(64);

    for block in input.split("\n\n") {
        if block.starts_with('#') {
            locks.push(parse_block(block)?);
        } else {
            keys.push(parse_block(block)?);
        }
    }
    Ok((locks, keys))
}

fn parse_block(block: &str) -> Result<u64, String> {
    if block.len() > 42 {
        // 7 rows × (5 columns + line breaks), at most blocks the last linebreak
        // split away
        return Err(format!("unexpected block length: {}", block.len()));
    }
    block
        .chars()
        .filter(|c| *c != '\n')
        .try_fold(0u64, |block, c| match c {
            '#' => Ok((block << 1) | 1),
            '.' => Ok(block << 1),
            _ => Err(format!("unexpected character in input: {c}")),
        })
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"#;

    #[test]
    fn count_lock_key_pairs_works_for_example() {
        // given
        let (locks, keys) = parse(EXAMPLE).expect("expected example input to parse");

        // when
        let fits = count_lock_key_pairs(&locks, &keys);

        // then
        assert_eq!(fits, 3);
    }
}
