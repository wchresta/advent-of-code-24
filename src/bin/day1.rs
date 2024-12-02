use std::{collections::HashMap, iter::zip};

use advent_of_code_24::input;

fn main() {
    println!("Hello, world!");
    let content: Vec<[i64;2]> = input::read_i64("day1");
    part1(&content);
    part2(&content);
}

fn part1(content: &Vec<[i64;2]>) {
    let mut lefts = Vec::new();
    let mut rights = Vec::new();
    for [left, right] in content {
        lefts.push(left);
        rights.push(right);
    }

    lefts.sort();
    rights.sort();

    println!("part1: {}", zip(lefts, rights).map(|(x,y)| (y-x).abs()).sum::<i64>())
}

fn part2(content: &Vec<[i64;2]>) {
    let mut right_counts = HashMap::<i64,i64>::new();
    for [_, right] in content {
        match right_counts.get_mut(&right) {
            Some(x) => {*x += 1;},
            None => {right_counts.insert(*right, 1);},
        }
    }

    let mut score = 0;
    for [left, _] in content {
        match right_counts.get(&left) {
            Some(count) => score += left * count,
            _ => {},
        }
    }

    println!("part2: {}", score)
}