use std::collections::BinaryHeap;
use std::collections::HashSet;

use advent_of_code_24::square::pos_add;
use advent_of_code_24::square::PosFind;
use advent_of_code_24::square::PosGet;
use advent_of_code_24::square::M;

fn main() {
    advent_of_code_24::solve("day16", parse, part1, part2);
}

type Pos = (isize, isize);
type Dir = (isize, isize);

#[derive(Debug, Clone)]
struct State {
    rect: M<char>,
    start: Pos,
    exit: Pos,
}

const EAST: Dir = (1, 0);
const WEST: Dir = (-1, 0);
const NORTH: Dir = (0, -1);
const SOUTH: Dir = (0, 1);

impl State {
    fn new(m: &M<char>) -> Self {
        let start = m.pos_find('S').unwrap();
        let exit = m.pos_find('E').unwrap();
        State {
            rect: m.to_owned(),
            start,
            exit,
        }
    }

    fn neighbours(&self, pos: Pos, dir: Dir) -> Vec<(i64, Pos, Dir)> {
        let mut dirs = Vec::from([(1i64, pos_add(pos, dir), dir)]);
        if dir.1 == 0 {
            // Going left or right
            dirs.push((1001, pos_add::<isize>(pos, NORTH), NORTH));
            dirs.push((1001, pos_add::<isize>(pos, SOUTH), SOUTH));
        } else {
            // Going up or down
            dirs.push((1001, pos_add::<isize>(pos, WEST), WEST));
            dirs.push((1001, pos_add::<isize>(pos, EAST), EAST));
        }
        dirs.into_iter()
            .filter(|(_s, p, _d)| self.rect.pos_get(*p) != '#')
            .collect()
    }
}

fn parse(s: &str) -> M<char> {
    advent_of_code_24::input::as_vecvec(s)
}

fn part1(m: &M<char>) -> i64 {
    let state = State::new(m);

    let mut to_check = BinaryHeap::new();
    to_check.push((0i64, state.start, EAST));
    let mut seen: HashSet<(Pos, Dir)> = HashSet::new();

    while let Some((score, pos, dir)) = to_check.pop() {
        /* println!(
            "Checking pos {:?} direction {:?} with score {}",
            pos, dir, score
        ); */
        if pos == state.exit {
            return -score;
        }
        for (dscore, new_pos, new_dir) in state.neighbours(pos, dir) {
            if seen.contains(&(new_pos, new_dir)) {
                continue;
            }
            seen.insert((pos, dir));
            to_check.push((score - dscore, new_pos, new_dir));
        }
    }
    0
}

fn part2(m: &M<char>) -> i64 {
    let state = State::new(m);

    let mut score_goal = None;
    let mut to_check = BinaryHeap::new();
    to_check.push((0i64, state.start, EAST, Vec::from([state.start])));
    let mut seen: HashSet<(Pos, Dir)> = HashSet::new();

    let mut best_path_tiles = HashSet::new();
    while let Some((score, pos, dir, path)) = to_check.pop() {
        if let Some(goal) = score_goal {
            if score < goal {
                // Cannot be better
                continue;
            }
        }

        /* println!(
            "Checking pos {:?} direction {:?} with score {}",
            pos, dir, score
        ); */
        if pos == state.exit {
            if score_goal.is_none() {
                score_goal = Some(score);
            }

            if score_goal.unwrap() == score {
                for t in path {
                    best_path_tiles.insert(t);
                }
            }

            continue;
        }

        for (dscore, new_pos, new_dir) in state.neighbours(pos, dir) {
            if seen.contains(&(new_pos, new_dir)) {
                continue;
            }
            seen.insert((pos, dir));
            let mut new_path = path.to_owned();
            new_path.push(new_pos);
            to_check.push((score - dscore, new_pos, new_dir, new_path));
        }
    }
    best_path_tiles.len() as i64
}

#[allow(unused)]
const TEST_INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

#[test]
fn test_part1() {
    advent_of_code_24::test1(TEST_INPUT, 7036, parse, part1);
}

#[test]
fn test_part2() {
    advent_of_code_24::test1(TEST_INPUT, 45, parse, part2);
}
