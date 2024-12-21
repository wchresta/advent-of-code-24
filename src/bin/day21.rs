extern crate lazy_static;

use advent_of_code_24::{
    input,
    square::{pos_sub, Pos},
};
use lazy_static::lazy_static;
use memoize::memoize;
use std::collections::HashMap;

fn main() {
    advent_of_code_24::solve("day21", input::as_lines, part1, part2);
}

type S = Vec<String>;

/*
  0   1   2
+---+---+---+
| 7 | 8 | 9 | 0
+---+---+---+
| 4 | 5 | 6 | 1
+---+---+---+
| 1 | 2 | 3 | 2
+---+---+---+
    |^0 | A | 3
+---+---+---+
| < | v | > | 4
+---+---+---+
 */

lazy_static! {
    static ref POSITIONS: HashMap<char, Pos> = HashMap::from([
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        ('0', (1, 3)),
        ('^', (1, 3)),
        ('A', (2, 3)),
        ('<', (0, 4)),
        ('v', (1, 4)),
        ('>', (2, 4)),
    ]);
}

fn part1(s: &S) -> usize {
    solve(s, 3)
}

fn part2(s: &S) -> usize {
    solve(s, 26)
}

fn solve(s: &S, depth: u8) -> usize {
    s.into_iter().map(|c| solve_one(c, depth)).sum()
}

fn solve_one(c: &str, depth: u8) -> usize {
    c[..c.len() - 1].parse::<usize>().unwrap() * find_shortest_len(c.to_string(), depth)
}

#[memoize]
fn find_shortest_len(code: String, depth: u8) -> usize {
    if depth == 0 {
        return code.len();
    }

    let mut curr = 'A';
    let mut length = 0;
    for tgt in code.chars() {
        // Find shortest path between cur and tgt
        let next_paths = paths(curr, tgt);
        length += next_paths
            .into_iter()
            .map(|p| find_shortest_len(p, depth - 1))
            .min()
            .unwrap();
        curr = tgt;
    }
    length
}

/*
  0   1   2
+---+---+---+
| 7 | 8 | 9 | 0
+---+---+---+
| 4 | 5 | 6 | 1
+---+---+---+
| 1 | 2 | 3 | 2
+---+---+---+
    |^0 | A | 3
+---+---+---+
| < | v | > | 4
+---+---+---+
 */

fn paths(from: char, to: char) -> Vec<String> {
    if from == to {
        return Vec::from(["A".to_owned()]);
    }

    let from_pos = *POSITIONS.get(&from).unwrap();
    let to_pos = *POSITIONS.get(&to).unwrap();

    let (dx, dy) = pos_sub(to_pos, from_pos);

    let horizontal = if dx > 0 {
        ">".repeat(dx as usize)
    } else if dx < 0 {
        "<".repeat(-dx as usize)
    } else {
        "".to_string()
    };

    let vertical = if dy > 0 {
        "v".repeat(dy as usize)
    } else if dy < 0 {
        "^".repeat(-dy as usize)
    } else {
        "".to_string()
    };

    if from_pos.0 == 0 && to_pos.1 == 3 {
        // We go from the first column to row 3
        // So we always go horizontal fist
        return Vec::from([horizontal + &vertical + "A"]);
    }
    if from_pos.1 == 3 && to_pos.0 == 0 {
        // We go from row 3 to column 0
        // So we always go vertical first
        return Vec::from([vertical + &horizontal + "A"]);
    }
    if dx == 0 || dy == 0 {
        // We go only horizontal or vertical
        return Vec::from([horizontal + &vertical + "A"]);
    }
    // We go both horizontal and vertical
    Vec::from([
        horizontal.clone() + &vertical + "A",
        vertical + &horizontal + "A",
    ])
}

#[allow(dead_code)]
const TEST_INPUT: &str = "029A
980A
179A
456A
379A";

#[test]
fn test_part1() {
    advent_of_code_24::test1(TEST_INPUT, 126384, input::as_lines, part1);
}
