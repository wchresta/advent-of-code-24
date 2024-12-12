pub mod input;
pub mod square;

#[allow(dead_code)]
pub fn solve1<T, U>(day: &str, input_func: impl Fn(&str) -> T, part1: impl Fn(&T) -> U)
where
    U: core::fmt::Display,
{
    let t: T = input_func(&input::read(day));
    println!("part1: {}", part1(&t));
}

#[allow(dead_code)]
pub fn test1<T, U>(inp: &str, want: U, input_func: impl Fn(&str) -> T, part1: impl Fn(&T) -> U)
where
    U: core::fmt::Display + core::fmt::Debug + Eq,
{
    let t: T = input_func(inp);
    assert_eq!(part1(&t), want);
}

#[allow(dead_code)]
pub fn solve<T, U>(
    day: &str,
    input_func: impl Fn(&str) -> T,
    part1: impl Fn(&T) -> U,
    part2: impl Fn(&T) -> U,
) where
    U: core::fmt::Display,
{
    let t: T = input_func(&input::read(day));
    println!("part1: {}", part1(&t));
    println!("part2: {}", part2(&t));
}

pub fn digits(num: u64) -> u32 {
    num.checked_ilog10().unwrap_or(0) + 1
}

pub fn split_digits(num: u64) -> (u64, u64) {
    let new_len = digits(num) / 2;
    (
        (num as i64 / 10_i64.pow(new_len)) as u64,
        num % 10_u64.pow(new_len),
    )
}
