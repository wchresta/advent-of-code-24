fn main() {
    advent_of_code_24::solve("day19", parse, part1, part2);
}

struct P {
    stripes: Vec<String>,
    towels: Vec<String>,
}

fn parse(s: &str) -> P {
    let mut l = s.lines().map(String::from);
    let mut stripes: Vec<String> = l.next().unwrap().split(", ").map(String::from).collect();
    let towels = l.skip(1).collect();
    // It's better for search algorithms to have chunky things first.
    stripes.sort_by_key(|s| s.len());
    stripes.reverse();
    P { stripes, towels }
}

fn part1(p: &P) -> i64 {
    p.towels
        .iter()
        .filter(|t| can_be_made(*t, &p.stripes))
        .count() as i64
}

fn can_be_made(towel: &str, ps: &Vec<String>) -> bool {
    let mut stack = Vec::from([towel]);
    while let Some(t) = stack.pop() {
        for p in ps {
            if !t.starts_with(p) {
                continue;
            }
            let left_piece = &t[p.len()..];
            if left_piece == "" {
                return true;
            }
            stack.push(left_piece);
        }
    }
    false
}

// We have singles for w,u,r,g
// That means only b (black) are non-singles.
// We can probably use b's to split our problems easily.
fn part2(p: &P) -> i64 {
    let mut total_score = 0;
    for towel in &p.towels {
        // Scores contains the ways of combinations
        // the last k characters of towel can be arranged.
        // So scores[0] is 1
        let mut scores = Vec::with_capacity(towel.len() + 1);
        scores.push(1);
        for k in 1..towel.len() + 1 {
            let end = &towel[towel.len() - k..];
            let mut k_score = 0;
            for s in &p.stripes {
                if end.starts_with(s) {
                    let rest_score = scores[k - s.len()];
                    k_score += rest_score;
                }
            }
            scores.push(k_score);
        }
        total_score += scores.last().unwrap();
    }
    total_score
}

#[allow(dead_code)]
const TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

#[test]
fn test_part1() {
    advent_of_code_24::test1(TEST_INPUT, 6, parse, part1);
}

#[test]
fn test_part2() {
    advent_of_code_24::test1(TEST_INPUT, 16, parse, part2);
}
