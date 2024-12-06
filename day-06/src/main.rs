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

    let distinct_guard_positions = track_guard(&map).len();
    println!("The guard visits {distinct_guard_positions} tiles before leaving the map");

    let obstructions = obstruction_candidates(&map);
    println!("{obstructions} different positions can be chosen for an obstruction");

    Ok(())
}

fn obstruction_candidates(map: &Map) -> usize {
    // Hello brute force my old friend
    track_guard(map)
        .iter()
        .filter(|obstruction_pos| {
            let mut rocks = map.rocks.clone();
            rocks.insert(**obstruction_pos);
            let updated_map = Map { rocks, ..*map };
            has_loop(&updated_map)
        })
        .count()
}

fn has_loop(map: &Map) -> bool {
    let mut pos = map.guard;
    let mut dir: (i32, i32) = (0, -1);
    let mut visited: HashSet<(i32, i32, i32, i32)> =
        HashSet::with_capacity((map.width * map.height) as usize);

    while pos.0 >= 0 && pos.1 >= 0 && pos.0 < map.width && pos.1 < map.height {
        if visited.contains(&(pos.0, pos.1, dir.0, dir.1)) {
            return true;
        }
        visited.insert((pos.0, pos.1, dir.0, dir.1));
        if map.rocks.contains(&(pos.0 + dir.0, pos.1 + dir.1)) {
            let odir = dir;
            dir.0 = -odir.1;
            dir.1 = odir.0;
        } else {
            pos.0 += dir.0;
            pos.1 += dir.1;
        }
    }
    false
}

fn track_guard(map: &Map) -> HashSet<(i32, i32)> {
    let mut pos = map.guard;
    let mut dir: (i32, i32) = (0, -1);
    let mut visited: HashSet<(i32, i32)> =
        HashSet::with_capacity((map.width * map.height) as usize);

    while pos.0 >= 0 && pos.1 >= 0 && pos.0 < map.width && pos.1 < map.height {
        visited.insert(pos);
        if map.rocks.contains(&(pos.0 + dir.0, pos.1 + dir.1)) {
            let odir = dir;
            dir.0 = -odir.1;
            dir.1 = odir.0;
        } else {
            pos.0 += dir.0;
            pos.1 += dir.1;
        }
    }

    visited
}

#[derive(PartialEq, Eq, Debug)]
struct Map {
    width: i32,
    height: i32,
    // the map is pretty sparse, so we use a HashSet for the rock positions
    rocks: HashSet<(i32, i32)>,
    guard: (i32, i32),
}

fn parse(map: &str) -> Result<Map, String> {
    let width = map
        .lines()
        .next()
        .ok_or_else(|| "expected at least one line".to_string())?
        .len() as i32;
    if !map.lines().all(|line| line.len() as i32 == width) {
        return Err(format!("expected all lines to have length {width}"));
    }
    let height = map.lines().count() as i32;

    let guard = map
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '^')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .next()
        .ok_or_else(|| "could not spot guard in input".to_string())?;

    let rocks: HashSet<(i32, i32)> = map
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect();

    Ok(Map {
        width,
        height,
        rocks,
        guard,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

    #[test]
    fn track_guard_works_for_example() {
        // given
        let map = parse(EXAMPLE).expect("expected exampe input to parse");

        // when
        let positions = track_guard(&map);

        // then
        assert_eq!(positions.len(), 41);
    }

    #[test]
    fn obstruction_candidates_works_for_example() {
        // given
        let map = parse(EXAMPLE).expect("expected exampe input to parse");

        // when
        let obstructions = obstruction_candidates(&map);

        // then
        assert_eq!(obstructions, 6);
    }
}
