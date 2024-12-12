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
    let garden = parse(&content)?;

    let price = fence_price(&garden);
    println!("The total fence price of all regions is {price}");

    Ok(())
}

fn fence_price(garden: &Garden) -> u32 {
    let mut visited: HashSet<(usize, usize)> = HashSet::with_capacity(garden.plots.len());

    let mut price = 0;
    for i in 0..garden.plots.len() {
        let x = i % garden.width;
        let y = i / garden.width;
        if !visited.contains(&(x, y)) {
            price += area_price(garden, &mut visited, x, y);
        }
    }
    price
}

fn area_price(garden: &Garden, visited: &mut HashSet<(usize, usize)>, x: usize, y: usize) -> u32 {
    let mut stack: Vec<(usize, usize)> = Vec::with_capacity(garden.plots.len());
    let mut area: u32 = 0;
    let mut perimeter: u32 = 0;
    stack.push((x, y));
    visited.insert((x, y));

    let crop = garden.get(x, y);

    while let Some((x, y)) = stack.pop() {
        area += 1;
        if x > 0 && garden.get(x - 1, y) == crop {
            if !visited.contains(&(x - 1, y)) {
                stack.push((x - 1, y));
                visited.insert((x - 1, y));
            }
        } else {
            perimeter += 1;
        }
        if y > 0 && garden.get(x, y - 1) == crop {
            if !visited.contains(&(x, y - 1)) {
                stack.push((x, y - 1));
                visited.insert((x, y - 1));
            }
        } else {
            perimeter += 1;
        }
        if garden.get(x + 1, y) == crop {
            if !visited.contains(&(x + 1, y)) {
                stack.push((x + 1, y));
                visited.insert((x + 1, y));
            }
        } else {
            perimeter += 1;
        }
        if garden.get(x, y + 1) == crop {
            if !visited.contains(&(x, y + 1)) {
                stack.push((x, y + 1));
                visited.insert((x, y + 1));
            }
        } else {
            perimeter += 1;
        }
    }

    area * perimeter
}

#[derive(Clone, Debug)]
struct Garden {
    width: usize,
    height: usize,
    plots: Box<[char]>,
}

impl Garden {
    fn get(&self, x: usize, y: usize) -> Option<char> {
        if x < self.width && y < self.height {
            self.plots.get(x + y * self.width).copied()
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Result<Garden, String> {
    let width = input
        .lines()
        .next()
        .ok_or_else(|| "expected at least one line".to_string())?
        .len();
    if !input.lines().all(|line| line.len() == width) {
        return Err("non-uniform line length".to_owned());
    }
    let height = input.lines().count();
    let plots = input.chars().filter(|c| *c != '\n').collect();
    Ok(Garden {
        width,
        height,
        plots,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL_EXAMPLE: &str = r#"AAAA
BBCD
BBCC
EEEC
"#;

    const ENCLAVE_EXAMPLE: &str = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"#;

    const EXAMPLE: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"#;

    #[test]
    fn fence_price_works_for_small_example() {
        // given
        let garden = parse(SMALL_EXAMPLE).expect("expected example input to parse");

        // when
        let price = fence_price(&garden);

        // then
        assert_eq!(price, 140);
    }

    #[test]
    fn fence_price_works_for_enclave_example() {
        // given
        let garden = parse(ENCLAVE_EXAMPLE).expect("expected example input to parse");

        // when
        let price = fence_price(&garden);

        // then
        assert_eq!(price, 772);
    }

    #[test]
    fn fence_price_works_for_example() {
        // given
        let garden = parse(EXAMPLE).expect("expected example input to parse");

        // when
        let price = fence_price(&garden);

        // then
        assert_eq!(price, 1930);
    }
}
