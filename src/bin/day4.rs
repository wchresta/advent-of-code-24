use std::cmp::min;

use advent_of_code_24::input;


fn main() {
    let content: M = input::as_matrix(&input::read("day4"));
    println!("part1 {}", part1(&content));
    println!("part2 {}", part2(&content));
}

type M = Vec<Vec<char>>;

fn part1(content: &M) -> usize {
    let block_sums: [usize;8] = [ content
    , &transpose(content)
    , &reverse(content)
    , &transpose(&flip(content))
    , &diagonals(content)
    , &diagonals(&flip(content))
    , &diagonals(&reverse(content))
    , &diagonals(&reverse(&flip(content)))
    ].map(|m| m.iter().map(find_xmas).sum());
    block_sums.iter().sum()
}

fn find_xmas(chars: &Vec<char>) -> usize {
    chars.into_iter().collect::<String>().match_indices("XMAS").count()
}

fn part2(mat: &M) -> isize {
    let m = mat.len();
    let n = mat[0].len();

    let mut count = 0;
    for i in 1..m-1 {
        for j in 1..n-1 {
            if mat[i][j] != 'A' {
                continue
            }
            let mut xmasses = 0;
            if mat[i-1][j-1] == 'M' && mat[i+1][j+1] == 'S' {
                xmasses += 1
            }    
            if mat[i-1][j+1] == 'M' && mat[i+1][j-1] == 'S' {
                xmasses += 1
            }    
            if mat[i+1][j-1] == 'M' && mat[i-1][j+1] == 'S' {
                xmasses += 1
            }    
            if mat[i+1][j+1] == 'M' && mat[i-1][j-1] == 'S' {
                xmasses += 1
            }    
            if xmasses >= 2 {
                count += 1
            }
        }
    }
    count
}

fn transpose(content: &M) -> M {
    let mut vec: M  = content[0].iter().map(|_| Vec::new()).collect();
    for line in content {
        for (i, char) in line.iter().enumerate() {
            vec[i].push(*char);
        }
    }
    vec
}

fn flip(content: &M) -> M {
    content.iter().rev().map(|s| s.clone()).collect()
}

fn reverse(content: &M) -> M {
    content.iter().map(|line| line.iter().rev().collect::<String>()).map(|s| s.chars().collect()).collect()
}

fn diagonals(content: &M) -> M {
    let mut vec: M  = Vec::new();
    let m: isize = content.len().try_into().unwrap();
    let n: isize = content[0].len().try_into().unwrap();
    // lower diagonals
    for d in 1..m {
        let mut diag = Vec::new();
        for i in 0..min(m-d,n) {
            diag.push(content[(i+d) as usize][i as usize]);
        }
        vec.push(diag);
    }
    // upper diagonals
    for d in 0..n {
        let mut diag = Vec::new();
        for i in 0..min(m,n-d) {
            diag.push(content[i as usize][(i+d) as usize]);
        }
        vec.push(diag);
    }
    vec
}

#[test]
fn test_diagonals() {
    let mat: M = Vec::from([
        "abc",
        "def",
        "ghi",
        "jkl"].map(|l| l.chars().collect()));
    let want: M = Vec::from(["dhl","gk","j","aei","bf","c"].map(|l| l.chars().collect()));
    assert_eq!(diagonals(&mat), want);

    let mat: M = Vec::from([
    "abcd",
    "efgh",
    "ijkl",
    ].map(|l| l.chars().collect()));
    let want: M = Vec::from(["ej","i","afk","bgl","ch","d"].map(|l| l.chars().collect()));
    assert_eq!(diagonals(&mat), want);
}


#[test]
fn test_part1() {
    let content = input::as_matrix("....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX");
    assert_eq!(part1(&content), 18);

    let content = input::as_matrix("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX");
    assert_eq!(part1(&content), 18);
}

#[test]
fn test_part2() {
    let content = input::as_matrix(
".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........");
    assert_eq!(part2(&content), 9);

    let content = input::as_matrix(
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX");
    assert_eq!(part2(&content), 9);
}
