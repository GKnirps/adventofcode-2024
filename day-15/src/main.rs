#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let (warehouse, instructions, start_x, start_y) = parse(&content)?;
    let wide_warehouse = widen_warehouse(&warehouse);

    let vandalized_warehouse = apply_instructions(warehouse, &instructions, start_x, start_y);
    let checksum = gps_sum(&vandalized_warehouse);
    println!("The sum of all boxes' GPS coordinates is {checksum}");

    let vandalized_warehouse =
        apply_instructions_wide(wide_warehouse, &instructions, start_x * 2, start_y);
    let checksum = gps_sum_wide(&vandalized_warehouse);
    println!("The sum of all boxes'GPS coordinates in the wide warehouse is {checksum}");

    Ok(())
}

fn apply_instructions_wide(
    mut warehouse: WideWarehouse,
    instructions: &[Dir],
    start_x: isize,
    start_y: isize,
) -> WideWarehouse {
    let mut rx = start_x;
    let mut ry = start_y;
    for dir in instructions {
        move_robot_wide(&mut warehouse, &mut rx, &mut ry, *dir);
    }
    warehouse
}

fn move_robot_wide(warehouse: &mut WideWarehouse, rx: &mut isize, ry: &mut isize, dir: Dir) {
    let (dx, dy) = match dir {
        Dir::Up => (0, -1),
        Dir::Right => (1, 0),
        Dir::Down => (0, 1),
        Dir::Left => (-1, 0),
    };
    let moved_tiles: HashSet<(isize, isize, WideTile)> = if dx != 0 {
        let mut i = 1;
        let mut tile = warehouse.get(*rx + dx * i, *ry);
        while tile != WideTile::Wall && tile != WideTile::Empty {
            i += 1;
            tile = warehouse.get(*rx + dx * i, *ry);
        }
        if tile == WideTile::Wall {
            return;
        }
        (1..i)
            .map(|d| (*rx + dx * d, *ry))
            .map(|(x, y)| (x, y, warehouse.get(x, y)))
            .collect()
    } else {
        let mut seen: HashSet<(isize, isize, WideTile)> = HashSet::with_capacity(128);
        let mut stack: Vec<(isize, isize)> = Vec::with_capacity(128);
        stack.push((*rx, *ry + dy));
        while let Some((x, y)) = stack.pop() {
            let tile = warehouse.get(x, y);
            if seen.contains(&(x, y, tile)) {
                continue;
            }
            match tile {
                WideTile::Wall => {
                    return;
                }
                WideTile::Empty => {
                    continue;
                }
                WideTile::RBox => {
                    seen.insert((x, y, tile));
                    stack.push((x - 1, y));
                    stack.push((x, y + dy));
                }
                WideTile::LBox => {
                    seen.insert((x, y, tile));
                    stack.push((x + 1, y));
                    stack.push((x, y + dy));
                }
            }
        }
        seen
    };
    *rx += dx;
    *ry += dy;
    for (x, y, _) in moved_tiles.iter().copied() {
        warehouse.set(x, y, WideTile::Empty);
    }
    for (x, y, tile) in moved_tiles.iter().copied() {
        warehouse.set(x + dx, y + dy, tile);
    }
}

fn gps_sum_wide(warehouse: &WideWarehouse) -> isize {
    warehouse
        .tiles
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, tile)| *tile == WideTile::LBox)
        .map(|(i, _)| {
            let x = i as isize % warehouse.width;
            let y = i as isize / warehouse.width;
            x + 100 * y
        })
        .sum()
}

fn gps_sum(warehouse: &Warehouse) -> isize {
    warehouse
        .tiles
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, tile)| *tile == Tile::Box)
        .map(|(i, _)| {
            let x = i as isize % warehouse.width;
            let y = i as isize / warehouse.width;
            x + 100 * y
        })
        .sum()
}

fn apply_instructions(
    mut warehouse: Warehouse,
    instructions: &[Dir],
    start_x: isize,
    start_y: isize,
) -> Warehouse {
    let mut rx = start_x;
    let mut ry = start_y;
    for dir in instructions {
        move_robot(&mut warehouse, &mut rx, &mut ry, *dir);
    }
    warehouse
}

fn move_robot(warehouse: &mut Warehouse, rx: &mut isize, ry: &mut isize, dir: Dir) {
    let (dx, dy) = match dir {
        Dir::Up => (0, -1),
        Dir::Right => (1, 0),
        Dir::Down => (0, 1),
        Dir::Left => (-1, 0),
    };
    let mut i = 1;
    let mut tile: Tile = warehouse.get(*rx + dx * i, *ry + dy * i);
    while tile != Tile::Wall && tile != Tile::Empty {
        i += 1;
        tile = warehouse.get(*rx + dx * i, *ry + dy * i);
    }
    if tile == Tile::Wall {
        return;
    }
    warehouse.swap(*rx + dx, *ry + dy, *rx + dx * i, *ry + dy * i);
    *rx += dx;
    *ry += dy;
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum WideTile {
    Empty,
    Wall,
    LBox,
    RBox,
}

#[derive(Clone, Debug)]
struct WideWarehouse {
    width: isize,
    tiles: Box<[WideTile]>,
}

impl WideWarehouse {
    fn get(&self, x: isize, y: isize) -> WideTile {
        self.tiles[x as usize + y as usize * self.width as usize]
    }

    fn set(&mut self, x: isize, y: isize, tile: WideTile) {
        self.tiles[x as usize + y as usize * self.width as usize] = tile;
    }
}

fn widen_warehouse(warehouse: &Warehouse) -> WideWarehouse {
    let tiles: Box<[WideTile]> = warehouse
        .tiles
        .iter()
        .flat_map(|tile| match tile {
            Tile::Empty => [WideTile::Empty, WideTile::Empty],
            Tile::Wall => [WideTile::Wall, WideTile::Wall],
            Tile::Box => [WideTile::LBox, WideTile::RBox],
        })
        .collect();

    WideWarehouse {
        tiles,
        width: warehouse.width * 2,
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Tile {
    Empty,
    Wall,
    Box,
}

impl Warehouse {
    fn get(&self, x: isize, y: isize) -> Tile {
        self.tiles[x as usize + y as usize * self.width as usize]
    }
    fn swap(&mut self, x1: isize, y1: isize, x2: isize, y2: isize) {
        let tile = self.tiles[x1 as usize + y1 as usize * self.width as usize];
        self.tiles[x1 as usize + y1 as usize * self.width as usize] =
            self.tiles[x2 as usize + y2 as usize * self.width as usize];
        self.tiles[x2 as usize + y2 as usize * self.width as usize] = tile;
    }
}

#[derive(Clone, Debug)]
struct Warehouse {
    width: isize,
    tiles: Box<[Tile]>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn parse(input: &str) -> Result<(Warehouse, Box<[Dir]>, isize, isize), String> {
    let (map, inst) = input
        .split_once("\n\n")
        .ok_or_else(|| "unable to split map from instructions".to_owned())?;
    let (warehouse, startx, starty) = parse_warehouse(map)?;
    let inst = parse_instructions(inst)?;

    Ok((warehouse, inst, startx, starty))
}

fn parse_warehouse(map: &str) -> Result<(Warehouse, isize, isize), String> {
    let width = map
        .lines()
        .next()
        .ok_or_else(|| "no lines in warehouse map".to_owned())?
        .len();
    if !map.lines().all(|line| line.len() == width) {
        return Err("not all lines in the warehouse map have the same length".to_owned());
    }
    let height = map.lines().count();
    let tiles: Box<[Tile]> = map
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '.' | '@' => Ok(Tile::Empty),
            '#' => Ok(Tile::Wall),
            'O' => Ok(Tile::Box),
            _ => Err(format!("unexpected tile: '{c}'")),
        })
        .collect::<Result<_, _>>()?;
    let pi = map
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .filter(|(_, c)| *c == '@')
        .map(|(i, _)| i)
        .next()
        .ok_or_else(|| "no robot in input".to_owned())?;
    let px = pi % width;
    let py = pi / width;

    for x in 0..width {
        if tiles[x] != Tile::Wall || tiles[x + (height - 1) * width] != Tile::Wall {
            return Err(format!("missing wall at x={x}"));
        }
    }
    for y in 0..height {
        if tiles[y * width] != Tile::Wall || tiles[width - 1 + y * width] != Tile::Wall {
            return Err(format!("missing wall at y={y}"));
        }
    }

    Ok((
        Warehouse {
            width: width as isize,
            tiles,
        },
        px as isize,
        py as isize,
    ))
}

fn parse_instructions(input: &str) -> Result<Box<[Dir]>, String> {
    input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '^' => Ok(Dir::Up),
            '>' => Ok(Dir::Right),
            'v' => Ok(Dir::Down),
            '<' => Ok(Dir::Left),
            _ => Err(format!("unexpected direction '{c}")),
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL_EXAMPLE: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"#;

    const EXAMPLE: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"#;

    #[test]
    fn apply_instructions_works_for_small_example() {
        // given
        let (warehouse, instructions, start_x, start_y) =
            parse(SMALL_EXAMPLE).expect("expected example input to parse");

        // when
        let warehouse = apply_instructions(warehouse, &instructions, start_x, start_y);
        let result = gps_sum(&warehouse);

        // then
        assert_eq!(result, 2028);
    }

    #[test]
    fn apply_instructions_wide_works_for_example() {
        // given
        let (warehouse, instructions, start_x, start_y) =
            parse(EXAMPLE).expect("expected example input to parse");
        let warehouse = widen_warehouse(&warehouse);

        // when
        let warehouse = apply_instructions_wide(warehouse, &instructions, start_x * 2, start_y);
        let result = gps_sum_wide(&warehouse);

        // then
        assert_eq!(result, 9021);
    }
}
