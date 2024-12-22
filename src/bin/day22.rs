use std::collections::HashMap;

use advent_of_code_24::input;
use itertools::multizip;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    advent_of_code_24::solve("day22", input::single_line_vec, part1, part2);
}

type N = u64;
type S = Vec<N>;

const MOD: N = 16777216;

struct Rand {
    secret: N,
}

impl Rand {
    pub fn new(secret: N) -> Self {
        Rand { secret }
    }
}

impl Iterator for Rand {
    type Item = N;

    fn next(&mut self) -> Option<N> {
        // We can try to optimize this using shift operations.
        // Also, since MOD = 0x1000000, we know the numbers will always
        // fit into u32.
        // However, in practice, none of this made real difference, so we
        // won't bother.
        self.secret ^= self.secret * 64;
        self.secret ^= self.secret % MOD / 32;
        self.secret ^= self.secret % MOD * 2048;
        self.secret %= MOD;
        Some(self.secret)
    }
}

struct DiffRand {
    rand: Rand,
    prev: u8,
}

impl DiffRand {
    pub fn new(secret: N) -> Self {
        DiffRand {
            rand: Rand::new(secret),
            prev: (secret % 10) as u8,
        }
    }
}

impl Iterator for DiffRand {
    type Item = (u8, i8);

    fn next(&mut self) -> Option<Self::Item> {
        let n = (self.rand.next().unwrap() % 10) as u8;
        let res = (n, n as i8 - self.prev as i8);
        self.prev = n;
        Some(res)
    }
}

fn part1(secrets: &S) -> N {
    secrets
        .into_iter()
        .map(|s| Rand::new(*s))
        .map(|r| r.skip(1999).next().unwrap())
        .sum()
}

fn part2(secrets: &S) -> N {
    secrets
        // Each secret can be handled independently; so we parallelize to make it go brrrr.
        .into_par_iter()
        .map(|s| DiffRand::new(*s).take(2000))
        .map(|seq| {
            // We want to run four iterators over the numbers.
            // It's just 2000, so we can easily fit this into memory.
            let seq: Vec<(u8, i8)> = seq.collect();
            // Will hold the achievable price for every 4-sequence.
            let mut prices = HashMap::new();
            for ((_, d0), (_, d1), (_, d2), (p, d3)) in multizip((
                seq.iter(),
                seq.iter().skip(1),
                seq.iter().skip(2),
                seq.iter().skip(3),
            )) {
                // Only insert a price if it doesn't already exist.
                prices.entry((*d0, *d1, *d2, *d3)).or_insert(*p as N);
            }
            prices
        })
        .reduce(HashMap::new, |mut acc, hash| {
            // Merge all hashmaps by summing their elements.
            for (seq, price) in hash {
                if acc.contains_key(&seq) {
                    acc.insert(seq, acc[&seq] + price);
                } else {
                    acc.insert(seq, price);
                }
            }
            acc
        })
        // The prices are stored in the values. We only care about the max price.
        .into_values()
        .max()
        .unwrap()
}

#[allow(dead_code)]
const TEST_OUTPUT1: &str = "15887950
16495136
527345
704524
1553684
12683156
11100544
12249484
7753432
5908254";

#[test]
fn test_rand() {
    assert_eq!(
        input::single_line_vec::<N>(TEST_OUTPUT1),
        Vec::from_iter(Rand::new(123).take(10))
    );
}

#[allow(dead_code)]
const TEST_INPUT1: &str = "1
10
100
2024";

#[test]
fn test_part1() {
    advent_of_code_24::test1(TEST_INPUT1, 37327623, input::single_line_vec, part1)
}

#[allow(dead_code)]
const TEST_INPUT2: &str = "1
2
3
2024";

#[test]
fn test_part2() {
    advent_of_code_24::test1(TEST_INPUT2, 23, input::single_line_vec, part2)
}
