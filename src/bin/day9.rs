use std::{borrow::BorrowMut, cmp::min};

use advent_of_code_24::solve;

#[allow(dead_code)]
const TEST_INPUT: &str = "2333133121414131402";

fn main() {
    solve("day9", parse, part1, part2);
}

#[derive(Clone, Debug, Copy)]
struct File {
    pos: u64,
    size: u64,
    id: u64,
}

fn parse(disk_map: &str) -> Vec<File> {
    let mut file_id = 0;
    let mut entry_pos = 0;
    disk_map
        .char_indices()
        .flat_map(|(i, e)| {
            let size = e.to_digit(10).unwrap() as u64;
            if i % 2 == 0 {
                let e = File {
                    pos: entry_pos,
                    size,
                    id: file_id,
                };
                entry_pos += size;
                file_id += 1;
                Some(e)
            } else {
                entry_pos += size;
                None
            }
        })
        .collect()
}

fn part1(disk_map: &Vec<File>) -> u64 {
    let mut fixed: Vec<File> = disk_map.to_vec();
    let mut must_write = None;
    while let Some((i, free_pos, free_until)) = find_gap(&fixed, 1) {
        if must_write.is_none() {
            must_write = fixed.pop();
        }
        let mut f = must_write.unwrap();
        let can_write = free_until - free_pos;
        let will_write = min(can_write, f.size);
        fixed.insert(
            i,
            File {
                pos: free_pos,
                size: will_write,
                id: f.id,
            },
        );
        f.size -= will_write;
        if f.size > 0 {
            must_write = Some(f);
        } else {
            must_write = None;
        }
    }
    // End leftovers at the end.
    if let Some(f) = must_write {
        let last = fixed.last().unwrap();
        fixed.push(File {
            pos: last.pos + last.size,
            id: f.id,
            size: f.size,
        })
    }
    checksum(&fixed)
}

fn part2(disk_map: &Vec<File>) -> u64 {
    let mut fixed: Vec<File> = disk_map.to_vec();
    let mut gaps = find_gaps(disk_map);

    for k in (0..fixed.len()).rev() {
        let f = fixed[k].borrow_mut();
        for i in 0..gaps.len() {
            let g = gaps[i].borrow_mut();
            if g.1 >= f.size && g.0 < f.pos {
                f.pos = g.0;
                g.0 += f.size;
                g.1 -= f.size;
                break;
            }
        }
    }

    fixed.sort_by_key(|f| f.pos);

    checksum(&fixed)
}

#[allow(dead_code)]
fn display_disk_map(disk_map: &Vec<File>) -> String {
    let mut disk_map = disk_map.to_vec();
    disk_map.sort_by_key(|f| f.pos);
    let mut s = String::new();
    let mut latest_pos = 0;
    for f in disk_map {
        for _ in 0..(f.pos - latest_pos) {
            s += ".";
        }
        for _ in 0..f.size {
            s += &(f.id % 10).to_string();
        }
        latest_pos = f.pos + f.size;
    }
    s
}

fn find_gap(disk_map: &Vec<File>, size: u64) -> Option<(usize, u64, u64)> {
    let mut last_end = 0;
    for (i, f) in disk_map.iter().enumerate() {
        if last_end + size < f.pos {
            return Some((i, last_end + 1, f.pos));
        }
        last_end = f.pos + f.size - 1;
    }
    None
}

fn find_gaps(disk_map: &Vec<File>) -> Vec<(u64, u64)> {
    let mut gaps = Vec::new();
    let mut last_end = 0;
    for f in disk_map.iter() {
        let gap = f.pos - last_end;
        if gap > 1 {
            gaps.push((last_end + 1, gap - 1));
        }
        last_end = f.pos + f.size - 1;
    }
    gaps
}

fn checksum(disk_map: &Vec<File>) -> u64 {
    disk_map
        .iter()
        .map(|f| {
            // (pos + pos+1 + ... + pos+size-1) * id
            if f.pos == 0 {
                return 0;
            }
            if f.pos == 1 {
                return ((f.size - 1) * f.size) / 2 * f.id;
            }
            ((f.pos + f.size - 1) * (f.pos + f.size) - (f.pos - 1) * f.pos) / 2 * f.id
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&parse(TEST_INPUT)), 1928);
}
#[test]
fn test_part2() {
    assert_eq!(part2(&parse(TEST_INPUT)), 2858);
}
