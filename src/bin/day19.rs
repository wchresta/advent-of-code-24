use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::Itertools;

fn main() {
    advent_of_code_24::solve("day19", parse, part1, part2);
}

struct P {
    stripes: Vec<String>,
    towels: Vec<String>,
}

fn parse(s: &str) -> P {
    let mut l = s.lines().map(String::from);
    let mut stripes: Vec<String> = l.next().unwrap().split(", ").map(String::from).collect();
    let towels = l.skip(1).collect();
    // It's better for search algorithms to have chunky things first.
    stripes.sort_by_key(|s| s.len());
    stripes.reverse();
    println!("{:?}", stripes);
    P { stripes, towels }
}

fn part1(p: &P) -> i64 {
    p.towels.iter().filter(|t| can_be_made(*t, &p.stripes)).count() as i64
}

fn can_be_made(towel: &str, ps: &Vec<String>) -> bool {
    let mut stack = Vec::from([towel]);
    while let Some(t) = stack.pop() {
        for p in ps {
            if !t.starts_with(p) {
                continue;
            }
            let left_piece = &t[p.len()..];
            if left_piece == "" {
                return true
            }
            stack.push(left_piece);
        }
    }
    false
}

struct TowelMaker {
    stripes: HashMap<String, usize>,
}

impl TowelMaker {
    fn new(p: &P) -> Self {
        let hash = p.stripes.iter().map(|s|
            (s.to_string(), brute_force_manufacture(s, &p.stripes))
        );
        TowelMaker {
            stripes: HashMap::from_iter(hash),
        }
    }

    fn count_ways(&mut self, towel: &str) -> usize {
        if let Some(count) = self.stripes.get(towel) {
            return *count;
        }

        self.reduce_once(towel);
        0
    }

    fn reduce_once(&mut self, towel: &str) -> &str {
        let mut heap = BinaryHeap::from_iter(self.stripes.iter().map(|(k,v)| (k.len(), k,v)));

        while let Some((_,chunk,_)) = heap.pop() {
            if !towel.starts_with(chunk) {
                continue
            }
        }
        ""
    }
}

// We have singles for w,u,r,g
// That means only b (black) are non-singles.
// We can probably use b's to split our problems easily.
fn part2(p: &P) -> i64 {
    let mut maker = TowelMaker::new(p);
    for (k,v) in &maker.stripes {
        println!("{}: {}", k, v);
    }
    p.towels.iter().map(|t| maker.count_ways(t)).sum::<usize>() as i64
}

fn brute_force_manufacture(towel: &str, stripes: &Vec<String>) -> usize {
    let mut stack = Vec::from([towel]);
    let mut count = 0;
    while let Some(t) = stack.pop() {
        for p in stripes {
            if !t.starts_with(p) {
                continue;
            }
            let left_piece = &t[p.len()..];
            if left_piece == "" {
                count += 1;
                continue;
            }
            stack.push(left_piece);
        }
    }
    count
}

#[allow(dead_code)]
const TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

#[test]
fn test_part1() {
    advent_of_code_24::test1(TEST_INPUT, 6, parse, part1);
}

#[test]
fn test_part2() {
    advent_of_code_24::test1(TEST_INPUT, 16, parse, part2);
}
