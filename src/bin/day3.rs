use advent_of_code_24::input;

use regex::Regex;

fn main() {
    let content = input::read("day3");

    println!("part1 {}", part1(&content));
    println!("part2 {}", part2(&content));
}

fn part1(content: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("cannot compile regex");
    re.captures_iter(content).map(|c| {
        let (_, [x,y]) = c.extract();
        x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap()
    }).sum()
}

#[test]
fn test_part1() {
    let content = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert!(part1(content) == 161);
}

fn part2(content: &str) -> i32 {
    let re = Regex::new(r"(?s)don't\(\).*?(do\(\)|$)").expect("cannot compile regex");
    let stripped = re.replace_all(content, "").to_owned();
    part1(&stripped)
}

#[test]
fn test_part2() {
    let content = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert!(part2(content) == 48);
}

#[test]
fn test_part2_newline() {
    let content = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+\nmul(32,64](mul(11,8)undo()?mul(8,5))";
    assert!(part2(content) == 48);
}
