use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let (rules, updates) = parse(&content)?;

    let checksum = ordered_checksum(&updates, &rules);
    println!("The sum of the middle page number in correctly sorted updates is {checksum}");

    Ok(())
}

fn ordered_checksum(updates: &[Update], rules: &HashMap<u32, Vec<u32>>) -> u32 {
    updates
        .iter()
        .filter(|update| !update.is_empty() && is_update_sorted(update, rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn is_update_sorted(update: &[u32], rules: &HashMap<u32, Vec<u32>>) -> bool {
    let mut seen: HashSet<u32> = HashSet::with_capacity(update.len());
    for page in update {
        if let Some(later_pages) = rules.get(page) {
            if later_pages
                .iter()
                .any(|later_page| seen.contains(later_page))
            {
                return false;
            }
        }
        seen.insert(*page);
    }
    true
}

fn parse(input: &str) -> Result<(HashMap<u32, Vec<u32>>, Box<[Update]>), String> {
    let (rules, updates) = input
        .split_once("\n\n")
        .ok_or_else(|| "unable to split rules from updated".to_string())?;
    let mut rule_map: HashMap<u32, Vec<u32>> = HashMap::with_capacity(128);
    for rule in rules.lines().map(parse_rule) {
        let (left, right) = rule?;
        let entry = rule_map.entry(left).or_insert(Vec::with_capacity(8));
        entry.push(right);
    }
    let updates: Box<[Update]> = updates
        .lines()
        .map(parse_update)
        .collect::<Result<_, String>>()?;
    Ok((rule_map, updates))
}

type Rule = (u32, u32);
fn parse_rule(line: &str) -> Result<Rule, String> {
    let (left, right) = line
        .split_once('|')
        .ok_or_else(|| format!("unable to split rule '{line}'"))?;
    let left: u32 = left
        .parse()
        .map_err(|e| format!("unable to parse left side of rule '{line}': {e}"))?;
    let right: u32 = right
        .parse()
        .map_err(|e| format!("unable to parse right side of rule '{line}': {e}"))?;

    Ok((left, right))
}

type Update = Box<[u32]>;
fn parse_update(line: &str) -> Result<Update, String> {
    line.split(',')
        .map(|page| {
            page.parse::<u32>()
                .map_err(|e| format!("unable to parse page number '{page}': {e}"))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

    #[test]
    fn ordered_checksum_works_for_example() {
        // given
        let (rules, updates) = parse(EXAMPLE).expect("expected example input to parse");

        // when
        let sum = ordered_checksum(&updates, &rules);

        // then
        assert_eq!(sum, 143);
    }
}
