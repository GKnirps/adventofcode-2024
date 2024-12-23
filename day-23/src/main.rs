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
    let connections = parse(&content)?;

    let candidates = count_chief_historian_candidates(&connections);
    println!("{candidates} sets of three interconnected computers contain at least one computer with a name that starts with t");

    Ok(())
}

fn count_chief_historian_candidates(connections: &Connections) -> usize {
    list_groups(connections)
        .iter()
        .filter(|group| group.iter().any(|pc| pc.starts_with('t')))
        .count()
}

fn list_groups<'a>(connections: &Connections<'a>) -> HashSet<[&'a str; 3]> {
    let mut groups: HashSet<[&str; 3]> = HashSet::with_capacity(connections.len() * 3);
    for (a, others) in connections {
        for (i, b) in others.iter().enumerate() {
            for c in &others[i + 1..] {
                if let Some(b_conns) = connections.get(b) {
                    if b_conns.contains(c) {
                        let mut group: [&str; 3] = [a, b, c];
                        group.sort_unstable();
                        groups.insert(group);
                    }
                }
            }
        }
    }
    groups
}

type Connections<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse(input: &str) -> Result<Connections, String> {
    let mut connections: Connections = HashMap::with_capacity(input.len() / 6);
    for conn in input.lines().map(|line| {
        line.split_once('-')
            .ok_or_else(|| format!("unable to split line '{line}'"))
    }) {
        let (a, b) = conn?;
        connections
            .entry(a)
            .or_insert(Vec::with_capacity(16))
            .push(b);
        connections
            .entry(b)
            .or_insert(Vec::with_capacity(16))
            .push(a);
    }
    Ok(connections)
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"#;

    #[test]
    fn count_chief_historian_candidates_works_for_example() {
        // given
        let connections = parse(EXAMPLE).expect("expected example input to parse");

        // when
        let count = count_chief_historian_candidates(&connections);

        // then
        assert_eq!(count, 7);
    }
}
