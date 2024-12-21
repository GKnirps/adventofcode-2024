#![forbid(unsafe_code)]

use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let codes = parse(&content).ok_or_else(|| "unable to parse input".to_string())?;

    let complexity = code_complexity(&codes);
    println!("The sum of the complexities of the five codes is {complexity}");

    Ok(())
}

fn path_valid(graph: &[&[(usize, Dir)]], path: &[Dir], from: usize) -> bool {
    let mut pos = from;
    for dir in path {
        if let Some((nextpos, _)) = graph[pos].iter().find(|(_, gdir)| gdir == dir) {
            pos = *nextpos;
        } else {
            return false;
        }
    }
    true
}
fn code_complexity(codes: &[Box<[usize]>]) -> usize {
    let numpad_paths: HashMap<usize, Paths> = (0..=10)
        .map(|from| {
            let mut paths = find_paths(NUMPAD_GRAPH, from);
            for path in &mut paths {
                path.sort_unstable();
                if !path_valid(NUMPAD_GRAPH, path, from) {
                    path.sort_unstable_by(|a, b| b.cmp(a))
                }
            }
            (from, paths)
        })
        .collect();
    let dirpad_paths: HashMap<usize, Paths> = (0..=4)
        .map(|from| {
            let mut paths = find_paths(DIRPAD_GRAPH, from);
            for path in &mut paths {
                path.sort_unstable();
                if !path_valid(DIRPAD_GRAPH, path, from) {
                    path.sort_unstable_by(|a, b| b.cmp(a))
                }
            }
            (from, paths)
        })
        .collect();

    codes
        .iter()
        .map(|code| {
            let dirs = get_dirs(code, &numpad_paths, &dirpad_paths);
            let numeric_part: usize = code
                .iter()
                .skip_while(|c| **c == 0)
                .take_while(|c| **c != 10)
                .fold(0usize, |v, c| v * 10 + c);
            dirs.len() * numeric_part
        })
        .sum()
}

fn get_dirs(
    code: &[usize],
    numpad_paths: &HashMap<usize, Paths>,
    dirpad_paths: &HashMap<usize, Paths>,
) -> Vec<usize> {
    let mut numpad_dirs: Vec<usize> = Vec::with_capacity(5 * code.len());
    let mut pos = 10; // starting at button A
    for c in code {
        for dir in &numpad_paths
            .get(&pos)
            .expect("expected all numpad paths in map")[*c]
        {
            numpad_dirs.push(dir.id());
        }
        numpad_dirs.push(4);
        pos = *c;
    }
    let numpad_dirs = numpad_dirs;

    let mut dirpad1_dirs: Vec<usize> = Vec::with_capacity(3 * numpad_dirs.len());
    let mut pos = 4; // starting at button A
    for c in &numpad_dirs {
        for dir in &dirpad_paths
            .get(&pos)
            .expect("expected all dirpad paths in map")[*c]
        {
            dirpad1_dirs.push(dir.id());
        }
        dirpad1_dirs.push(4);
        pos = *c;
    }
    let dirpad1_dirs = dirpad1_dirs;

    let mut dirpad2_dirs: Vec<usize> = Vec::with_capacity(3 * dirpad1_dirs.len());
    let mut pos = 4; // starting at button A
    for c in &dirpad1_dirs {
        for dir in &dirpad_paths
            .get(&pos)
            .expect("expected all dirpad paths in map")[*c]
        {
            dirpad2_dirs.push(dir.id());
        }
        dirpad2_dirs.push(4);
        pos = *c;
    }
    dirpad2_dirs
}

type Paths = Box<[Box<[Dir]>]>;

fn find_paths(graph: &[&[(usize, Dir)]], from: usize) -> Paths {
    let mut seen: HashMap<usize, Option<(usize, Dir)>> = HashMap::with_capacity(graph.len());
    let mut queue: VecDeque<(usize, Option<(usize, Dir)>)> =
        VecDeque::with_capacity(graph.len() * 4);
    queue.push_back((from, None));

    while let Some((button, dir)) = queue.pop_front() {
        if seen.contains_key(&button) {
            continue;
        }
        seen.insert(button, dir);
        for (neighbour, dir) in graph[button] {
            queue.push_back((*neighbour, Some((button, *dir))));
        }
    }
    (0..graph.len())
        .map(|button| {
            let mut dirs: Vec<Dir> = Vec::with_capacity(5);
            let mut b = button;
            while let Some(Some((pred, dir))) = seen.get(&b) {
                dirs.push(*dir);
                b = *pred;
            }
            dirs.reverse();
            dirs.into_boxed_slice()
        })
        .collect()
}

fn parse(input: &str) -> Option<Box<[Box<[usize]>]>> {
    input.lines().map(parse_code).collect()
}

fn parse_code(line: &str) -> Option<Box<[usize]>> {
    line.chars()
        .map(|c| c.to_digit(11).map(|d| d as usize))
        .collect()
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Dir {
    Left,
    Down,
    Right,
    Up,
}

impl Dir {
    const fn id(self) -> usize {
        match self {
            Dir::Up => 0,
            Dir::Right => 1,
            Dir::Down => 2,
            Dir::Left => 3,
        }
    }
}

static NUMPAD_GRAPH: &[&[(usize, Dir)]] = &[
    &[(2, Dir::Up), (10, Dir::Right)], // 0
    &[(4, Dir::Up), (2, Dir::Right)],  // 1
    &[
        (5, Dir::Up),
        (3, Dir::Right),
        (0, Dir::Down),
        (1, Dir::Left),
    ], // 2
    &[(6, Dir::Up), (10, Dir::Down), (2, Dir::Left)], // 3
    &[(7, Dir::Up), (5, Dir::Right), (1, Dir::Down)], // 4
    &[
        (6, Dir::Right),
        (8, Dir::Up),
        (2, Dir::Down),
        (4, Dir::Left),
    ], // 5
    &[(9, Dir::Up), (3, Dir::Down), (5, Dir::Left)], // 6
    &[(8, Dir::Right), (4, Dir::Down)], // 7
    &[(9, Dir::Right), (5, Dir::Down), (7, Dir::Left)], // 8
    &[(8, Dir::Left), (6, Dir::Down)], // 9
    &[(3, Dir::Up), (0, Dir::Left)],   // A
];

static DIRPAD_GRAPH: &[&[(usize, Dir)]] = &[
    &[(Dir::Down.id(), Dir::Down), (4, Dir::Right)], // up button
    &[(Dir::Down.id(), Dir::Left), (4, Dir::Up)],    // right button
    &[
        (Dir::Left.id(), Dir::Left),
        (Dir::Up.id(), Dir::Up),
        (Dir::Right.id(), Dir::Right),
    ], // down
    &[(Dir::Down.id(), Dir::Right)],                 // left button
    &[(Dir::Right.id(), Dir::Down), (Dir::Up.id(), Dir::Left)], // A
];

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"029A
980A
179A
456A
379A
"#;

    #[test]
    fn code_complexity_works_for_example() {
        // given
        let codes = parse(EXAMPLE).expect("expected example input to parse");

        // when
        let complexity = code_complexity(&codes);

        // then
        assert_eq!(complexity, 126384);
    }
}
