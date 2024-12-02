use std::{format, fs, str::FromStr};

pub fn read_all(day: &str) -> String {
    let path = format!("inputs/{}", day);
    fs::read_to_string(&path).expect(&format!("input file {} not found", &path))
}

pub fn read_lines(day: &str) -> Vec<String> {
    read_all(day).lines().map(String::from).collect()
}

pub fn read_i64<T, const D: usize>(day: &str) -> Vec<[T;D]>
where T: FromStr + std::fmt::Debug, <T as FromStr>::Err: std::fmt::Debug {
    read_lines(day).iter().map(|line| 
        line.split_whitespace()
            .take(D)
            .map(|p| p.parse::<T>().expect(&format!("cannot read {} as number from {}", p, day)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    ).collect()
}
