use std::{cmp::Ordering, collections::HashSet};

use advent_of_code_24::input;

type Rules = Vec<(u32, u32)>;
type Pages = Vec<Vec<u32>>;

fn main() {
    let (rules, pages) = parse(&input::read("day5"));

    println!("part1: {}", part1(&rules, &pages));
    println!("part2: {}", part2(&rules, &pages));
}

fn parse(inp: &str) -> (Rules, Pages) {
    let mut state = 0;
    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut pages: Vec<Vec<u32>> = Vec::new();
    for line in inp.lines() {
        if line.is_empty() {
            state += 1;
            continue;
        }

        match state {
            0 => {
                let xy: Vec<&str> = line.split('|').collect();
                rules.push((xy[0].parse().unwrap(), xy[1].parse().unwrap()));
            }
            _ => {
                pages.push(line.split(',').map(|n| n.parse().unwrap()).collect());
            }
        }
    }

    (rules, pages)
}

fn part1(rules: &Rules, pages: &Pages) -> u32 {
    let mut order: HashSet<(u32, u32)> = HashSet::new();
    for r in rules {
        order.insert(*r);
    }

    let mut value = 0;
    for ps in pages {
        let mut violates = false;
        for i in 0..ps.len() - 1 {
            for j in i + 1..ps.len() {
                if order.contains(&(ps[j], ps[i])) {
                    violates = true;
                    break;
                }
            }
        }
        if !violates {
            value += ps[ps.len() / 2];
        }
    }
    value
}

fn part2(rules: &Rules, pages: &Pages) -> u32 {
    let mut order: HashSet<(u32, u32)> = HashSet::new();
    for r in rules {
        order.insert(*r);
    }

    let mut incorrect: Pages = Vec::new();
    for ps in pages {
        let mut violates = false;
        for i in 0..ps.len() - 1 {
            for j in i + 1..ps.len() {
                if order.contains(&(ps[j], ps[i])) {
                    violates = true;
                    break;
                }
            }
        }
        if violates {
            incorrect.push(ps.to_vec());
        }
    }

    let mut value = 0;
    for mut ps in incorrect {
        ps.sort_by(|a, b| {
            if order.contains(&(*a, *b)) {
                Ordering::Less
            } else if order.contains(&(*b, *a)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        value += ps[ps.len() / 2];
    }

    value
}

#[test]
fn test_part1() {
    let (rules, pages) = parse(
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
    );
    assert_eq!(part1(&rules, &pages), 143)
}

#[test]
fn test_part2() {
    let (rules, pages) = parse(
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
    );
    assert_eq!(part2(&rules, &pages), 123)
}
