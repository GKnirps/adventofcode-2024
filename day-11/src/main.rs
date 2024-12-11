#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let initial_stones = parse(&content)?;

    let after_25_blinks = dynamic_blinks(&initial_stones, 25);
    println!("after 25 blinks, there are {after_25_blinks} stones");

    let after_75_blinks = dynamic_blinks(&initial_stones, 75);
    println!("after 75 blinks, there are {after_75_blinks} stones");

    Ok(())
}

fn dynamic_blinks(stones: &[u128], n: u32) -> u64 {
    let mut cache: HashMap<(u128, u32), u64> = HashMap::with_capacity(1024);
    let mut stone_count = 0;

    for stone in stones {
        stone_count += dynamic_blinks_internal(*stone, n, &mut cache);
    }
    stone_count
}

fn dynamic_blinks_internal(stone: u128, n: u32, cache: &mut HashMap<(u128, u32), u64>) -> u64 {
    if n == 0 {
        return 1;
    }
    if let Some(stones) = cache.get(&(stone, n)) {
        return *stones;
    }
    let stones = if stone == 0 {
        dynamic_blinks_internal(1, n - 1, cache)
    } else if stone.ilog10() % 2 == 1 {
        let div = 10u128.pow((stone.ilog10() + 1) / 2);
        dynamic_blinks_internal(stone / div, n - 1, cache)
            + dynamic_blinks_internal(stone % div, n - 1, cache)
    } else {
        dynamic_blinks_internal(stone * 2024, n - 1, cache)
    };
    cache.insert((stone, n), stones);
    stones
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
    fn dynamic_blinks_works_for_example() {
        // given
        let stones = parse("125 17\n").expect("expect example input to parse");

        // when
        let stones = dynamic_blinks(&stones, 25);

        // then
        assert_eq!(stones, 55312);
    }
}
