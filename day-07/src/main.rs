use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let equations = parse(&content)?;

    let tcr = total_calibration_result(&equations);
    println!("the total calibration result is {tcr}");

    Ok(())
}

fn total_calibration_result(equations: &[Equation]) -> u64 {
    equations
        .iter()
        .filter(|(lhs, rhs)| possibly_valid(*lhs, rhs))
        .map(|(lhs, _)| *lhs)
        .sum()
}

fn possibly_valid(lhs: u64, rhs: &[u64]) -> bool {
    if rhs.is_empty() {
        return false;
    }
    (0u64..1 << (rhs.len() - 1)).any(|operators| calculate(rhs, operators) == lhs)
}

fn calculate(numbers: &[u64], operators: u64) -> u64 {
    if numbers.is_empty() {
        return 0;
    }
    let mut result = numbers[0];
    for (i, number) in numbers[1..].iter().enumerate() {
        if (operators >> i) & 1 == 0 {
            result += number;
        } else {
            result *= number;
        }
    }
    result
}

fn parse(input: &str) -> Result<Box<[Equation]>, String> {
    input.lines().map(parse_equation).collect()
}

type Equation = (u64, Box<[u64]>);
fn parse_equation(line: &str) -> Result<Equation, String> {
    let (left, right) = line
        .split_once(": ")
        .ok_or_else(|| format!("unable to split equation in line '{line}'"))?;
    let left: u64 = left
        .parse()
        .map_err(|e| format!("unable to parse left side '{left}': {e}"))?;
    let right: Box<[u64]> = right
        .split(' ')
        .map(|n| {
            n.parse::<u64>()
                .map_err(|e| format!("unable to parse right side '{n}': {e}"))
        })
        .collect::<Result<_, _>>()?;

    Ok((left, right))
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

    #[test]
    fn total_calibration_result_works_for_example() {
        // given
        let equations = parse(EXAMPLE).expect("expected example input to parse");

        // when
        let tcr = total_calibration_result(&equations);

        // then
        assert_eq!(tcr, 3749);
    }
}
