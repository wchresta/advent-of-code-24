use std::collections::{HashMap, HashSet};

use advent_of_code_24::input;

type M = Vec<Vec<char>>;

type Coord = (usize, usize);

fn main() {
    let inp: M = input::as_vecvec(&input::read("day8"));
    println!("part1: {}", part1(&inp));
    println!("part2: {}", part2(&inp));
}

fn part1(inp: &M) -> usize {
    let max = (inp.len(), inp[0].len());
    let antinodes: HashSet<Coord> = find_antennas(inp)
        .iter()
        .flat_map(|(_, vs)| {
            make_pairs(vs)
                .iter()
                .flat_map(|(a, b)| leap2(max, (**a, **b)))
                .collect::<Vec<Coord>>()
        })
        .collect();
    antinodes.len()
}

fn part2(inp: &M) -> usize {
    let max = (inp.len(), inp[0].len());
    let antinodes: HashSet<Coord> = find_antennas(inp)
        .iter()
        .flat_map(|(_, vs)| {
            make_pairs(vs)
                .iter()
                .flat_map(|(a, b)| leap(max, (**a, **b)))
                .collect::<Vec<Coord>>()
        })
        .collect();
    antinodes.len()
}

fn find_antennas(m: &M) -> Vec<(char, Vec<Coord>)> {
    let mut antennas = Vec::new();
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            let c = m[i][j];
            if c == '.' {
                continue;
            }
            antennas.push((c, (i, j)));
        }
    }
    // Collect Vec<(A,B)> into Vec<A,Vec<B>>
    let mut groups = HashMap::new();
    for (a, b) in antennas {
        groups.entry(a).or_insert_with(Vec::new).push(b);
    }
    groups.into_iter().collect()
}

fn make_pairs<'a, T>(v: &'a Vec<T>) -> Vec<(&'a T, &'a T)> {
    let mut pairs = Vec::new();
    for i in 0..v.len() - 1 {
        for j in i + 1..v.len() {
            pairs.push((&v[i], &v[j]));
        }
    }
    pairs
}

fn linadd(max: Coord, k: isize, a: Coord, b: (i64,i64)) -> Option<Coord> {
    let akb = ((a.0 as i64 + k as i64*b.0), (a.1 as i64 + k as i64*b.1));
    if akb.0 < 0 || akb.1 < 0 || akb.0 >= max.0 as i64 || akb.1 >= max.1 as i64 {
        None
    } else {
        Some((akb.0 as usize, akb.1 as usize))
    }
}

fn leap2(max: Coord, (a, b): (Coord, Coord)) -> Vec<Coord> {
    let ab = (b.0 as i64 - a.0 as i64, b.1 as i64 - a.1 as i64);
    let mut res = Vec::new();
    if let Some(coord) = linadd(max, -1, a, ab) {
        res.push(coord);
    }
    if let Some(coord) = linadd(max, 2, a, ab) {
        res.push(coord);
    }
    res
}

fn leap(max: Coord, (a,b): (Coord, Coord)) -> Vec<Coord> {
    let ab = (b.0 as i64 - a.0 as i64, b.1 as i64 - a.1 as i64);
    let mut res = Vec::new();
    // Positive
    for k in 0.. {
        if let Some(coord) = linadd(max, k, a, ab) {
            // k = 0 is a, k = 1 is b
            res.push(coord);
        } else {
            break;
        }
    }
    // Negative
    for k in 1.. {
        if let Some(coord) = linadd(max, -k, a, ab) {
            res.push(coord);
        } else {
            break;
        }
    }
    res
}

#[test]
fn test_part1() {
    let inp = input::as_vecvec(
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
    );
    assert_eq!(part1(&inp), 14);
}

#[test]
fn test_part2() {
    let inp = input::as_vecvec(
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
    );
    assert_eq!(part2(&inp), 34);
}

