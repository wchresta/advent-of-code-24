use std::collections::HashMap;

use advent_of_code_24::{digits, input, solve, split_digits};

#[allow(dead_code)]
const TEST_INPUT: &str = "125 17";

fn main() {
    solve("day11", input::single_line_vec, part1, part2);
}

fn part1(stones: &Vec<u64>) -> u64 {
    run_stones(25, stones)
}

fn part2(stones: &Vec<u64>) -> u64 {
    run_stones(75, stones)
}

fn run_stones(steps: u64, stones: &Vec<u64>) -> u64 {
    Runner::new().run_stones(steps, stones)
}

#[derive(Clone, Debug)]
struct Runner {
    cache: HashMap<(u64, u64), u64>,
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            cache: HashMap::new(),
        }
    }

    pub fn run_stones(&mut self, steps: u64, stones: &Vec<u64>) -> u64 {
        stones.iter().map(|s| self.run_stone(steps, *s)).sum()
    }

    pub fn run_stone(&mut self, steps: u64, stone: u64) -> u64 {
        let mut to_calc = Vec::from([(steps, stone)]);
        while let Some((n, s)) = to_calc.pop() {
            if self.cache.contains_key(&(n, s)) {
                continue;
            }

            let next = Runner::stone_step(s);
            if n == 1 {
                self.cache.insert((1, s), next.len() as u64);
                continue;
            }

            // Check if all stones are in cache
            let mut new_to_calc = Vec::new();
            let mut result = 0;

            for t in next {
                match self.cache.get(&(n - 1, t)) {
                    None => {
                        new_to_calc.push((n - 1, t));
                    }
                    Some(r) => {
                        result += *r;
                    }
                }
            }

            if new_to_calc.is_empty() {
                // We know all values, so we know this value.
                self.cache.insert((n, s), result);
            } else {
                // There's something to calculate, still.
                // So add this value back and also add the ones we still need after.
                // This ensures the dependencies are calculated before this one.
                to_calc.push((n, s));
                to_calc.append(&mut new_to_calc);
            }
        }
        *self.cache.get(&(steps, stone)).unwrap()
    }

    pub fn stone_step(stone: u64) -> Vec<u64> {
        match stone {
            0 => Vec::from([1]),
            x if digits(x) % 2 == 0 => {
                let (a, b) = split_digits(x);
                Vec::from([a, b])
            }
            x => Vec::from([x * 2024]),
        }
    }
}

#[test]
fn test_part1() {
    test1("125 17", 13, input::single_line_vec, |s| run_stones(5, s));
    test1("125 17", 22, input::single_line_vec, |s| run_stones(6, s));
    test1(TEST_INPUT, 55312, input::single_line_vec, |s| {
        run_stones(25, s)
    });
}
