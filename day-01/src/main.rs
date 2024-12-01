use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;

    let ids = parse(&content)?;

    println!("The sum of differences is: {}", difference_sum(&ids));

    Ok(())
}

fn difference_sum(ids: &[(u32, u32)]) -> u32 {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = ids.iter().copied().unzip();

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right)
        .map(|(l, r)| l.max(&r) - l.min(&r))
        .sum()
}

fn parse(content: &str) -> Result<Box<[(u32, u32)]>, String> {
    content.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<(u32, u32), String> {
    let (left, right) = line
        .split_once(" ")
        .ok_or_else(|| format!("no whitespace in line '{line}'"))?;
    let left = left
        .trim()
        .parse::<u32>()
        .map_err(|e| format!("unable to parse left part of line '{line}': {e}"))?;
    let right = right
        .trim()
        .parse::<u32>()
        .map_err(|e| format!("unable to parse right part of line '{line}': {e}"))?;
    Ok((left, right))
}
