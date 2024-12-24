use std::collections::{HashMap, HashSet, VecDeque};

use itertools::{Itertools, TupleCombinations};

fn main() {
    advent_of_code_24::solve("day24", parse, part1, part2);
}

type Sig = bool;
type S = (Vec<(String, Sig)>, Vec<(String, String, Op, String)>);

/*
y00 AND x00 -> tss  (lower carry)

; Every bit has 5 gates, 2 XOR, 2 AND, 1 OR
x01 XOR y01 -> rvp  (bit-add)
x01 AND y01 -> jcr  (carry)
rvp XOR tss -> z01  (merge with previous carry gives out)
rvp AND tss -> bcr  (carry for previous operation)
bcr OR jcr -> tdp   (carry for next bit)

ccn AND tdp -> tkr
tdp XOR ccn -> z02
 */

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

fn part1((start, gate_instr): &S) -> String {
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
        .to_string()
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

type Gates = Vec<Gate>;
type GateLookup = HashMap<String, Vec<usize>>;

fn part2((start, gates): &S) -> String {
    let (init_gates, gate_lookup) = make_gates(gates);

    let (size, _) = start
        .iter()
        .enumerate()
        .find(|(_, (wire, _))| wire == "y00")
        .unwrap();

    // Find swap candidates
    let mut swap_candidates = Vec::new();
    for i in 0..size {
        let test_res = test_bit(&init_gates, &gate_lookup, i);

        if let Some(cand) = test_res {
            let mut bit_swap_candidates = Vec::new();
            for (c1, c2) in cand.iter().tuple_combinations() {
                if test_bit_with_swaps(&init_gates, &gate_lookup, i, vec![(*c1, *c2)]) {
                    bit_swap_candidates.push((*c1, *c2));
                }
            }
            if !bit_swap_candidates.is_empty() {
                swap_candidates.push(bit_swap_candidates);
            }
        }
    }

    let mut solution = Vec::new();
    for swaps in swap_candidates
        .into_iter()
        .map(|v| v.into_iter())
        .multi_cartesian_product()
    {
        if test_full_with_swaps(&init_gates, &gate_lookup, size, swaps.clone()) {
            solution = swaps;
            break;
        }
    }

    solution
        .iter()
        .flat_map(|(a, b)| [init_gates[*a].out.clone(), init_gates[*b].out.clone()])
        .sorted()
        .join(",")
}

fn test_full_with_swaps(
    gates: &Gates,
    gate_lookup: &GateLookup,
    size: usize,
    swaps: Vec<(usize, usize)>,
) -> bool {
    for _ in 0..10 {
        let mut fixed_gates = gates.clone();
        for (c1, c2) in swaps.clone() {
            let tmp = fixed_gates[c1].out.clone();
            fixed_gates[c1].out = fixed_gates[c2].out.clone();
            fixed_gates[c2].out = tmp;
        }

        // Ensure we don't overflow
        let x: u64 = rand::random::<u64>() & ((1 << size) - 1);
        let y: u64 = rand::random::<u64>() & ((1 << size) - 1);
        let z = x + y;

        let mut outputs = HashMap::new();
        for i in 0..size {
            run_input(
                &mut fixed_gates,
                gate_lookup,
                (wlabel("x", i), ((x >> i) & 1) == 1),
            )
            .into_iter()
            .for_each(|(_, out, val)| {
                if out.starts_with("z") {
                    outputs.insert(out, val);
                }
            });
            run_input(
                &mut fixed_gates,
                gate_lookup,
                (wlabel("y", i), (y >> i) & 1 == 1),
            )
            .into_iter()
            .for_each(|(_, out, val)| {
                if out.starts_with("z") {
                    outputs.insert(out, val);
                }
            });
        }

        let mut z_got = 0u64;
        for i in 0..size + 1 {
            match outputs.get(&wlabel("z", i)) {
                Some(z_out) => {
                    if *z_out {
                        z_got |= 1 << i;
                    }
                }
                None => return false,
            }
        }

        assert_ne!(z_got, 0);
        if z != z_got {
            return false;
        }
    }
    return true;
}

fn test_bit_with_swaps(
    gates: &Gates,
    gate_lookup: &GateLookup,
    bit: usize,
    swaps: Vec<(usize, usize)>,
) -> bool {
    let mut fixed_gates = gates.clone();
    for (c1, c2) in swaps {
        let tmp = fixed_gates[c1].out.clone();
        fixed_gates[c1].out = fixed_gates[c2].out.clone();
        fixed_gates[c2].out = tmp;
    }

    test_bit(&fixed_gates, &gate_lookup, bit).is_none()
}

fn test_bit(gates: &Gates, gate_lookup: &GateLookup, bit: usize) -> Option<HashSet<usize>> {
    let mut gates = gates.clone();
    // Warm up all bits before and ignore their effects.
    for b in 0..bit {
        run_input(&mut gates, gate_lookup, (wlabel("x", b), false));
        run_input(&mut gates, gate_lookup, (wlabel("y", b), false));
    }

    let mut candidates: HashSet<usize> = HashSet::new();
    let mut failed = false;
    for (x, y, want) in [
        (false, true, true),
        (true, false, true),
        (true, true, false),
    ] {
        let mut gates = gates.clone();
        // Warm up with previous bits
        run_input(&mut gates, gate_lookup, (wlabel("x", bit), x));
        let outputs = run_input(&mut gates, gate_lookup, (wlabel("y", bit), y));

        let mut found = false;
        let mut fired_now = Vec::new();
        for (out_gate, out, out_val) in outputs {
            fired_now.push(out_gate);
            if out == wlabel("z", bit) && out_val == want {
                found = true;
            }
        }

        if !found {
            failed = true;
            for f in fired_now {
                candidates.insert(f);
            }
        }
    }

    if failed {
        Some(candidates)
    } else {
        None
    }
}

fn run_input(
    gates: &mut Gates,
    gate_lookup: &GateLookup,
    signal: (String, bool),
) -> Vec<(usize, String, bool)> {
    let mut outputs = Vec::new();

    // Prepare queue with 0 for all bits below the current
    let mut queue = vec![signal];
    while let Some((in_wire, in_val)) = queue.pop() {
        let input_for = gate_lookup.get(&in_wire);
        if input_for.is_none() {
            continue;
        }
        for gate_idx in input_for.unwrap() {
            if let Some((out_wire, out_val)) = gates[*gate_idx].load(in_val) {
                queue.push((out_wire.clone(), out_val));
                outputs.push((*gate_idx, out_wire, out_val));
            }
        }
    }
    outputs
}

fn wlabel(prefix: &str, num: usize) -> String {
    format!("{}{:02}", prefix, num)
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    advent_of_code_24::test1(TEST_INPUT, "4".to_string(), parse, part1);
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
    advent_of_code_24::test1(TEST_INPUT2, "2024".to_string(), parse, part1);
}
