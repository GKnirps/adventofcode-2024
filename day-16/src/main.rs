#![forbid(unsafe_code)]

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let (maze, start, goal) = parse(&content)?;

    if let Some(score) = winning_score(&maze, start, goal) {
        println!("The lowest score is {score}");
    } else {
        println!("There is no path to the goal.");
    }

    Ok(())
}

fn winning_score(maze: &Maze, start: V2, goal: V2) -> Option<u32> {
    let mut queue: BinaryHeap<Candidate> = BinaryHeap::with_capacity(maze.width * maze.height);
    let mut seen: HashSet<(V2, Dir)> = HashSet::with_capacity(maze.width * maze.height);
    queue.push(Candidate {
        cost: 0,
        pos: start,
        dir: Dir::East,
    });

    while let Some(Candidate { cost, pos, dir }) = queue.pop() {
        let (x, y) = pos;
        if pos == goal {
            return Some(cost);
        }
        if seen.contains(&(pos, dir)) {
            continue;
        }
        seen.insert((pos, dir));
        let neighbour = match dir {
            Dir::East => (x + 1, y),
            Dir::South => (x, y + 1),
            Dir::West => (x - 1, y),
            Dir::North => (x, y - 1),
        };
        if maze.get(neighbour) == Tile::Floor {
            queue.push(Candidate {
                cost: cost + 1,
                pos: neighbour,
                dir,
            });
        }
        queue.push(Candidate {
            cost: cost + 1000,
            pos,
            dir: dir.rot(),
        });
        queue.push(Candidate {
            cost: cost + 1000,
            pos,
            dir: dir.rot_counter(),
        });
    }
    None
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Candidate {
    cost: u32,
    pos: V2,
    dir: Dir,
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then(self.pos.0.cmp(&other.pos.0))
            .then(self.pos.1.cmp(&other.pos.1))
            .then(self.dir.cmp(&other.dir))
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Dir {
    East,
    South,
    West,
    North,
}

impl Dir {
    fn rot(self) -> Self {
        match self {
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
            Dir::North => Dir::East,
        }
    }
    fn rot_counter(self) -> Self {
        match self {
            Dir::East => Dir::North,
            Dir::North => Dir::West,
            Dir::West => Dir::South,
            Dir::South => Dir::East,
        }
    }
}

type V2 = (usize, usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Tile {
    Wall,
    Floor,
}

#[derive(Clone, Debug)]
struct Maze {
    tiles: Box<[Tile]>,
    width: usize,
    height: usize,
}

impl Maze {
    fn get(&self, (x, y): V2) -> Tile {
        self.tiles[x + y * self.width]
    }
}

fn parse(input: &str) -> Result<(Maze, V2, V2), String> {
    let width = input
        .lines()
        .next()
        .ok_or_else(|| "no lines in maze".to_owned())?
        .len();
    if !input.lines().all(|line| line.len() == width) {
        return Err("not all lines in the maze have the same length".to_owned());
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
        return Err("no wall around the maze".to_string());
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
        Maze {
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

    static EXAMPLE_1: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"#;

    #[test]
    fn winning_score_works_for_example_1() {
        // given
        let (maze, start, goal) = parse(EXAMPLE_1).expect("expected example input to parse");

        // when
        let score = winning_score(&maze, start, goal);

        // then
        assert_eq!(score, Some(7036));
    }
}
