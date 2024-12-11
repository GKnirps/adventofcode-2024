#![forbid(unsafe_code)]

use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let initial_stones = parse(&content)?;

    let after_25_blinks = blinks(initial_stones, 25).len();
    println!("after 25 blinks, there are {after_25_blinks} stones");

    Ok(())
}

fn blinks(mut stones: Box<[u128]>, n: u32) -> Box<[u128]> {
    for _ in 0..n {
        stones = blink(&stones);
    }
    stones
}

fn blink(stones: &[u128]) -> Box<[u128]> {
    stones
        .iter()
        .flat_map(|stone| {
            if *stone == 0 {
                [Some(1), None]
            } else if stone.ilog10() % 2 == 1 {
                let div = 10u128.pow((stone.ilog10() + 1) / 2);
                [Some(stone / div), Some(stone % div)]
            } else {
                [Some(stone * 2024), None]
            }
        })
        .flatten()
        .collect()
}

fn parse(input: &str) -> Result<Box<[u128]>, String> {
    input
        .split_whitespace()
        .map(|n| {
            n.parse::<u128>()
                .map_err(|e| format!("unable to parse engraving '{n}': {e}"))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn blinks_works_for_example() {
        // given
        let stones = parse("125 17\n").expect("expect example input to parse");

        // when
        let stones = blinks(stones, 25);

        // then
        assert_eq!(stones.len(), 55312);
    }
}
