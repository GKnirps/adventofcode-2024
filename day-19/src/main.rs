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
    let (towels, designs) = parse(&content)?;

    let possible_designs = count_possible_designs(&designs, &towels);
    println!("{possible_designs} designs are possible.");

    let option_count = sum_design_options(&designs, &towels);
    println!("There are {option_count} ways to make designs");

    Ok(())
}

fn sum_design_options(designs: &[&str], towels: &HashSet<&str>) -> u64 {
    let mut cache: HashMap<&str, u64> = HashMap::with_capacity(1024);
    designs
        .iter()
        .map(|design| design_options(design, towels, &mut cache))
        .sum()
}

fn design_options<'a>(
    design: &'a str,
    towels: &HashSet<&str>,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if let Some(n) = cache.get(design) {
        return *n;
    }
    if design.is_empty() {
        return 1;
    }
    let mut n = 0;
    for subdesign in (1..=design.len()).filter_map(|i| design.get(i..)) {
        if design
            .strip_suffix(subdesign)
            .map(|prefix| towels.contains(prefix))
            .unwrap_or(false)
        {
            n += design_options(subdesign, towels, cache);
        }
    }
    cache.insert(design, n);
    n
}

fn count_possible_designs(designs: &[&str], towels: &HashSet<&str>) -> usize {
    designs
        .iter()
        .filter(|design| design_possible(design, towels))
        .count()
}

fn design_possible(design: &str, towels: &HashSet<&str>) -> bool {
    let mut stack: Vec<&str> = Vec::with_capacity(design.len());
    let mut seen: HashSet<&str> = HashSet::with_capacity(design.len());
    stack.push(design);
    while let Some(partial_design) = stack.pop() {
        if partial_design.is_empty() {
            return true;
        }
        for new_partial_design in towels
            .iter()
            .filter_map(|towel| partial_design.strip_prefix(towel))
        {
            if seen.contains(new_partial_design) {
                continue;
            }
            seen.insert(new_partial_design);

            stack.push(new_partial_design);
        }
    }
    false
}

fn parse(input: &str) -> Result<(HashSet<&str>, Box<[&str]>), String> {
    // so many towels… are there any hitchhikers around?
    let (towels, designs) = input
        .split_once("\n\n")
        .ok_or_else(|| "unable to find separation between towels and designs".to_string())?;
    let towels: HashSet<&str> = towels.split(", ").collect();
    let designs: Box<[&str]> = designs.lines().collect();

    Ok((towels, designs))
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"#;

    #[test]
    fn count_possible_designs_works_for_example() {
        // given
        let (towels, designs) = parse(EXAMPLE).expect("expected example input to parse");

        // when
        let count = count_possible_designs(&designs, &towels);

        // then
        assert_eq!(count, 6);
    }

    #[test]
    fn sum_design_options_works_for_example() {
        // given
        let (towels, designs) = parse(EXAMPLE).expect("expected example input to parse");

        // when
        let sum = sum_design_options(&designs, &towels);

        // then
        assert_eq!(sum, 16);
    }
}
