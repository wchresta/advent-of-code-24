use std::collections::{HashMap, HashSet};

use advent_of_code_24::{
    input, solve,
    square::{self},
};

type Pos = (isize, isize);

fn main() {
    solve("day12", input::as_vecvec, part1, part2);
}

fn part1(garden: &Vec<Vec<char>>) -> u64 {
    let regions = partition(garden);
    regions.iter().map(score1).sum()
}

fn part2(garden: &Vec<Vec<char>>) -> u64 {
    let regions = partition(garden);
    regions.iter().map(score2).sum()
}

fn partition(garden: &Vec<Vec<char>>) -> Vec<Vec<Pos>> {
    let mut regions = Vec::new();
    let mut seen: HashSet<(isize, isize)> = HashSet::new();

    for y in 0..garden.len() as isize {
        for x in 0..garden[y as usize].len() as isize {
            if seen.contains(&(x, y)) {
                continue;
            }

            seen.insert((x, y));
            let region = explore_region(garden, (x, y));
            for r in &region {
                seen.insert(r.to_owned());
            }
            regions.push(region);
        }
    }
    regions
}

fn explore_region(garden: &Vec<Vec<char>>, (x, y): Pos) -> Vec<Pos> {
    let r = garden[y as usize][x as usize];
    let mut stack = Vec::from([(x, y)]);
    let mut seen = HashSet::from([(x, y)]);
    while let Some(pos) = stack.pop() {
        square::straight_neighbours(garden, pos, |_, v| *v == r).for_each(|(p, _)| {
            if !seen.contains(&p) {
                stack.push(p.to_owned());
                seen.insert(p);
            }
        })
    }
    seen.into_iter().collect()
}

fn score1(region: &Vec<Pos>) -> u64 {
    let h: HashSet<(isize, isize)> =
        HashSet::from_iter(region.iter().map(|(x, y)| (*x as isize, *y as isize)));
    let mut area = 0;
    let mut perimeter = 0;
    for (x, y) in region {
        area += 1;
        perimeter += [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .filter(|(dx, dy)| !h.contains(&(*x as isize + dx, *y as isize + dy)))
            .count();
    }
    (area * perimeter) as u64
}

fn add((a, b): Pos, (c, d): Pos) -> Pos {
    (a + c, b + d)
}

type Dir = (isize, isize);
fn score2(region: &Vec<Pos>) -> u64 {
    let mut region = region.clone();
    // For later algorithms, it's important the region positions are sorted top-left to bottom-right
    region.sort();

    let h: HashSet<&Pos> = HashSet::from_iter(&region);
    let (up, right, down, left) = ((0, -1), (1, 0), (0, 1), (-1, 0));

    let mut edges = HashMap::<&Pos, HashSet<Dir>>::new();
    for pos in &region {
        edges.insert(pos, HashSet::new());
        for dir in [up, right, down, left] {
            if h.contains(&add(*pos, dir)) {
                continue;
            }

            edges.get_mut(pos).unwrap().insert(dir);
        }
    }

    // Remove shared edges with right or down
    for pos in &region {
        for dir in [down, right] {
            let to_remove;
            if let Some(their_edges) = edges.get(&add(*pos, dir)) {
                to_remove = their_edges.to_owned();
            } else {
                continue;
            }

            let v = edges.get_mut(pos).unwrap();
            for e in to_remove {
                v.remove(&e);
            }
        }
    }

    // All edges that are left represent an actual edge.
    let edge_score = edges.values().map(|e| e.len() as u64).sum::<u64>();
    let area = region.len() as u64;
    area * edge_score
}

#[allow(dead_code)]
const TEST_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

#[test]
fn test_part1() {
    advent_of_code_24::test1(TEST_INPUT, 1930, input::as_vecvec, part1);
}

#[test]
fn test_part2() {
    advent_of_code_24::test1(TEST_INPUT, 1206, input::as_vecvec, part2);
}
