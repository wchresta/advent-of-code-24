use std::{
    collections::{BinaryHeap, HashMap, HashSet},
};

use advent_of_code_24::{
    input,
    square::{straight_neighbours, PeekStraight, Pos, PosFind, PosIter, PosSet, M},
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
    fn shortest_path(&self) -> Option<isize> {
        let mut heap = BinaryHeap::new();
        let mut seen = HashSet::from([self.start_pos]);

        heap.push((0, self.start_pos));

        while let Some((score, pos)) = heap.pop() {
            if pos == self.end_pos {
                return Some(-score);
            }

            for (npos, _) in straight_neighbours(&self.maze, pos, |_, t| *t != '#') {
                if seen.contains(&npos) {
                    continue;
                }
                seen.insert(npos);
                heap.push(((score - 1), npos))
            }
        }
        None
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
    savings(s)
        .iter()
        .filter_map(|(s, a)| if *s >= 100 { Some(a) } else { None })
        .sum()
}

fn savings(s: &S) -> Vec<(isize, isize)> {
    let baseline = s.shortest_path().unwrap();
    let mut savings: HashMap<isize, isize> = HashMap::new();

    s.maze.pos_iter().for_each(|(pos, t)| {
        if t != '#' {
            return;
        }
        {
            let (n, s, e, w) = (
                *s.maze.peek_north(pos).unwrap_or(&'#'),
                *s.maze.peek_south(pos).unwrap_or(&'#'),
                *s.maze.peek_east(pos).unwrap_or(&'#'),
                *s.maze.peek_west(pos).unwrap_or(&'#'),
            );
            let can_jump = (n != '#' && s != '#') || (e != '#' && w != '#');
            if !can_jump {
                return;
            }
        }

        let mut s2: S = s.clone();
        s2.maze.pos_set(pos, '.');
        let score = s2.shortest_path().unwrap();

        if score < baseline {
            let saving = baseline - score;
            savings
                .entry(saving)
                .and_modify(|a| {
                    *a += 1;
                })
                .or_insert(1);
        }
    });

    savings.into_iter().sorted().collect_vec()
}

fn part2(_s: &S) -> isize {
    0
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
        savings,
    );
}

#[test]
fn test_part2() {
    advent_of_code_24::test1(TEST_INPUT, 0, parse, part2);
}
