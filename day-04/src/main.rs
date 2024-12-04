use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let (width, height, grid) = make_grid(&content)?;

    let xmas_count = count_non_palindrome(&grid, width, height, b"XMAS");
    println!("{xmas_count} occurences of 'XMAS'");

    Ok(())
}

fn make_grid(input: &str) -> Result<(usize, usize, Box<[u8]>), String> {
    // the grid indices we use later require uniform char length. We assume ascii input here to
    // make it easier
    if !input.is_ascii() {
        return Err("input is not all ASCII".to_string());
    }
    let height = input.lines().count();
    let mut lines = input.lines();
    let width = lines.next().unwrap_or("").len();
    if lines.any(|line| line.len() != width) {
        return Err("not all input lines are equally long!".to_string());
    }
    let grid = input
        .as_bytes()
        .iter()
        .filter(|c| **c != b'\n')
        .copied()
        .collect();
    Ok((width, height, grid))
}

fn count_non_palindrome(input: &[u8], width: usize, height: usize, word: &[u8]) -> u32 {
    if word.is_empty() {
        return 0;
    }
    input
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == word[0])
        .map(|(i, _)| {
            let x = i % width;
            let y = i / width;

            let mut count: u32 = 0;

            // backwards horizontal
            if x + 1 >= word.len()
                && word
                    .iter()
                    .copied()
                    .enumerate()
                    .all(|(dx, c)| c == input[y * width + x - dx])
            {
                count += 1;
            }
            // forwards horizontal
            if width >= word.len() + x
                && word
                    .iter()
                    .copied()
                    .enumerate()
                    .all(|(dx, c)| c == input[y * width + x + dx])
            {
                count += 1;
            }
            // backwards vertical
            if y + 1 >= word.len()
                && word
                    .iter()
                    .copied()
                    .enumerate()
                    .all(|(dy, c)| c == input[(y - dy) * width + x])
            {
                count += 1;
            }
            // forwards vertical
            if height >= word.len() + y
                && word
                    .iter()
                    .copied()
                    .enumerate()
                    .all(|(dy, c)| c == input[(y + dy) * width + x])
            {
                count += 1;
            }
            // diagonal up left
            if x + 1 >= word.len()
                && y + 1 >= word.len()
                && word
                    .iter()
                    .copied()
                    .enumerate()
                    .all(|(d, c)| c == input[(y - d) * width + x - d])
            {
                count += 1;
            }
            // diagonal up right
            if width >= word.len() + x
                && y + 1 >= word.len()
                && word
                    .iter()
                    .copied()
                    .enumerate()
                    .all(|(d, c)| c == input[(y - d) * width + x + d])
            {
                count += 1;
            }
            // diagonal down right
            if width >= word.len() + x
                && height >= word.len() + y
                && word
                    .iter()
                    .copied()
                    .enumerate()
                    .all(|(d, c)| c == input[(y + d) * width + x + d])
            {
                count += 1;
            }
            // diagonal down left
            if x + 1 >= word.len()
                && height >= word.len() + y
                && word
                    .iter()
                    .copied()
                    .enumerate()
                    .all(|(d, c)| c == input[(y + d) * width + x - d])
            {
                count += 1;
            }

            count
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

    #[test]
    fn count_non_palindrome_works_for_example() {
        // given
        let (width, height, grid) = make_grid(INPUT).expect("expected well-formed input");
        let search_word = b"XMAS";

        // when
        let count = count_non_palindrome(&grid, width, height, search_word);

        // then
        assert_eq!(count, 18);
    }
}
