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
    let (track, start, goal) = parse(&content)?;

    let shortcuts = find_path_with_cheat(&track, start, goal);
    let n: usize = shortcuts
        .iter()
        .filter(|(dist, _)| **dist >= 100)
        .map(|(_, n)| *n)
        .sum();
    println!("There are {n} cheats that save at least 100 picosedonds");

    Ok(())
}

fn find_path_with_cheat(track: &Track, start: V2, goal: V2) -> HashMap<u64, usize> {
    let uncheated_dist = match find_path(track, start, goal) {
        Some(dist) => dist,
        None => {
            return HashMap::new();
        }
    };
    let mut shortcuts: HashMap<u64, usize> = HashMap::with_capacity(track.tiles.len());
    // brute force, but still faster than the fancy algorithm I tried first
    for dist in (1..track.height - 1)
        .flat_map(|y| (1..track.width - 1).map(move |x| (x, y)))
        .filter_map(|(x, y)| {
            if track.get((x, y)) == Tile::Wall {
                let mut cheat_track = track.clone();
                cheat_track.tiles[x + y * track.width] = Tile::Floor;
                Some(cheat_track)
            } else {
                None
            }
        })
        .filter_map(|track| find_path(&track, start, goal))
    {
        if uncheated_dist > dist {
            *shortcuts.entry(uncheated_dist - dist).or_insert(0) += 1;
        }
    }
    shortcuts
}

fn find_path(track: &Track, start: V2, goal: V2) -> Option<u64> {
    let mut seen: HashMap<V2, u64> = HashMap::with_capacity(track.tiles.len());
    let mut queue: VecDeque<(usize, usize, u64)> = VecDeque::with_capacity(track.tiles.len());
    queue.push_back((start.0, start.1, 0));
    while let Some((x, y, dist)) = queue.pop_front() {
        if x == goal.0 && y == goal.1 {
            return Some(dist);
        }
        if seen.contains_key(&(x, y)) {
            continue;
        }
        seen.insert((x, y), dist);
        if track.get((x - 1, y)) == Tile::Floor {
            queue.push_back((x - 1, y, dist + 1));
        }
        if track.get((x, y - 1)) == Tile::Floor {
            queue.push_back((x, y - 1, dist + 1));
        }
        if track.get((x + 1, y)) == Tile::Floor {
            queue.push_back((x + 1, y, dist + 1));
        }
        if track.get((x, y + 1)) == Tile::Floor {
            queue.push_back((x, y + 1, dist + 1));
        }
    }
    None
}

type V2 = (usize, usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Tile {
    Wall,
    Floor,
}

#[derive(Clone, Debug)]
struct Track {
    tiles: Box<[Tile]>,
    width: usize,
    height: usize,
}

impl Track {
    fn get(&self, (x, y): V2) -> Tile {
        self.tiles[x + y * self.width]
    }
}

fn parse(input: &str) -> Result<(Track, V2, V2), String> {
    let width = input
        .lines()
        .next()
        .ok_or_else(|| "no lines on track".to_owned())?
        .len();
    if !input.lines().all(|line| line.len() == width) {
        return Err("not all lines in the track have the same length".to_owned());
    }
    let height = input.lines().count();
    let tiles: Box<[Tile]> = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '#' => Ok(Tile::Wall),
            '.' | 'E' | 'S' => Ok(Tile::Floor),
            _ => Err(format!("unknown tile: '{c}'")),
        })
        .collect::<Result<_, _>>()?;
    if (0..width).any(|x| tiles[x] != Tile::Wall || tiles[x + (height - 1) * width] != Tile::Wall)
        || (0..height)
            .any(|y| tiles[y * width] != Tile::Wall || tiles[width - 1 + y * width] != Tile::Wall)
    {
        return Err("no wall around the track".to_string());
    }
    let start: V2 = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == 'S')
                .map(move |(x, _)| (x, y))
        })
        .next()
        .ok_or_else(|| "unable to find starting position".to_string())?;
    let goal: V2 = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == 'E')
                .map(move |(x, _)| (x, y))
        })
        .next()
        .ok_or_else(|| "unable to find goal position".to_string())?;

    Ok((
        Track {
            width,
            height,
            tiles,
        },
        start,
        goal,
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"#;

    #[test]
    fn find_path_with_cheat_works_for_example() {
        // given
        let (track, start, goal) = parse(EXAMPLE).expect("expected example input to parse");

        // when
        let shortcuts = find_path_with_cheat(&track, start, goal);

        // then
        println!("{shortcuts:?}");
        assert_eq!(shortcuts.len(), 11);
        assert_eq!(shortcuts.get(&2), Some(&14));
        assert_eq!(shortcuts.get(&4), Some(&14));
        assert_eq!(shortcuts.get(&6), Some(&2));
        assert_eq!(shortcuts.get(&8), Some(&4));
        assert_eq!(shortcuts.get(&10), Some(&2));
        assert_eq!(shortcuts.get(&12), Some(&3));
        assert_eq!(shortcuts.get(&20), Some(&1));
        assert_eq!(shortcuts.get(&36), Some(&1));
        assert_eq!(shortcuts.get(&38), Some(&1));
        assert_eq!(shortcuts.get(&40), Some(&1));
        assert_eq!(shortcuts.get(&64), Some(&1));
    }
}
