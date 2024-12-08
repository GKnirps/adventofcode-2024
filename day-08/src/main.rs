use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let map = parse(&content)?;

    let antinodes = find_antinodes(&map);
    println!(
        "{} unique locations within the map bounds contain an antinode",
        antinodes.len()
    );

    Ok(())
}

fn find_antinodes(map: &Map) -> HashSet<(i64, i64)> {
    let mut antinodes: HashSet<(i64, i64)> = HashSet::with_capacity(map.antennas.len() * 16);
    for antennas in map.antennas.values() {
        for (i, (x1, y1)) in antennas.iter().enumerate() {
            for (x2, y2) in &antennas[i + 1..] {
                let dx = x2 - x1;
                let dy = y2 - y1;
                let ax1 = x2 + dx;
                let ay1 = y2 + dy;
                if ax1 >= 0 && ay1 >= 0 && ax1 < map.width && ay1 < map.height {
                    antinodes.insert((ax1, ay1));
                }
                let ax2 = x1 - dx;
                let ay2 = y1 - dy;
                if ax2 >= 0 && ay2 >= 0 && ax2 < map.width && ay2 < map.height {
                    antinodes.insert((ax2, ay2));
                }
            }
        }
    }
    antinodes
}

#[derive(Clone, Debug)]
struct Map {
    width: i64,
    height: i64,
    antennas: HashMap<char, Vec<(i64, i64)>>,
}

fn parse(content: &str) -> Result<Map, String> {
    let width = content
        .lines()
        .next()
        .ok_or_else(|| "expected non-empty input".to_string())?
        .len() as i64;
    if !content.lines().all(|line| line.len() as i64 == width) {
        return Err("not all lines have the same length".to_owned());
    }
    let height = content.lines().count() as i64;

    let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::with_capacity(height as usize * 5);
    for (x, y, a) in content
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, a)| (x, y, a)))
        .filter(|(_, _, a)| *a != '.')
    {
        antennas
            .entry(a)
            .or_insert(Vec::with_capacity(16))
            .push((x as i64, y as i64));
    }

    Ok(Map {
        width,
        height,
        antennas,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;

    #[test]
    fn find_antinodes_works_for_example() {
        // given
        let map = parse(EXAMPLE).expect("expected example input to parse");

        // when
        let antinodes = find_antinodes(&map);

        // then
        assert_eq!(antinodes.len(), 14);
    }
}
