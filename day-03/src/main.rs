use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;

    let mul_instructions = parse_ignore_corrupted(&content);
    let sum = mul_sum(&mul_instructions);
    println!("The sum of the non-corrupted mul instructions is {sum}");

    let do_mul_instructions = parse_handle_do(&content);
    let sum = mul_sum(&do_mul_instructions);
    println!("The sum when you also handle the do/don't operations is {sum}");

    Ok(())
}

fn parse_ignore_corrupted(memory: &str) -> Box<[(i64, i64)]> {
    memory
        .split("mul(")
        .filter_map(parse_mul_instruction)
        .collect()
}

fn parse_handle_do(memory: &str) -> Vec<(i64, i64)> {
    let mut on = true;
    let mut instructions: Vec<(i64, i64)> = Vec::with_capacity(128);
    for part in memory.split("mul(") {
        if on {
            if let Some(instruction) = parse_mul_instruction(part) {
                instructions.push(instruction);
            }
        }
        let do_pos = part.rfind("do()");
        let dont_pos = part.rfind("don't()");
        on = match (do_pos, dont_pos) {
            (None, None) => on,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (Some(dop), Some(dontp)) => dop > dontp,
        };
    }
    instructions
}

fn parse_mul_instruction(s: &str) -> Option<(i64, i64)> {
    let (params, _) = s.split_once(')')?;
    let (l, r) = params.split_once(',')?;
    Some((l.parse::<i64>().ok()?, r.parse::<i64>().ok()?))
}

fn mul_sum(instructions: &[(i64, i64)]) -> i64 {
    instructions.iter().map(|(l, r)| l * r).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_ignore_corrupted_works_for_example() {
        // when
        let instructions = parse_ignore_corrupted(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );

        // then
        let expected: &[(i64, i64)] = &[(2, 4), (5, 5), (11, 8), (8, 5)];
        let actual: &[(i64, i64)] = &instructions;
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_handle_do_works_for_example() {
        // when
        let instructions = parse_handle_do(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );

        // then
        let expected: &[(i64, i64)] = &[(2, 4), (8, 5)];
        let actual: &[(i64, i64)] = &instructions;
        assert_eq!(actual, expected);
    }
}
