fn main() {
    advent_of_code_24::solve("day17", parse, part1, part2);
}

type S = Vec<char>;

fn parse(s: &str) -> S {
    todo!()
}

fn part1(s: &S) -> i64 {
    0
}

fn part2(s: &S) -> i64 {
    0
}

#[allow(dead_code)]
const TEST_INPUT: &str = "";

#[test]
fn test_part1() {
    advent_of_code_24::test1(TEST_INPUT, 0, parse, part1);
}

#[test]
fn test_part2() {
    advent_of_code_24::test1(TEST_INPUT, 0, parse, part2);
}