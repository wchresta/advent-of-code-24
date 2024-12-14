extern crate nalgebra as na;

use std::collections::HashSet;

#[allow(unused_imports)]
use advent_of_code_24::{solve, test1};
use itertools::Itertools;
use na::{Vector2, Vector4};
use rayon::prelude::*;
use regex::Regex;

type V = Vector2<i64>;

fn main() {
    solve("day14", parse, part1, part2);
}

fn parse(inp: &str) -> Vec<(V, V)> {
    let r = Regex::new(r"(?s)p=(\d+),(\d+) v=(-?\d+),(-?\d+)\s*").unwrap();

    r.captures_iter(inp)
        .map(|m| {
            let (px, py, vx, vy) = m
                .iter()
                .skip(1)
                .map(|c| c.unwrap().as_str().parse().unwrap())
                .next_tuple()
                .unwrap();
            (V::from([px, py]), V::from([vx, vy]))
        })
        .collect()
}

fn part1(puzzles: &Vec<(V, V)>) -> i64 {
    simulate1(puzzles, 101, 103)
}

fn simulate1(puzzles: &Vec<(V, V)>, width: i64, height: i64) -> i64 {
    let mid_x = (width - 1) / 2; // 50 for width=101
    let mid_y = (height - 1) / 2; // 51 for height=103
    let quadrants = puzzles
        .par_iter()
        .map(|(pos, vel)| {
            let mut end = pos.clone();
            for _ in 0..100 {
                end = end + vel;
                end.x = (end.x + width) % width;
                end.y = (end.y + height) % height;
            }
            let mut quadrant = 0;
            if end.x == mid_x || end.y == mid_y {
                return Vector4::zeros();
            }
            if end.x > mid_x {
                quadrant += 1;
            }
            if end.y > mid_y {
                quadrant += 2;
            }

            match quadrant {
                0 => Vector4::new(1, 0, 0, 0),
                1 => Vector4::new(0, 1, 0, 0),
                2 => Vector4::new(0, 0, 1, 0),
                3 => Vector4::new(0, 0, 0, 1),
                _ => panic!("unknown quadrant"),
            }
        })
        .reduce(|| Vector4::zeros(), |v, w| v + w);
    println!("{}", quadrants);
    quadrants.product()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

#[test]
fn test_part1() {
    test1(TEST_INPUT, 12, parse, |p| simulate1(p, 11, 7));
}

fn part2(puzzles: &Vec<(V, V)>) -> i64 {
    simulate2(puzzles, 101, 103)
}

fn simulate2(puzzles: &Vec<(V, V)>, width: i64, height: i64) -> i64 {
    let mut puz = puzzles.to_owned();
    for t in 1..50_000 {
        step(&mut puz, width, height);

        let pos: HashSet<(i64, i64)> = HashSet::from_iter(puz.iter().map(|(p, _)| (p.x, p.y)));
        for x in 0..width {
            let mut in_a_row = 0;
            for y in 0..height {
                if pos.contains(&(x, y)) {
                    in_a_row += 1;
                } else {
                    in_a_row = 0;
                }

                if in_a_row > 10 {
                    return t;
                }
            }
        }
    }
    // Not found :(
    0
}

fn step(puzzles: &mut Vec<(V, V)>, width: i64, height: i64) {
    puzzles.par_iter_mut().for_each(|(pos, vel)| {
        *pos = *pos + *vel;
        pos.x = (pos.x + width) % width;
        pos.y = (pos.y + height) % height;
    });
}
#[test]
fn test_part2() {
    test1(TEST_INPUT, 12, parse, |p| simulate2(p, 11, 7));
}
