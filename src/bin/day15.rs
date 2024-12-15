use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use advent_of_code_24::{input, solve, test1};

type M = Vec<Vec<char>>;

type Pos = (isize, isize);

fn main() {
    solve("day15", parse, part1, part2);
}

fn parse(content: &str) -> (M, Vec<char>) {
    let (m, vs) = content.split_once("\n\n").unwrap();
    (input::as_vecvec(m), vs.chars().collect())
}

fn part1((m, vs): &(M, Vec<char>)) -> i64 {
    let mut m = m.to_owned();
    let mut pos: Pos = (0, 0);
    for (j, line) in m.iter().enumerate() {
        for (i, x) in line.iter().enumerate() {
            if *x == '@' {
                pos = (i as isize, j as isize);
                break;
            }
        }
    }
    for v in vs {
        pos = step1(&mut m, pos, *v);
    }
    // println!("=======\n{}", m.iter().map(|line| line.iter().collect::<String>()).collect::<Vec<String>>().join("\n"));
    m.iter()
        .enumerate()
        .map(|(j, line)| {
            line.iter()
                .enumerate()
                .map(|(i, x)| if *x == 'O' { i + 100 * j } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>() as i64
}

fn part2((m, vs): &(M, Vec<char>)) -> i64 {
    let mut m: M = m
        .iter()
        .map(|line| {
            line.iter()
                .flat_map(|x| match *x {
                    '#' => vec!['#', '#'],
                    'O' => vec!['[', ']'],
                    '.' => vec!['.', '.'],
                    '@' => vec!['@', '.'],
                    _ => vec![],
                })
                .collect()
        })
        .collect();

    let mut pos: Pos = (0, 0);
    for (j, line) in m.iter().enumerate() {
        for (i, x) in line.iter().enumerate() {
            if *x == '@' {
                pos = (i as isize, j as isize);
                break;
            }
        }
    }

    for v in vs {
        /*
        println!(
            "======= :: {}\n{}",
            v,
            m.iter()
                .map(|line| line.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        );
        */
        pos = step2(&mut m, pos, *v);
    }
    // println!("=======\n{}", m.iter().map(|line| line.iter().collect::<String>()).collect::<Vec<String>>().join("\n"));
    m.iter()
        .enumerate()
        .map(|(j, line)| {
            line.iter()
                .enumerate()
                .map(|(i, x)| if *x == '[' { i + 100 * j } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>() as i64
}

fn get(m: &M, p: Pos) -> char {
    m[p.1 as usize][p.0 as usize]
}

fn set(m: &mut M, p: Pos, c: char) {
    m[p.1 as usize][p.0 as usize] = c;
}

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

fn step1(m: &mut M, pos: Pos, v: char) -> Pos {
    let dir: Pos = match v {
        '>' => (1, 0),
        'v' => (0, 1),
        '<' => (-1, 0),
        '^' => (0, -1),
        _ => {
            return pos;
        }
    };
    let move_to = add(pos, dir);
    let mut look = move_to.clone();
    loop {
        match get(m, look) {
            'O' => {
                look = add(look, dir);
                continue;
            }
            '#' => {
                return pos;
            }
            _ => break,
        }
    }
    if get(m, move_to) == 'O' {
        // We need to move the stone first
        set(m, look, 'O');
    }
    // We move
    set(m, pos, '.');
    set(m, move_to, '@');
    move_to
}

fn step2(m: &mut M, pos: Pos, v: char) -> Pos {
    assert_eq!(get(m, pos), '@');
    let dir: Pos = match v {
        '>' => (1, 0),
        'v' => (0, 1),
        '<' => (-1, 0),
        '^' => (0, -1),
        _ => {
            return pos;
        }
    };

    let mut needs_move = Vec::from([pos]);
    let mut needs_check = VecDeque::from([HashSet::from([pos])]);
    while let Some(layer) = needs_check.pop_front() {
        // layer contains elements we know can and must be moved.
        // We also know they are already in needs_move.
        let mut new_layer = HashSet::new();
        for p in layer {
            let p_new = add(p, dir);
            match get(m, p_new) {
                '#' => return pos,
                '[' => {
                    new_layer.insert(p_new);
                    if dir.1 != 0 {
                        // Because p_new is [, we know p_new + (1,0) must be ']'
                        new_layer.insert(add(p_new, (1, 0)));
                    }
                }
                ']' => {
                    new_layer.insert(p_new);
                    if dir.1 != 0 {
                        // Because p_new is ], we know p_new + (-1,0) must be '['
                        new_layer.insert(add(p_new, (-1, 0)));
                    }
                }
                _ => {}
            }
        }
        if !new_layer.is_empty() {
            // Ensure new_layer is moved when we move things.
            needs_check.push_back(new_layer.clone());
            needs_move.append(&mut Vec::from_iter(new_layer));
        }
    }

    // All moves would be successful, so perform them.
    needs_move.reverse();
    for p in needs_move {
        set(m, add(p, dir), get(m, p));
        set(m, p, '.');
    }

    add(pos, dir)
}

#[allow(dead_code)]
const TEST_SMOL: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

#[allow(dead_code)]
const TEST_INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

#[test]
fn test_part1() {
    test1(TEST_SMOL, 2028, parse, part1);
    test1(TEST_INPUT, 10092, parse, part1);
}

#[test]
fn test_part2() {
    test1(TEST_INPUT, 9021, parse, part2);
}
