use std::collections::HashSet;

use advent_of_code_24::{input, solve, square, test1};

type Pos = (usize, usize);

fn main() {
    solve("day12", input::as_vecvec, part1, part2);
}

fn part1(garden: &Vec<Vec<char>>) -> u64 {
    let regions = partition(garden);
    regions.iter().map(score).sum()
}

fn partition(garden: &Vec<Vec<char>>) -> Vec<Vec<Pos>> {
    let mut regions = Vec::new();
    let mut seen = HashSet::new();

    for y in 0..garden.len() {
        for x in 0..garden[y].len() {
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
    let r = garden[y][x];
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

fn score(region: &Vec<Pos>) -> u64 {
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

fn part2(stones: &Vec<Vec<char>>) -> u64 {
    todo!()
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
    test1(TEST_INPUT, 1930, input::as_vecvec, part1);
}
