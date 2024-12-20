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

    let shortcuts = find_path_with_cheat(&track, start, goal, 2);
    let n: usize = shortcuts
        .iter()
        .filter(|(dist, _)| **dist >= 100)
        .map(|(_, n)| *n)
        .sum();
    println!(
        "There are {n} cheats of up to 2 picosecond length that save at least 100 picoseconds"
    );

    let shortcuts = find_path_with_cheat(&track, start, goal, 20);
    let n: usize = shortcuts
        .iter()
        .filter(|(dist, _)| **dist >= 100)
        .map(|(_, n)| *n)
        .sum();
    println!(
        "There are {n} cheats of up to 20 picosecond length that save at least 100 picoseconds"
    );

    Ok(())
}

fn find_path_with_cheat(
    track: &Track,
    start: V2,
    goal: V2,
    cheat_time: u64,
) -> HashMap<u64, usize> {
    let uncheated_dist = find_path(track, start);
    let uncheated_shortest_path = match uncheated_dist.get(&goal) {
        Some(d) => d,
        None => return HashMap::new(),
    };
    let uncheated_dist_inverse = find_path(track, goal);
    let mut cheated_dists: HashMap<u64, usize> = HashMap::with_capacity(1024);

    for (i, tile) in track.tiles.iter().enumerate() {
        if *tile == Tile::Floor {
            let x = i % track.width;
            let y = i / track.width;
            let uncheated_from = match uncheated_dist.get(&(x, y)) {
                Some(d) => d,
                None => {
                    continue;
                }
            };
            for cheated_distance in 2..=cheat_time {
                for dy in 0..=cheated_distance {
                    let dx = cheated_distance - dy;
                    for (cheat_x, cheat_y) in cheatable_tiles(track, x, y, dx as usize, dy as usize)
                        .iter()
                        .filter_map(|t| *t)
                    {
                        if let Some(from_cheated) = uncheated_dist_inverse.get(&(cheat_x, cheat_y))
                        {
                            let total_cheated_distance =
                                uncheated_from + cheated_distance + from_cheated;
                            if total_cheated_distance < *uncheated_shortest_path {
                                *cheated_dists
                                    .entry(uncheated_shortest_path - total_cheated_distance)
                                    .or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    cheated_dists
}

fn cheatable_tiles(track: &Track, x: usize, y: usize, dx: usize, dy: usize) -> [Option<V2>; 4] {
    let mut result: [Option<V2>; 4] = [None; 4];
    if dx < x {
        if dy < y {
            let cheat_x = x - dx;
            let cheat_y = y - dy;
            if track.get((cheat_x, cheat_y)) == Tile::Floor {
                result[0] = Some((cheat_x, cheat_y));
            }
        }
        if y + dy < track.height - 1 && dy != 0 {
            let cheat_x = x - dx;
            let cheat_y = y + dy;
            if track.get((cheat_x, cheat_y)) == Tile::Floor {
                result[1] = Some((cheat_x, cheat_y));
            }
        }
    }
    if dx + x < track.width - 1 && dx != 0 {
        if dy < y {
            let cheat_x = x + dx;
            let cheat_y = y - dy;
            if track.get((cheat_x, cheat_y)) == Tile::Floor {
                result[2] = Some((cheat_x, cheat_y));
            }
        }
        if y + dy < track.height - 1 && dy != 0 {
            let cheat_x = x + dx;
            let cheat_y = y + dy;
            if track.get((cheat_x, cheat_y)) == Tile::Floor {
                result[3] = Some((cheat_x, cheat_y));
            }
        }
    }
    result
}

fn find_path(track: &Track, start: V2) -> HashMap<V2, u64> {
    let mut seen: HashMap<V2, u64> = HashMap::with_capacity(track.tiles.len());
    let mut queue: VecDeque<(usize, usize, u64)> = VecDeque::with_capacity(track.tiles.len());
    queue.push_back((start.0, start.1, 0));
    while let Some((x, y, dist)) = queue.pop_front() {
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
    seen
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
        let shortcuts = find_path_with_cheat(&track, start, goal, 2);

        // then
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

    #[test]
    fn find_path_with_cheat_works_with_20_ps_for_example() {
        // given
        let (track, start, goal) = parse(EXAMPLE).expect("expected example input to parse");

        // when
        let shortcuts = find_path_with_cheat(&track, start, goal, 20);

        // then
        let shortcuts_50: HashMap<u64, usize> = shortcuts
            .iter()
            .filter(|(dist, _)| **dist >= 50)
            .map(|(d, n)| (*d, *n))
            .collect();
        println!("{shortcuts_50:?}");
        assert_eq!(shortcuts_50.len(), 14);
        assert_eq!(shortcuts_50.get(&50), Some(&32));
        assert_eq!(shortcuts_50.get(&52), Some(&31));
        assert_eq!(shortcuts_50.get(&54), Some(&29));
        assert_eq!(shortcuts_50.get(&56), Some(&39));
        assert_eq!(shortcuts_50.get(&58), Some(&25));
        assert_eq!(shortcuts_50.get(&60), Some(&23));
        assert_eq!(shortcuts_50.get(&62), Some(&20));
        assert_eq!(shortcuts_50.get(&64), Some(&19));
        assert_eq!(shortcuts_50.get(&66), Some(&12));
        assert_eq!(shortcuts_50.get(&68), Some(&14));
        assert_eq!(shortcuts_50.get(&70), Some(&12));
        assert_eq!(shortcuts_50.get(&72), Some(&22));
        assert_eq!(shortcuts_50.get(&74), Some(&4));
        assert_eq!(shortcuts_50.get(&76), Some(&3));
    }
}
