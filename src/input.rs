use std::{format, fs, str::FromStr};

pub fn read(day: &str) -> String {
    let path = format!("inputs/{}", day);
    fs::read_to_string(&path).expect(&format!("input file {} not found", &path))
}

pub fn as_lines(content: &str) -> Vec<String> {
    content.lines().map(String::from).collect()
}

pub fn as_vs<T>(content: &str) -> Vec<Vec<T>>
where T: FromStr + std::fmt::Debug, <T as FromStr>::Err: std::fmt::Debug {
    as_lines(content).iter().map(|line| 
        line.split_whitespace()
            .map(|p| p.parse::<T>().expect(&format!("cannot read {} as number", p)))
            .collect::<Vec<_>>()
    ).collect()
}

pub fn as_as<T, const D: usize>(content: &str) -> Vec<[T;D]>
where T: FromStr + std::fmt::Debug, <T as FromStr>::Err: std::fmt::Debug {
    as_lines(content).iter().map(|line| 
        line.split_whitespace()
            .take(D)
            .map(|p| p.parse::<T>().expect(&format!("cannot read {} as number", p)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    ).collect()
}

pub fn as_matrix<'a, T>(content: &'a str) -> Vec<Vec<T>>
where Vec<T>: FromIterator<char> {
    content.lines().map(|line| line.chars().collect()).collect()
}