use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let map = parse(&content)?;

    let scores = find_all_valid_trails(&map);
    println!("The sum of scores of all trailheads is {scores}");

    Ok(())
}

fn find_all_valid_trails(map: &Map) -> usize {
    map.tiles
        .iter()
        .enumerate()
        .map(|(i, height)| {
            if *height == 0 {
                find_valid_trails_from(map, i % map.width, i / map.width)
            } else {
                0
            }
        })
        .sum()
}

fn find_valid_trails_from(map: &Map, start_x: usize, start_y: usize) -> usize {
    if start_x >= map.width || start_y >= map.height {
        return 0;
    }
    let mut stack: Vec<(usize, usize)> = Vec::with_capacity(256);
    stack.push((start_x, start_y));
    let mut tops: HashSet<(usize, usize)> = HashSet::with_capacity(256);

    while let Some((x, y)) = stack.pop() {
        let height = map.get(x, y).unwrap();
        if height == 9 {
            tops.insert((x, y));
            continue;
        }
        if x > 0 && map.get(x - 1, y) == Some(height + 1) {
            stack.push((x - 1, y));
        }
        if y > 0 && map.get(x, y - 1) == Some(height + 1) {
            stack.push((x, y - 1));
        }
        if map.get(x + 1, y) == Some(height + 1) {
            stack.push((x + 1, y));
        }
        if map.get(x, y + 1) == Some(height + 1) {
            stack.push((x, y + 1));
        }
    }

    tops.len()
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Map {
    width: usize,
    height: usize,
    tiles: Box<[u8]>,
}

impl Map {
    fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width || y >= self.height {
            None
        } else {
            self.tiles.get(x + y * self.width).copied()
        }
    }
}

fn parse(input: &str) -> Result<Map, String> {
    let width = input
        .lines()
        .next()
        .ok_or_else(|| "no rows in input".to_string())?
        .len();
    if !input.lines().all(|line| line.len() == width) {
        return Err("not all lines have the same length".to_string());
    }
    let height = input.lines().count();
    let tiles: Box<[u8]> = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| {
            c.to_digit(10)
                .map(|d| d as u8)
                .ok_or_else(|| format!("unable to parse map tile '{c}'"))
        })
        .collect::<Result<_, _>>()?;

    Ok(Map {
        width,
        height,
        tiles,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;

    #[test]
    fn find_all_valid_trails_works_for_example() {
        // given
        let map = parse(EXAMPLE).expect("expected example input to parse");

        // when
        let scores = find_all_valid_trails(&map);

        // then
        assert_eq!(scores, 36);
    }
}
