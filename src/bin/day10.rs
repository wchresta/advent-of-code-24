use std::collections::HashSet;

use advent_of_code_24::{input, solve};

fn main() {
    solve("day10", input::as_digit_square, part1, part2);
}

fn part1(bx: &Vec<Vec<u8>>) -> u64 {
    let topo = Topo::new(bx.to_vec());
    let mut score = 0;
    for y in 0..bx.len() {
        for x in 0..bx[y].len() {
            if bx[y][x] == 0 {
                score += topo.uphill_paths((x,y)).iter().map(|p| (*p.first().unwrap(), *p.last().unwrap())).collect::<HashSet<(Pos,Pos)>>().len();
            }
        }
    }
    score as u64
}

fn part2(bx: &Vec<Vec<u8>>) -> u64 {
    let topo = Topo::new(bx.to_vec());
    let mut scores: Vec<u64> = Vec::new();
    for y in 0..bx.len() {
        for x in 0..bx[y].len() {
            if bx[y][x] != 0 {
                continue
            }

            let paths = topo.uphill_paths((x,y));
            scores.push(paths.len() as u64)
        }
    }
    scores.iter().sum()
}

type Pos = (usize, usize);

struct Topo {
    heights: Vec<Vec<u8>>,
    h: u8,
    w: u8,
}

impl Topo {
    pub fn new(heights: Vec<Vec<u8>>) -> Self {
        let (h, w) = (heights.len() as u8, heights[0].len() as u8);
        Topo { heights, h, w }
    }

    pub fn uphill_paths(&self, pos: Pos) -> Vec<Vec<Pos>> {
        if self.get(pos).unwrap() == 9 {
            return Vec::from([Vec::from([pos])]);
        }

        self.uphills(pos)
            .iter()
            .filter_map(|p| {
                let tail = self.uphill_paths(*p);
                if tail.is_empty() {
                    None
                } else {
                    Some(
                        tail.iter()
                            .map(|t| {
                                let mut t = t.to_vec();
                                t.insert(0, pos);
                                t
                            })
                            .collect::<Vec<Vec<Pos>>>(),
                    )
                }
            })
            .flatten()
            .collect()
    }

    pub fn uphills(&self, (x, y): Pos) -> Vec<Pos> {
        let h = self.heights[y][x];
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .filter_map(|(dx, dy)| {
                let (x, y) = (x as isize + dx, y as isize + dy);
                self.iget((x, y)).and_then(|w| {
                    if h + 1 == w {
                        Some((x as usize, y as usize))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub fn get(&self, (x, y): Pos) -> Option<u8> {
        if x as u8 >= self.w || y as u8 >= self.h {
            None
        } else {
            Some(self.heights[y][x])
        }
    }

    pub fn iget(&self, (x, y): (isize, isize)) -> Option<u8> {
        if x.is_negative() || y.is_negative() {
            None
        } else {
            self.get((x as usize, y as usize))
        }
    }
}

#[allow(dead_code)]
const TEST_INPUT1: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

#[test]
fn test_part1() {
    test1(TEST_INPUT1, 36, input::as_digit_square, part1)
}

#[test]
fn test_part2() {
    test1(TEST_INPUT1, 81, input::as_digit_square, part2)
}

