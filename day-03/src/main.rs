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

    Ok(())
}

fn parse_ignore_corrupted(memory: &str) -> Box<[(i64, i64)]> {
    memory
        .split("mul(")
        .filter_map(|s| {
            let (params, _) = s.split_once(')')?;
            let (l, r) = params.split_once(',')?;
            Some((l.parse::<i64>().ok()?, r.parse::<i64>().ok()?))
        })
        .collect()
}

fn mul_sum(instructions: &[(i64, i64)]) -> i64 {
    instructions.iter().map(|(l, r)| l * r).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn parse_ignore_corrupted_works_for_example() {
        // when
        let instructions = parse_ignore_corrupted(INPUT);

        // then
        let expected: &[(i64, i64)] = &[(2, 4), (5, 5), (11, 8), (8, 5)];
        let actual: &[(i64, i64)] = &instructions;
        assert_eq!(actual, expected);
    }
}
