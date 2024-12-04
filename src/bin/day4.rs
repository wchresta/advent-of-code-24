use std::str::{Chars, Lines};

use advent_of_code_24::input;


fn main() {
    let content = input::read_all("day4");
    println!("part1 {}", part1(&content));
}

fn part1(content: &str) -> usize {
    [ content.lines()
    , transpose(&content).lines()
    ].map(|block| block.map(|line| find_xmas(line)).sum()).iter().sum()
}

fn transpose(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let line_len = lines[0].len();
    let line_num = lines.len();

    let mut new_lines: Vec<String> = Vec::new();
    for j in 0..line_len-1 {
        let mut new_line = String::with_capacity(line_num);
        for i in 0..line_num-1 {
            new_line.push(lines[i].as_bytes()[j].into());
        }
        new_lines.push(new_line);
    }
    new_lines.join("\n")
}

fn find_xmas(chars: &str) -> usize {
    chars.match_indices("xmas").count()
}

