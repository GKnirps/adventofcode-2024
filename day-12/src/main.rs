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

    let price = fence_discount_price(&garden);
    println!("The total discount fence price of all regions is {price}");

    Ok(())
}

fn fence_discount_price(garden: &Garden) -> u32 {
    let mut visited: HashSet<(usize, usize)> = HashSet::with_capacity(garden.plots.len());

    let mut price = 0;
    for i in 0..garden.plots.len() {
        let x = i % garden.width;
        let y = i / garden.width;
        if !visited.contains(&(x, y)) {
            price += area_discount_price(garden, &mut visited, x, y);
        }
    }
    price
}

fn area_discount_price(
    garden: &Garden,
    visited: &mut HashSet<(usize, usize)>,
    x: usize,
    y: usize,
) -> u32 {
    let mut stack: Vec<(usize, usize)> = Vec::with_capacity(garden.plots.len());
    let mut area_tiles: HashSet<(usize, usize)> = HashSet::with_capacity(garden.plots.len());
    let mut area: u32 = 0;
    stack.push((x, y));
    visited.insert((x, y));
    area_tiles.insert((x, y));

    let crop = garden.get(x, y);

    while let Some((x, y)) = stack.pop() {
        area += 1;
        if x > 0 && garden.get(x - 1, y) == crop && !visited.contains(&(x - 1, y)) {
            stack.push((x - 1, y));
            visited.insert((x - 1, y));
            area_tiles.insert((x - 1, y));
        }
        if y > 0 && garden.get(x, y - 1) == crop && !visited.contains(&(x, y - 1)) {
            stack.push((x, y - 1));
            visited.insert((x, y - 1));
            area_tiles.insert((x, y - 1));
        }
        if garden.get(x + 1, y) == crop && !visited.contains(&(x + 1, y)) {
            stack.push((x + 1, y));
            visited.insert((x + 1, y));
            area_tiles.insert((x + 1, y));
        }
        if garden.get(x, y + 1) == crop && !visited.contains(&(x, y + 1)) {
            stack.push((x, y + 1));
            visited.insert((x, y + 1));
            area_tiles.insert((x, y + 1));
        }
    }

    let mut perimeter = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::with_capacity(area_tiles.len());
    for (x, y) in area_tiles.iter().copied() {
        if visited.contains(&(x, y)) || y > 0 && area_tiles.contains(&(x, y - 1)) {
            continue;
        }
        perimeter += 1;
        let mut x2 = x;
        while area_tiles.contains(&(x2, y))
            && !(y > 0 && area_tiles.contains(&(x2, y - 1)))
            && !visited.contains(&(x2, y))
        {
            visited.insert((x2, y));
            if x2 == 0 {
                break;
            }
            x2 -= 1;
        }
        x2 = x + 1;
        while area_tiles.contains(&(x2, y))
            && !(y > 0 && area_tiles.contains(&(x2, y - 1)))
            && !visited.contains(&(x2, y))
        {
            visited.insert((x2, y));
            x2 += 1;
        }
    }
    visited.clear();
    for (x, y) in area_tiles.iter().copied() {
        if visited.contains(&(x, y)) || area_tiles.contains(&(x, y + 1)) {
            continue;
        }
        perimeter += 1;
        let mut x2 = x;
        while area_tiles.contains(&(x2, y))
            && !area_tiles.contains(&(x2, y + 1))
            && !visited.contains(&(x2, y))
        {
            visited.insert((x2, y));
            if x2 == 0 {
                break;
            }
            x2 -= 1;
        }
        x2 = x + 1;
        while area_tiles.contains(&(x2, y))
            && !area_tiles.contains(&(x2, y + 1))
            && !visited.contains(&(x2, y))
        {
            visited.insert((x2, y));
            x2 += 1;
        }
    }
    visited.clear();
    for (x, y) in area_tiles.iter().copied() {
        if visited.contains(&(x, y)) || x > 0 && area_tiles.contains(&(x - 1, y)) {
            continue;
        }
        perimeter += 1;
        let mut y2 = y;
        while area_tiles.contains(&(x, y2))
            && !(x > 0 && area_tiles.contains(&(x - 1, y2)))
            && !visited.contains(&(x, y2))
        {
            visited.insert((x, y2));
            if y2 == 0 {
                break;
            }
            y2 -= 1;
        }
        y2 = y + 1;
        while area_tiles.contains(&(x, y2))
            && !(x > 0 && area_tiles.contains(&(x - 1, y2)))
            && !visited.contains(&(x, y2))
        {
            visited.insert((x, y2));
            y2 += 1;
        }
    }
    visited.clear();
    for (x, y) in area_tiles.iter().copied() {
        if visited.contains(&(x, y)) || area_tiles.contains(&(x + 1, y)) {
            continue;
        }
        perimeter += 1;
        let mut y2 = y;
        while area_tiles.contains(&(x, y2))
            && !area_tiles.contains(&(x + 1, y2))
            && !visited.contains(&(x, y2))
        {
            visited.insert((x, y2));
            if y2 == 0 {
                break;
            }
            y2 -= 1;
        }
        y2 = y + 1;
        while area_tiles.contains(&(x, y2))
            && !area_tiles.contains(&(x + 1, y2))
            && !visited.contains(&(x, y2))
        {
            visited.insert((x, y2));
            y2 += 1;
        }
    }

    area * perimeter
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

    const E_EXAMPLE: &str = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
"#;

    const AB_EXAMPLE: &str = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
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

    #[test]
    fn fence_discount_price_works_for_small_example() {
        // given
        let garden = parse(SMALL_EXAMPLE).expect("expected example input to parse");

        // when
        let price = fence_discount_price(&garden);

        // then
        assert_eq!(price, 80);
    }

    #[test]
    fn fence_discount_price_works_for_enclave_example() {
        // given
        let garden = parse(ENCLAVE_EXAMPLE).expect("expected example input to parse");

        // when
        let price = fence_discount_price(&garden);

        // then
        assert_eq!(price, 436);
    }

    #[test]
    fn fence_discount_price_works_for_e_example() {
        // given
        let garden = parse(E_EXAMPLE).expect("expected example input to parse");

        // when
        let price = fence_discount_price(&garden);

        // then
        assert_eq!(price, 236);
    }

    #[test]
    fn fence_discount_price_works_for_ab_example() {
        // given
        let garden = parse(AB_EXAMPLE).expect("expected example input to parse");

        // when
        let price = fence_discount_price(&garden);

        // then
        assert_eq!(price, 368);
    }

    #[test]
    fn fence_discount_price_works_for_example() {
        // given
        let garden = parse(EXAMPLE).expect("expected example input to parse");

        // when
        let price = fence_discount_price(&garden);

        // then
        assert_eq!(price, 1206);
    }
}
