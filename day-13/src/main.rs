#![forbid(unsafe_code)]

use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let machines = parse(&content)?;

    let price = min_tokens_for_all_prizes(&machines);
    println!("The fewest tokens I would have to spend to win all possible prizes is {price}");

    let fixed_machines = fix_machines(&machines);
    let price = min_tokens_for_all_prizes(&fixed_machines);
    println!(
        "The fewest tokens I would have to spend to win all possible prizes for the fixed machines is {price}"
    );

    Ok(())
}

fn fix_machines(machines: &[Machine]) -> Box<[Machine]> {
    machines
        .iter()
        .map(|broken| Machine {
            prize_x: broken.prize_x + 10_000_000_000_000,
            prize_y: broken.prize_y + 10_000_000_000_000,
            ..*broken
        })
        .collect()
}

fn min_tokens_for_all_prizes(machines: &[Machine]) -> i64 {
    machines.iter().filter_map(find_cheapest_solution).sum()
}

fn find_cheapest_solution(machine: &Machine) -> Option<i64> {
    // Today is math day. I used Pen & Paper to get these equations
    let num = machine.prize_y * machine.a_x - machine.a_y * machine.prize_x;
    let denom = machine.b_y * machine.a_x - machine.a_y * machine.b_x;
    if num % denom != 0 {
        return None;
    }
    let b = num / denom;
    if b < 0 {
        return None;
    }
    let a = find_a_presses(machine, b)?;

    Some(3 * a + b)
}

fn find_a_presses(machine: &Machine, b: i64) -> Option<i64> {
    let x = machine.b_x * b;
    if x > machine.prize_x {
        return None;
    }
    let dx = machine.prize_x - x;
    let a = dx / machine.a_x;
    if x + a * machine.a_x == machine.prize_x
        && machine.a_y * a + machine.b_y * b == machine.prize_y
    {
        Some(a)
    } else {
        None
    }
}

#[derive(Copy, Clone, Debug)]
struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

fn parse(input: &str) -> Result<Box<[Machine]>, String> {
    input.split("\n\n").map(parse_machine).collect()
}

fn parse_machine(block: &str) -> Result<Machine, String> {
    let mut lines = block.lines();
    let line_a = lines
        .next()
        .ok_or_else(|| format!("expected three lines in block '{block}'"))?;
    let line_b = lines
        .next()
        .ok_or_else(|| format!("expected three lines in block '{block}'"))?;
    let line_prize = lines
        .next()
        .ok_or_else(|| format!("expected three lines in block '{block}'"))?;

    let (a_x, a_y) = line_a
        .strip_prefix("Button A: ")
        .and_then(|l| l.split_once(", "))
        .ok_or_else(|| format!("invalid format for button A: '{line_a}'"))?;
    let a_x: i64 = a_x
        .strip_prefix("X+")
        .ok_or_else(|| format!("invalid format for X in line '{line_a}'"))?
        .parse::<i64>()
        .map_err(|e| format!("unable to parse value X in line '{line_a}': {e}"))?;
    let a_y: i64 = a_y
        .strip_prefix("Y+")
        .ok_or_else(|| format!("invalid format for Y in line '{line_a}'"))?
        .parse::<i64>()
        .map_err(|e| format!("unable to parse value Y in line '{line_a}': {e}"))?;

    let (b_x, b_y) = line_b
        .strip_prefix("Button B: ")
        .and_then(|l| l.split_once(", "))
        .ok_or_else(|| format!("invalid format for button B: '{line_b}'"))?;
    let b_x: i64 = b_x
        .strip_prefix("X+")
        .ok_or_else(|| format!("invalid format for X in line '{line_b}'"))?
        .parse::<i64>()
        .map_err(|e| format!("unable to parse value X in line '{line_b}': {e}"))?;
    let b_y: i64 = b_y
        .strip_prefix("Y+")
        .ok_or_else(|| format!("invalid format for Y in line '{line_b}'"))?
        .parse::<i64>()
        .map_err(|e| format!("unable to parse value Y in line '{line_b}': {e}"))?;

    let (prize_x, prize_y) = line_prize
        .strip_prefix("Prize: ")
        .and_then(|l| l.split_once(", "))
        .ok_or_else(|| format!("invalid format for prize: '{line_prize}'"))?;
    let prize_x: i64 = prize_x
        .strip_prefix("X=")
        .ok_or_else(|| format!("invalid format for X in line '{line_prize}'"))?
        .parse::<i64>()
        .map_err(|e| format!("unable to parse X value for prize in line '{line_prize}': {e}"))?;
    let prize_y: i64 = prize_y
        .strip_prefix("Y=")
        .ok_or_else(|| format!("invalid format for Y in line '{line_prize}'"))?
        .parse::<i64>()
        .map_err(|e| format!("unable to parse Y value for prize in line '{line_prize}': {e}"))?;

    Ok(Machine {
        a_x,
        a_y,
        b_x,
        b_y,
        prize_x,
        prize_y,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#;

    #[test]
    fn min_token_for_all_prizes_works_for_example() {
        // given
        let machines = parse(EXAMPLE).expect("expected example input to parse");

        // when
        let price = min_tokens_for_all_prizes(&machines);

        // then
        assert_eq!(price, 480);
    }
}
