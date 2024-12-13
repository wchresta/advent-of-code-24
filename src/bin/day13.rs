extern crate nalgebra as na;

#[allow(unused_imports)]
use advent_of_code_24::{solve, test1};
use itertools::Itertools;
use na::linalg::{self};
use na::{Matrix2, Vector2};
use regex::Regex;

fn main() {
    solve("day13", parse, part1, part2);
}

#[derive(Debug, Clone)]
struct Puzzle {
    ab: Matrix2<f64>,
    p: Vector2<f64>,
}

fn parse(content: &str) -> Vec<Puzzle> {
    let puzzle_p = Regex::new(
        r"(?s)Button A: X\+(\d+), Y\+(\d+)\s*
Button B: X\+(\d+), Y\+(\d+)\s*
Prize: X=(\d+), Y=(\d+)\s*",
    )
    .unwrap();

    puzzle_p
        .captures_iter(content)
        .map(|m| {
            let (ax, ay, bx, by, px, py) = m
                .iter()
                .skip(1)
                .map(|c| c.unwrap().as_str().parse().unwrap())
                .next_tuple()
                .unwrap();
            Puzzle {
                ab: Matrix2::from([[ax, ay], [bx, by]]),
                p: Vector2::from([px, py]),
            }
        })
        .collect_vec()
}

fn part1(puzzles: &Vec<Puzzle>) -> i64 {
    let mut score = 0;
    for p in puzzles {
        if let Some(soln) = linalg::LU::new(p.ab).solve(&p.p) {
            let (xf, yf) = (soln.get(0).unwrap(), soln.get(1).unwrap());
            let (x, y) = (xf.round(), yf.round());
            if (x - xf).abs() < 0.00001 && (y - yf).abs() < 0.00001 {
                score += 3 * x as i64 + y as i64
            }
        }
    }
    score
}

#[allow(dead_code)]
const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

#[test]
fn test_part1() {
    test1(TEST_INPUT, 480, parse, part1);
}

const PART2_MUL: f64 = 10000000000000f64;

fn part2(puzzles: &Vec<Puzzle>) -> i64 {
    let mut score = 0;
    for p in puzzles {
        if let Some(soln) = linalg::LU::new(p.ab).solve(&p.p.add_scalar(PART2_MUL)) {
            let (xf, yf) = (soln.get(0).unwrap(), soln.get(1).unwrap());
            let (x, y) = (xf.round(), yf.round());
            if (x - xf).abs() < 0.001 && (y - yf).abs() < 0.001 {
                score += 3 * x as i64 + y as i64
            }
        }
    }
    score
}
