#![forbid(unsafe_code)]

use std::collections::HashSet;
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
    println!("{possible_designs} designs are possible");

    Ok(())
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
}
