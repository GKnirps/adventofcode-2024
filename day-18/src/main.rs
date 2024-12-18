#![forbid(unsafe_code)]

use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let bytes = parse(&content)?;

    if bytes.len() < 1024 {
        return Err(format!(
            "cannot simulate 1024 bytes falling: there are only {} bytes",
            bytes.len()
        ));
    }
    if let Some(dist) = shortest_path_after_bytes(&bytes[..1024], 71) {
        println!("After 1024 bytes, the  shortest path is {dist} steps long");
    } else {
        println!("No path to bottom right corner.");
    }

    Ok(())
}

fn shortest_path_after_bytes(bytes: &[(u64, u64)], width: u64) -> Option<u64> {
    let corrupted: HashSet<(u64, u64)> = bytes.iter().copied().collect();
    let mut queue: VecDeque<(u64, u64, u64)> = VecDeque::with_capacity((width * width) as usize);
    let mut seen: HashSet<(u64, u64)> = HashSet::with_capacity((width * width) as usize);
    queue.push_back((0, 0, 0));
    while let Some((x, y, dist)) = queue.pop_front() {
        if seen.contains(&(x, y)) {
            continue;
        }
        seen.insert((x, y));
        if x == width - 1 && y == width - 1 {
            return Some(dist);
        }
        if x > 0 && !corrupted.contains(&(x - 1, y)) {
            queue.push_back((x - 1, y, dist + 1));
        }
        if y > 0 && !corrupted.contains(&(x, y - 1)) {
            queue.push_back((x, y - 1, dist + 1));
        }
        if x < width - 1 && !corrupted.contains(&(x + 1, y)) {
            queue.push_back((x + 1, y, dist + 1));
        }
        if y < width - 1 && !corrupted.contains(&(x, y + 1)) {
            queue.push_back((x, y + 1, dist + 1));
        }
    }
    None
}

fn parse(input: &str) -> Result<Box<[(u64, u64)]>, String> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line
                .split_once(',')
                .ok_or_else(|| format!("unable to split coordinated '{line}'"))?;
            let x: u64 = x
                .parse()
                .map_err(|e| format!("unable to split x coordinate in line '{line}': {e}"))?;
            let y: u64 = y
                .parse()
                .map_err(|e| format!("unable to split y coordinate in line '{line}': {e}"))?;
            Ok((x, y))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"#;

    #[test]
    fn shortest_path_after_bytes_works_for_example() {
        // given
        let bytes = parse(EXAMPLE).expect("expeced example input to parse");

        // when
        let dist = shortest_path_after_bytes(&bytes[..12], 7);

        // then
        assert_eq!(dist, Some(22));
    }
}
