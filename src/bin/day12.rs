use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use advent_of_code_24::{input, solve, square, test1};

type Pos = (usize, usize);

fn main() {
    solve("day12", input::as_vecvec, part1, part2);
}

fn part1(garden: &Vec<Vec<char>>) -> u64 {}

fn partition(garden: &Vec<Vec<char>>) -> Vec<Vec<Pos>> {
    let mut regions = Vec::new();
    let mut seen = HashSet::new();

    for y in 0..garden.len() {
        for x in 0..garden[y].len() {
            let t = garden[y][x];
        }
    }
    regions
}

fn explore_region(garden: &Vec<Vec<char>>, (x, y): Pos) -> Vec<Pos> {
    let r = garden[y][x];
    let mut stack = Vec::from([(x, y)]);
    let mut seen = HashSet::new();

    let mut found = Vec::new();
    while let Some(pos) = stack.pop() {
        square::straight_neighbours(garden, pos, |_, v| *v == r).for_each(|(p, _)| {
            found.push(p.to_owned());
            if !seen.contains(&p) {
                stack.push(p.to_owned());
                seen.insert(p);
            }
        })
    }
    found
}

fn part2(stones: &Vec<Vec<char>>) -> u64 {
    todo!()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

#[test]
fn test_part1() {
    test1("125 17", 1930, input::as_vecvec, part1);
}
