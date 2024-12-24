#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let (inputs, gates) = parse(&content)?;

    let output = calculate_output_number(&gates, inputs.clone());
    println!("The number output on the 'z' wires is {output}");

    print_graphviz_graph(&gates);

    let swapped_wires = find_wrong_pairs(&gates, &inputs);
    println!("The names of the swapped wires are {swapped_wires}");

    Ok(())
}

fn find_wrong_pairs(gates: &[Operation], input_values: &HashMap<&str, bool>) -> String {
    // screw this. It's christmas eve and I have no time. I'll just look at the graph and debug the
    // wires manually
    "<not implemented yet>".to_owned()
}

fn print_graphviz_graph(gates: &[Operation]) {
    println!("digraph {{");

    let mut op_counter = 0;
    for (input1, input2, gate, output) in gates {
        let op_name = match gate {
            Gate::And => "AND",
            Gate::Or => "OR",
            Gate::Xor => "XOR",
        };
        let op_node_name = format!("{op_name}_{op_counter}");
        op_counter += 1;
        println!("{op_node_name} [label = \"{op_name}\"];");
        println!("{input1} -> {op_node_name};");
        println!("{input2} -> {op_node_name};");
        println!("{op_node_name} -> {output};");
    }

    println!("}}");
}

fn calculate_output_number(gates: &[Operation], input_values: HashMap<&str, bool>) -> u64 {
    let values = resolve_gates(gates, input_values);
    let mut z_values: Vec<(&str, bool)> = values
        .iter()
        .filter(|(name, _)| name.starts_with('z'))
        .map(|(name, value)| (*name, *value))
        .collect();
    z_values.sort_unstable_by(|(name1, _), (name2, _)| name2.cmp(name1));
    let mut number: u64 = 0;
    for (_, digit) in z_values {
        number <<= 1;
        number |= if digit { 1 } else { 0 };
    }
    number
}

fn resolve_gates<'a>(
    gates: &[Operation<'a>],
    mut values: HashMap<&'a str, bool>,
) -> HashMap<&'a str, bool> {
    // this is a bit inefficient, but should work fine for the input size
    let mut gates: Vec<Operation> = gates.to_vec();
    let mut queue: Vec<Operation> = Vec::with_capacity(gates.len());

    while !gates.is_empty() {
        for operation in &gates {
            let (op1, op2, gate, target) = operation;
            if let (Some(a), Some(b)) = (values.get(op1), values.get(op2)) {
                values.insert(target, gate.apply(*a, *b));
            } else {
                queue.push(*operation);
            }
        }
        std::mem::swap(&mut gates, &mut queue);
        queue.clear();
    }
    values
}

fn parse(input: &str) -> Result<(HashMap<&str, bool>, Box<[Operation]>), String> {
    let (inputs, gates) = input
        .split_once("\n\n")
        .ok_or_else(|| "unable to split inputs from operations".to_owned())?;
    let inputs = parse_inputs(inputs)?;
    let gates = parse_gates(gates)?;
    Ok((inputs, gates))
}

fn parse_inputs(input: &str) -> Result<HashMap<&str, bool>, String> {
    input
        .lines()
        .map(|line| {
            let (name, value) = line
                .split_once(": ")
                .ok_or_else(|| format!("unable to split line '{line}'"))?;
            let value = match value {
                "0" => false,
                "1" => true,
                _ => return Err("invalid value in line '{line}'".to_owned()),
            };
            Ok((name, value))
        })
        .collect()
}

type Operation<'a> = (&'a str, &'a str, Gate, &'a str);

fn parse_gates(input: &str) -> Result<Box<[Operation<'_>]>, String> {
    input
        .lines()
        .map(|line| {
            let (operation, target) = line
                .split_once(" -> ")
                .ok_or_else(|| format!("unable to split operation from target in line '{line}'"))?;
            let (operand_1, rest) = operation
                .split_once(' ')
                .ok_or_else(|| format!("unable to split off first operand in line '{line}'"))?;
            let (gate, operand_2) = rest.split_once(' ').ok_or_else(|| {
                format!("unable to split gate and second operand in line '{line}'")
            })?;
            let gate = match gate {
                "AND" => Gate::And,
                "OR" => Gate::Or,
                "XOR" => Gate::Xor,
                _ => return Err(format!("unknown gate '{gate}' in line '{line}'")),
            };
            Ok((operand_1, operand_2, gate, target))
        })
        .collect()
}

#[derive(Copy, Clone, Debug)]
enum Gate {
    And,
    Or,
    Xor,
}

impl Gate {
    fn apply(self, op1: bool, op2: bool) -> bool {
        match self {
            Gate::And => op1 && op2,
            Gate::Or => op1 || op2,
            Gate::Xor => op1 ^ op2,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"#;

    #[test]
    fn calculate_output_number_works_for_example() {
        // given
        let (inputs, gates) = parse(EXAMPLE).expect("expected example input to parse");

        // when
        let result = calculate_output_number(&gates, inputs);

        // then
        assert_eq!(result, 2024);
    }
}
