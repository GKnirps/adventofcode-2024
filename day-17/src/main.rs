#![forbid(unsafe_code)]

use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let (registers, program) = parse(&content)?;

    let output = run_program(&program, registers)?;
    let output = output
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(",");
    println!("The output is {output}");

    println!("input program:");
    print_mnemonics(&program)?;
    println!();

    let quine_a = find_quine_input(&program, registers)?;
    println!("The lowest positive initial value for register A that causes the program to output a copy of itself is {quine_a}");

    Ok(())
}

// This solution is specific to structures (division by eight) in my input and the example input
// it may not work on all AoC inputs and will certainly not work on all valid programs
fn find_quine_input(program: &[u64], registers: Registers) -> Result<u64, String> {
    let mut a: u64 = 0;
    for (ip, des_output) in program.iter().enumerate().rev() {
        a <<= 3;
        for i in 0..256 {
            let output = run_program(
                program,
                Registers {
                    a: a + i,
                    ..registers
                },
            )?;
            if output[0] == *des_output && (output.len() < 2 || output[1] == program[ip + 1]) {
                a |= i;
                break;
            }
        }
    }

    let output = run_program(program, Registers { a, ..registers })?;
    if output == program {
        Ok(a)
    } else {
        Err(format!(
            "output does not equal program, output is {output:?}"
        ))
    }
}

fn print_mnemonics(program: &[u64]) -> Result<(), String> {
    for (i, op) in program.chunks_exact(2).enumerate() {
        let operator = op[0];
        let operand = op[1];
        print!("{:02}\t", i * 2);
        match operator {
            0 => {
                println!("adv {}", combo_operand_mnemonic(operand)?);
            }
            1 => {
                println!("bxl {operand}");
            }
            2 => {
                println!("bst {}", combo_operand_mnemonic(operand)?);
            }
            3 => {
                println!("jnz {operand}");
            }
            4 => {
                println!("bxc");
            }
            5 => {
                println!("out {}", combo_operand_mnemonic(operand)?);
            }
            6 => {
                println!("bdv {}", combo_operand_mnemonic(operand)?);
            }
            7 => {
                println!("cdv {}", combo_operand_mnemonic(operand)?);
            }
            _ => return Err(format!("unknown operator '{operator}'")),
        }
    }
    Ok(())
}

fn combo_operand_mnemonic(op: u64) -> Result<String, String> {
    match op {
        0..=3 => Ok(op.to_string()),
        4 => Ok("A".to_string()),
        5 => Ok("B".to_string()),
        6 => Ok("C".to_string()),
        _ => Err(format!("unknown combo operand: '{op}'")),
    }
}

fn run_program(program: &[u64], mut registers: Registers) -> Result<Vec<u64>, String> {
    let mut output: Vec<u64> = Vec::with_capacity(16);
    let mut ip: u64 = 0;
    while handle_instruction(program, &mut ip, &mut registers, &mut output)? {}

    Ok(output)
}

fn handle_instruction(
    program: &[u64],
    ip: &mut u64,
    registers: &mut Registers,
    output: &mut Vec<u64>,
) -> Result<bool, String> {
    if *ip as usize + 1 >= program.len() {
        return Ok(false);
    }
    let operator = program[*ip as usize];
    let operand = program[*ip as usize + 1];

    match operator {
        // adv
        0 => {
            registers.a /= 1 << value_combo_operand(operand, registers)?;
            *ip += 2;
        }
        // bxl
        1 => {
            registers.b ^= operand;
            *ip += 2;
        }
        // bst
        2 => {
            registers.b = value_combo_operand(operand, registers)? % 8;
            *ip += 2;
        }
        // jnz
        3 => {
            if registers.a != 0 {
                *ip = operand;
            } else {
                *ip += 2;
            }
        }
        // bxc
        4 => {
            registers.b ^= registers.c;
            *ip += 2;
        }
        // out
        5 => {
            output.push(value_combo_operand(operand, registers)? % 8);
            *ip += 2;
        }
        // bdv
        6 => {
            registers.b = registers.a / (1 << value_combo_operand(operand, registers)?);
            *ip += 2;
        }
        // cdv
        7 => {
            registers.c = registers.a / (1 << value_combo_operand(operand, registers)?);
            *ip += 2;
        }
        _ => {
            return Err(format!("invalid opcode: {operator}"));
        }
    }

    Ok(true)
}

fn value_combo_operand(operand: u64, registers: &Registers) -> Result<u64, String> {
    match operand {
        0..=3 => Ok(operand),
        4 => Ok(registers.a),
        5 => Ok(registers.b),
        6 => Ok(registers.c),
        _ => Err(format!("invalid operand: {operand}")),
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

fn parse(input: &str) -> Result<(Registers, Box<[u64]>), String> {
    let mut lines = input.lines();
    let a: u64 = lines
        .next()
        .and_then(|line| line.strip_prefix("Register A: "))
        .ok_or_else(|| "invalid format for register A".to_owned())?
        .parse()
        .map_err(|e| format!("unable to parse value for register A: {e}"))?;
    let b: u64 = lines
        .next()
        .and_then(|line| line.strip_prefix("Register B: "))
        .ok_or_else(|| "invalid format for register B".to_owned())?
        .parse()
        .map_err(|e| format!("unable to parse value for register B: {e}"))?;
    let c: u64 = lines
        .next()
        .and_then(|line| line.strip_prefix("Register C: "))
        .ok_or_else(|| "invalid format for register C".to_owned())?
        .parse()
        .map_err(|e| format!("unable to parse value for register C: {e}"))?;

    if lines.next() != Some("") {
        return Err("no empty line between registers and program".to_owned());
    }

    let program: Box<[u64]> = lines
        .next()
        .and_then(|line| line.strip_prefix("Program: "))
        .ok_or_else(|| "invalid format for program".to_owned())?
        .split(',')
        .map(|n| {
            n.parse::<u64>()
                .map_err(|e| format!("unable to parse program value '{n}': {e}"))
        })
        .collect::<Result<_, _>>()?;

    Ok((Registers { a, b, c }, program))
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"#;

    static QUINE_EXAMPLE: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"#;

    #[test]
    fn run_program_works_for_example() {
        // given
        let (registers, program) = parse(EXAMPLE).expect("expected example program to parse");

        // when
        let output = run_program(&program, registers);

        // then
        assert_eq!(output, Ok(vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]))
    }

    #[test]
    fn run_quine_input_works_for_example() {
        // given
        let (registers, program) = parse(QUINE_EXAMPLE).expect("expected example program to parse");

        // when
        let a = find_quine_input(&program, registers);

        // then
        assert_eq!(a, Ok(117440));
    }
}
