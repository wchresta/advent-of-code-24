use std::collections::{BinaryHeap, HashMap};

use advent_of_code_24::{
    input,
    square::{pos_add, straight_neighbours, Counter, Pos, PosFind, M},
};
use itertools::Itertools;

fn main() {
    advent_of_code_24::solve("day20", parse, part1, part2);
}

#[derive(Debug, Clone)]
struct S {
    maze: M<char>,
    start_pos: Pos,
    end_pos: Pos,
}

impl S {
    fn find_distances(&self, start: Pos) -> HashMap<Pos, isize> {
        let mut heap = BinaryHeap::new();
        let mut distances = HashMap::new();

        distances.insert(start, 0);
        heap.push((0, start));

        while let Some((score, pos)) = heap.pop() {
            for (npos, _) in straight_neighbours(&self.maze, pos, |_, t| *t != '#') {
                if distances.contains_key(&npos) {
                    continue;
                }
                distances.insert(npos, -score + 1);
                heap.push(((score - 1), npos))
            }
        }
        distances
    }
}

fn parse(s: &str) -> S {
    let maze = input::as_vecvec(s);
    let start_pos = maze.pos_find('S').unwrap();
    let end_pos = maze.pos_find('E').unwrap();
    S {
        maze,
        start_pos,
        end_pos,
    }
}

fn part1(s: &S) -> isize {
    saving_min(s, 2, 100).iter().map(|(_, s)| s).sum()
}

fn part2(s: &S) -> isize {
    saving_min(s, 20, 100).iter().map(|(_, s)| s).sum()
}

fn savings(s: &S, dist: isize) -> Vec<(isize, isize)> {
    let start_distances = s.find_distances(s.start_pos);
    let end_distances = s.find_distances(s.end_pos);
    let baseline = *start_distances.get(&s.end_pos).unwrap();

    let mut savings = Counter::new();
    for (pos, cost) in start_distances.iter() {
        for dx in -dist..dist + 1 {
            for dy in -dist + dx.abs()..dist - dx.abs() + 1 {
                if let Some(rest_cost) = end_distances.get(&pos_add(*pos, (dx, dy))) {
                    let total_cost = cost + rest_cost + dx.abs() + dy.abs();
                    let saved = baseline - total_cost;
                    if saved > 0 {
                        savings.inc(saved);
                    }
                }
            }
        }
    }
    savings
        .counts
        .into_iter()
        .sorted_by_key(|(key, _)| *key)
        .collect_vec()
}

fn saving_min(s: &S, dist: isize, min: isize) -> Vec<(isize, isize)> {
    savings(s, dist)
        .into_iter()
        .filter(|(s, _)| *s >= min)
        .collect_vec()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

#[test]
fn test_part1() {
    advent_of_code_24::test1(
        TEST_INPUT,
        vec![
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1),
        ],
        parse,
        |s| savings(s, 2),
    );
}

#[test]
fn test_part2() {
    advent_of_code_24::test1(
        TEST_INPUT,
        vec![
            (50, 32),
            (52, 31),
            (54, 29),
            (56, 39),
            (58, 25),
            (60, 23),
            (62, 20),
            (64, 19),
            (66, 12),
            (68, 14),
            (70, 12),
            (72, 22),
            (74, 4),
            (76, 3),
        ],
        parse,
        |s| saving_min(s, 20, 50),
    );
}
