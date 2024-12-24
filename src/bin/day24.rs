use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    advent_of_code_24::solve("day24", parse, part1, part2);
}

type Sig = bool;
type S = (Vec<(String, Sig)>, Vec<(String, String, Op, String)>);

fn parse(s: &str) -> S {
    let (register_str, gate_str) = s.split_once("\n\n").unwrap();
    let registers = register_str
        .lines()
        .map(|line| {
            let (reg, val_str) = line.split_once(": ").unwrap();
            (reg.to_owned(), val_str == "1")
        })
        .collect_vec();
    let gates = gate_str
        .lines()
        .map(|line| {
            let (rest, out) = line.split_once(" -> ").unwrap();
            let (left, op_str, right) = rest.split_whitespace().next_tuple().unwrap();
            (
                left.to_owned(),
                right.to_owned(),
                Op::new(op_str),
                out.to_owned(),
            )
        })
        .collect_vec();
    (registers, gates)
}

fn part1((start, gate_instr): &S) -> i64 {
    let (mut gates, gate_lookup) = make_gates(gate_instr);
    let mut queue = start.clone();
    let mut outputs = Vec::new();
    while let Some((in_wire, in_val)) = queue.pop() {
        let input_for = gate_lookup.get(&in_wire);
        if input_for.is_none() {
            outputs.push((in_wire, in_val));
            continue;
        }
        for gate_idx in input_for.unwrap() {
            if let Some(out) = gates[*gate_idx].load(in_val) {
                queue.push(out);
            }
        }
    }

    outputs.sort();
    outputs.reverse();
    outputs
        .into_iter()
        .fold(0, |acc, (_, val)| (acc << 1) | if val { 1 } else { 0 })
}

fn make_gates(
    gate_instr: &Vec<(String, String, Op, String)>,
) -> (Vec<Gate>, HashMap<String, Vec<usize>>) {
    let mut gate_lookup = HashMap::<String, Vec<usize>>::new();
    let mut gates = Vec::new();
    for (left, right, op, out) in gate_instr {
        let gate = Gate::new(op.clone(), out.clone());
        gates.push(gate);
        gate_lookup
            .entry(left.to_owned())
            .or_default()
            .push(gates.len() - 1);
        gate_lookup
            .entry(right.to_owned())
            .or_default()
            .push(gates.len() - 1);
    }
    (gates, gate_lookup)
}

fn part2((start, gate_instr): &S) -> i64 {
    let (init_gates, gate_lookup) = make_gates(gate_instr);

    let (size, _) = start
        .iter()
        .enumerate()
        .find(|(_, (wire, _))| wire == "y00")
        .unwrap();
    println!("Register size: {}", size);

    for (x, y) in [(false, false), (true, false), (false, true), (true, true)] {
        for i in 0..size {
            let mut gates = init_gates.clone();
            let mut queue = vec![(format!("x{:02}", i), x), (format!("y{:02}", i), y)];
            let mut outputs = Vec::new();
            while let Some((in_wire, in_val)) = queue.pop() {
                let input_for = gate_lookup.get(&in_wire);
                if input_for.is_none() {
                    outputs.push((in_wire, in_val));
                    continue;
                }
                for gate_idx in input_for.unwrap() {
                    if let Some(out) = gates[*gate_idx].load(in_val) {
                        queue.push(out);
                    }
                }
            }
            println!("Output bit {} {} {}: {:?}", i, x, y, outputs);
        }
    }
    1
}

#[derive(Clone, Debug)]
enum Op {
    AND,
    OR,
    XOR,
}

impl Op {
    fn new(s: &str) -> Self {
        match s {
            "AND" => Self::AND,
            "OR" => Self::OR,
            "XOR" => Self::XOR,
            _ => panic!("no gate"),
        }
    }
}

#[derive(Clone, Debug)]
struct Gate {
    op: Op,
    other: Option<Sig>,
    out: String,
}

impl Gate {
    fn new(op: Op, out: String) -> Self {
        Gate {
            op,
            other: None,
            out,
        }
    }

    // Load value if gate is empty, return result
    // on second load.
    fn load(&mut self, val: Sig) -> Option<(String, Sig)> {
        match self.other {
            None => {
                self.other = Some(val);
                None
            }
            Some(other) => Some((
                self.out.to_owned(),
                match self.op {
                    Op::AND => val & other,
                    Op::OR => val | other,
                    Op::XOR => val ^ other,
                },
            )),
        }
    }
}

#[allow(dead_code)]
const TEST_INPUT: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

#[test]
fn test_part1() {
    advent_of_code_24::test1(TEST_INPUT, 4, parse, part1);
}

#[allow(dead_code)]
const TEST_INPUT2: &str = "x00: 1
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
tnw OR pbm -> gnj";

#[test]
fn test_part1_large() {
    advent_of_code_24::test1(TEST_INPUT2, 2024, parse, part1);
}

#[allow(dead_code)]
const TEST_INPUT3: &str = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";

#[test]
fn test_part2() {
    advent_of_code_24::test1(TEST_INPUT3, 0, parse, part2);
}
