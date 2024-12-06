use advent_of_code_24::input;
use nalgebra::DMatrix;

type M = DMatrix<u8>;

fn main() {
    let mat: M = input::as_matrix(&input::read("day6"));
    println!("part1 {}", part1(&mat));
}

#[derive(Clone)]
struct Guard {
    mat: M,
    pos: (usize, usize),
    dir: (isize, isize),
    dist: u16,
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
                '#' => {
                    self.rotate_right();
                }
                _ => panic!("invalid cell"),
            }
            false
        } else {
            true
        }
    }

    fn rotate_right(&mut self) {
        self.dir = match self.dir {
            (-1, 0) => (0, 1),
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            _ => panic!("invalid dir"),
        };
    }

    fn move_forward(&mut self) {
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
