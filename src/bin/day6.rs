use advent_of_code_24::input;

use nalgebra::DMatrix;
use rayon::prelude::*;
use std::collections::HashSet;

type M = DMatrix<u8>;
type Pos = (usize, usize);
type Dir = (isize, isize);

fn main() {
    let mat: M = input::as_matrix(&input::read("day6"));
    println!("part1 {}", part1(&mat));
    println!("part2 {}", part2(&mat));
}

#[derive(Clone)]
struct Guard {
    mat: M,
    pos: Pos,
    dir: Dir,
    dist: u16,
    seen: HashSet<(Pos, Dir)>,
    found_loop: bool,
}

impl Guard {
    fn from_mat(mat: &M) -> Self {
        let m = mat.clone_owned();
        let mut pos: (usize, usize) = (0, 0);
        for (i, row) in m.row_iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell as char == '^' {
                    pos = (i, j);
                    break;
                }
            }
        }
        Self {
            mat: m,
            pos,
            dir: (-1, 0),
            dist: 0,
            seen: HashSet::from([(pos, (-1, 0))]),
            found_loop: false,
        }
    }

    fn get(&self, i: usize, j: usize) -> Option<u8> {
        self.mat.get((i, j)).map(|x| *x)
    }

    fn peek_forward(&self) -> Option<u8> {
        let (new_i, new_j) = self.forward_pos()?;
        self.get(new_i, new_j)
    }

    fn forward_pos(&self) -> Option<(usize, usize)> {
        let i = self.pos.0.checked_add_signed(self.dir.0)?;
        let j = self.pos.1.checked_add_signed(self.dir.1)?;
        if i >= self.mat.nrows() {
            None
        } else if j >= self.mat.ncols() {
            None
        } else {
            Some((i, j))
        }
    }

    fn step(&mut self) -> bool {
        if let Some(cell) = self.peek_forward() {
            match cell as char {
                '.' | '^' | 'X' => {
                    self.move_forward();
                }
                '#' | 'O' => {
                    self.rotate_right();
                }
                _ => panic!("invalid cell"),
            }

            // Check if we have been here before
            if self.seen.contains(&(self.pos, self.dir)) {
                self.found_loop = true;
                true
            } else {
                false
            }
        } else {
            true
        }
    }

    fn rotate_right(&mut self) {
        self.seen.insert((self.pos, self.dir));
        self.dir = match self.dir {
            (-1, 0) => (0, 1),
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            _ => panic!("invalid dir"),
        };
    }

    fn move_forward(&mut self) {
        self.seen.insert((self.pos, self.dir));
        if self.mat[self.pos] != 'X' as u8 {
            // Only count distinct positions
            self.dist += 1;
        }
        self.mat[self.pos] = 'X' as u8;
        self.pos = self.forward_pos().unwrap();
    }

    #[allow(dead_code)]
    fn display(&self) -> String {
        let mut s = String::new();
        for row in self.mat.row_iter() {
            for cell in row {
                s.push(*cell as char);
            }
            s.push('\n');
        }
        s
    }
}

fn part1(mat: &M) -> u16 {
    let m = mat.clone_owned();
    let mut guard = Guard::from_mat(&m);
    loop {
        if guard.step() {
            break;
        }
    }
    // We wont move off the board, so we have to count the last one extra.
    guard.dist + 1
}

fn part2(mat: &M) -> u16 {
    let m = mat.clone_owned();
    (0..m.nrows())
        .collect::<Vec<usize>>()
        .par_iter()
        .map(|i| {
            let mut found_loops = 0;
            for j in 0..m.ncols() {
                if m[(*i, j)] != '.' as u8 {
                    continue;
                }

                let mut m = m.clone();
                m[(*i, j)] = 'O' as u8;

                let mut guard = Guard::from_mat(&m);
                loop {
                    if guard.step() {
                        break;
                    }
                }
                if guard.found_loop {
                    found_loops += 1;
                }
            }
            found_loops
        })
        .sum()
}

#[test]
fn test_part1() {
    let mat = input::as_matrix(
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
    );

    assert_eq!(part1(&mat), 41);
}

#[test]
fn test_part2() {
    let mat = input::as_matrix(
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
    );

    assert_eq!(part2(&mat), 6);
}
