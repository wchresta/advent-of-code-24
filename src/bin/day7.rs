use advent_of_code_24::input;

fn main() {
    let inp = input::as_lines(&input::read("day7"));
    println!("part1: {}", part1(&inp));
    println!("part2: {}", part2(&inp));
}

fn part1(lines: &Vec<String>) -> i64 {
    lines
        .iter()
        .map(Eqn::new_part1)
        .filter(Eqn::has_soln)
        .map(|e| e.tot)
        .sum()
}

fn part2(lines: &Vec<String>) -> i64 {
    lines
        .iter()
        .map(Eqn::new_part2)
        .filter(Eqn::has_soln)
        .map(|e| e.tot)
        .sum()
}

#[derive(Clone)]
struct Eqn {
    tot: i64,
    terms: Vec<i64>,
    part2: bool,
}

impl Eqn {
    fn new_part1(line: &String) -> Self {
        let (tot_str, val_str) = line.split_once(": ").unwrap();
        Self {
            tot: tot_str.parse().expect(&format!("cannot parse {tot_str}")),
            terms: val_str
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
            part2: false,
        }
    }

    fn new_part2(line: &String) -> Self {
        let (tot_str, val_str) = line.split_once(": ").unwrap();
        Self {
            tot: tot_str.parse().expect(&format!("cannot parse {tot_str}")),
            terms: val_str
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
            part2: true,
        }
    }

    fn has_soln(&self) -> bool {
        let mut stack: Vec<Eqn> = Vec::from([self.to_owned()]);
        while let Some(mut eq) = stack.pop() {
            if eq.terms.is_empty() {
                if eq.tot == 0 {
                    return true;
                }
                continue;
            }

            if eq.terms.len() == 1 {
                if eq.terms[0] == eq.tot {
                    return true;
                }
                continue;
            }
            // No immediate solution

            let last_term = eq.terms.pop().unwrap();

            if eq.tot % last_term == 0 {
                stack.push(Eqn {
                    tot: eq.tot / last_term,
                    terms: eq.terms.clone(),
                    part2: eq.part2,
                })
            }
            if last_term < eq.tot {
                stack.push(Eqn {
                    tot: eq.tot - last_term,
                    terms: eq.terms.clone(),
                    part2: eq.part2,
                })
            }
            if eq.part2 {
                let term_len = last_term.checked_ilog10().unwrap_or(0) + 1;
                if eq.tot % 10_i64.pow(term_len) == last_term {
                    stack.push(Eqn {
                        tot: eq.tot / 10_i64.pow(term_len),
                        terms: eq.terms.clone(),
                        part2: eq.part2,
                    })
                }
            }
        }
        false
    }
}

#[test]
fn test_part1() {
    let inp = input::as_lines(
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
    );
    assert_eq!(part1(&inp), 3749);
}

#[test]
fn test_part2() {
    let inp = input::as_lines(
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
    );
    assert_eq!(part2(&inp), 11387);
}
