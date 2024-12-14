extern crate nalgebra as na;

#[allow(unused_imports)]
use advent_of_code_24::{solve, test1};
use itertools::Itertools;
use na::{Vector2, Vector4};
use regex::Regex;
use rayon::prelude::*;

type V = Vector2<i64>;

fn main() {
    solve("day14", parse, part1, part2);
}

fn parse(inp: &str) -> Vec<(V,V)> {
    let r = Regex::new(r"(?s)p=(\d+),(\d+) v=(-?\d+),(-?\d+)\s*").unwrap();

    r.captures_iter(inp).map(|m| {
        let (px,py,vx,vy) = m.iter().skip(1).map(|c| c.unwrap().as_str().parse().unwrap()).next_tuple().unwrap();
        (V::from([px,py]), V::from([vx,vy]))
    }).collect()
}

fn part1(puzzles: &Vec<(V,V)>) -> u64 {
    simulate1(puzzles, 101, 103)
}

fn simulate1(puzzles: &Vec<(V,V)>, width: i64, height: i64) -> u64 {
    let mid_x = (width-1)/ 2; // 50 for width=101
    let mid_y = (height-1)/2; // 51 for height=103
    let quadrants = puzzles.par_iter().map(|(pos,vel)| {
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
            0 => Vector4::new(1,0,0,0),
            1 => Vector4::new(0,1,0,0),
            2 => Vector4::new(0,0,1,0),
            3 => Vector4::new(0,0,0,1),
            _ => panic!("unknown quadrant"),
        }
    }).reduce(|| Vector4::zeros(), |v,w| v + w);
    println!("{}", quadrants);
    quadrants.product()
}

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

fn part2(puzzles: &Vec<(V,V)>) -> u64 {
    todo!()
}