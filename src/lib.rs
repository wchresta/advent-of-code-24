pub mod input;

#[allow(dead_code)]
pub fn solve1<T,U>(day: &str, input_func: impl Fn(&str) -> T, solve1: impl Fn(&T) -> U)
where U: core::fmt::Display {
    let t: T = input_func(&input::read(day));
    println!("part1: {}", solve1(&t));
}

#[allow(dead_code)]
pub fn solve<T,U>(day: &str, input_func: impl Fn(&str) -> T, solve1: impl Fn(&T) -> U, solve2: impl Fn(&T) -> U) 
where U: core::fmt::Display {
    let t: T = input_func(&input::read(day));
    println!("part1: {}", solve1(&t));
    println!("part2: {}", solve2(&t));
}
