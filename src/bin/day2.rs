use advent_of_code_24::input;
use std::iter::zip;

fn main() {
    let content: Vec<Vec<i32>> = input::as_vs(&input::read("day2"));
    println!("part1: {}", part1(&content));
    println!("part2: {}", part2(&content));
}

fn is_safe(r: &Vec<i32>) -> bool {
    let monotone = r.iter().all(|x| *x > 0) || r.iter().all(|x| *x < 0);
    let bounded = r.iter().all(|x| x.abs() <= 3);
    monotone && bounded
}

fn part1(content: &Vec<Vec<i32>>) -> usize {
    let diffs: Vec<Vec<i32>> = content
        .iter()
        .map(|r: &Vec<i32>| {
            zip(r.iter(), r.iter().skip(1))
                .map(|(x, y)| y - x)
                .collect()
        })
        .collect();
    diffs.iter().filter(|r| is_safe(r)).count()
}

#[test]
fn test_part1() {
    let content = input::as_vs::<i32>(
        "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9",
    );
    assert!(part1(&content) == 2)
}

fn part2(content: &Vec<Vec<i32>>) -> usize {
    let mut count = 0;
    for report in content {
        for i in 0..report.len() {
            let cut_report = [&report[..i], &report[i + 1..]].concat();
            let diffs = zip(cut_report.iter(), cut_report.iter().skip(1))
                .map(|(x, y)| y - x)
                .collect();
            if is_safe(&diffs) {
                count += 1;
                break;
            }
        }
    }
    count
}

#[test]
fn test_part2() {
    let content = input::as_vs::<i32>(
        "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9",
    );
    assert!(part2(&content) == 4)
}
