use advent_of_code_24::{input, solve1, test1};

fn main() {
    solve1("day10", input::as_vecvec, part1);
}

fn part1(bx: &Vec<Vec<u8>>) -> u64 {
    0
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

    pub fn uphills(&self, pos: Pos) -> Vec<Pos> {
        let h = self.heights[pos.1][pos.0];
        [(-1, -1), (1, -1), (-1, 1), (1, 1)]
            .iter()
            .filter_map(|(dx, dy)| {
                let (x, y) = (pos.0 as isize + dx, pos.1 as isize + dy);
                self.iget((x, y))
                    .map(|w| if h + 1 == w { Some((x, y)) } else { None })
            })
            .flatten()
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
    test1(TEST_INPUT1, 36, input::as_vecvec, part1)
}
