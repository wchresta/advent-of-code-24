use advent_of_code_24::input;

fn main() {
    println!("Hello, world!");
    let content: Vec<[i64;2]> = input::read_i64("day1");

    for [left, right] in content {
        println!("left: {}, right: {}", left, right);
    }
}
