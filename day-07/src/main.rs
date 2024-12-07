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

    let tcr = concat_calibration_result(&equations);
    println!("the total calibration result (with concat) is {tcr}");

    Ok(())
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Op {
    Add,
    Mul,
    Cat,
}

fn concat_calibration_result(equations: &[Equation]) -> u64 {
    equations
        .iter()
        .filter(|(lhs, rhs)| possibly_valid_concat(*lhs, rhs))
        .map(|(lhs, _)| *lhs)
        .sum()
}

fn possibly_valid_concat(lhs: u64, rhs: &[u64]) -> bool {
    if rhs.is_empty() {
        return false;
    }
    let mut operators: Box<[Op]> = (0..(rhs.len() - 1)).map(|_| Op::Add).collect();
    let mut has_next = true;
    while has_next {
        let mut result = rhs[0];
        for (operand, operator) in rhs[1..].iter().zip(operators.iter()) {
            result = match operator {
                Op::Add => result + operand,
                Op::Mul => result * operand,
                Op::Cat => concat(result, *operand),
            };
        }
        if result == lhs {
            return true;
        }
        has_next = next_op(&mut operators);
    }
    false
}

fn next_op(operators: &mut [Op]) -> bool {
    let mut carry = true;
    let mut i = 0;
    while carry && i < operators.len() {
        (operators[i], carry) = match operators[i] {
            Op::Add => (Op::Mul, false),
            Op::Mul => (Op::Cat, false),
            Op::Cat => (Op::Add, true),
        };
        i += 1;
    }
    !carry
}

fn concat(lhs: u64, rhs: u64) -> u64 {
    lhs * 10u64.pow(rhs.checked_ilog10().unwrap_or(0) + 1) + rhs
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

    #[test]
    fn concat_calibration_result_works_for_example() {
        // given
        let equations = parse(EXAMPLE).expect("expcted exampe input to parse");

        // when
        let tcr = concat_calibration_result(&equations);

        // then
        assert_eq!(tcr, 11387);
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(12, 345), 12345);
        assert_eq!(concat(0, 345), 345);
        assert_eq!(concat(12, 0), 120);
    }

    #[test]
    fn next_op_works_for_round_trip() {
        // given
        let mut ops = [Op::Add, Op::Add];

        // when/then
        let fine = next_op(&mut ops);
        assert!(fine); // This is fine.
        assert_eq!(&ops, &[Op::Mul, Op::Add]);

        let fine = next_op(&mut ops);
        assert!(fine); // This is fine.
        assert_eq!(&ops, &[Op::Cat, Op::Add]);

        let fine = next_op(&mut ops);
        assert!(fine); // This is fine.
        assert_eq!(&ops, &[Op::Add, Op::Mul]);

        let fine = next_op(&mut ops);
        assert!(fine); // This is fine.
        assert_eq!(&ops, &[Op::Mul, Op::Mul]);

        let fine = next_op(&mut ops);
        assert!(fine); // This is fine.
        assert_eq!(&ops, &[Op::Cat, Op::Mul]);

        let fine = next_op(&mut ops);
        assert!(fine); // This is fine.
        assert_eq!(&ops, &[Op::Add, Op::Cat]);

        let fine = next_op(&mut ops);
        assert!(fine); // This is fine.
        assert_eq!(&ops, &[Op::Mul, Op::Cat]);

        let fine = next_op(&mut ops);
        assert!(fine); // This is fine.
        assert_eq!(&ops, &[Op::Cat, Op::Cat]);

        let fine = next_op(&mut ops);
        assert!(!fine);
        assert_eq!(&ops, &[Op::Add, Op::Add]);
    }
}
