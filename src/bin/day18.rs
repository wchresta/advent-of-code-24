use std::collections::{BinaryHeap, HashSet};

use advent_of_code_24::square::pos_add;

fn main() {
    advent_of_code_24::solve("day18", parse, part1, part2);
}

type Pos = (isize, isize);
type S = Vec<Pos>;

fn parse(s: &str) -> S {
    s.lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn part1(s: &S) -> String {
    drop_and_solve_maze(s, 1024, 70).to_string()
}

fn drop_and_solve_maze(s: &S, take: usize, max: isize) -> i64 {
    let walls: HashSet<Pos> = HashSet::from_iter(s.clone().into_iter().take(take));
    let mut seen = HashSet::from([(0, 0)]);
    let mut stack = BinaryHeap::new();
    stack.push((0, (0, 0)));

    while let Some((cost, pos)) = stack.pop() {
        if pos == (max, max) {
            return -cost as i64;
        }
        for n in neighbours(max, pos) {
            if walls.contains(&n) {
                continue;
            }
            if seen.contains(&n) {
                continue;
            }
            seen.insert(n.clone());
            stack.push((cost - 1, n));
        }
    }
    0
}

fn neighbours(max: isize, pos: Pos) -> Vec<Pos> {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .iter()
        .map(|d| pos_add::<isize>(pos, *d))
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x <= max && *y <= max)
        .collect()
}

fn part2(s: &S) -> String {
    find_failure(s, 1024, 70)
}

fn find_failure(s: &S, min_fall: usize, max: isize) -> String {
    // Binary search
    // a b are number of elements that are to be used.
    let mut a = min_fall;
    let mut b = s.len();

    while a + 1 < b {
        let c = (a + b) / 2;
        if drop_and_solve_maze(s, c, max) > 0 {
            // There is a solution
            a = c;
        } else {
            // There is no solution
            b = c - 1;
        }
    }
    // a has a path, b does not have a path.
    // So the b'ts element (index b-1) does not allow any path.
    let pos = s[b - 1];
    format!("{},{}", pos.0, pos.1)
}

#[allow(dead_code)]
const TEST_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

#[test]
fn test_part1() {
    advent_of_code_24::test1(TEST_INPUT, 22, parse, |s| drop_and_solve_maze(s, 12, 6));
}

#[test]
fn test_part2() {
    advent_of_code_24::test1(TEST_INPUT, String::from("6,1"), parse, |s| {
        find_failure(s, 1, 6)
    });
}
