use itertools::Itertools;
use regex::Regex;

fn main() {
    advent_of_code_24::solve("day17", parse, part1, part2);
}

#[derive(Debug, Clone)]
struct Computer {
    a: i64,
    b: i64,
    c: i64,
    ip: usize,
    instr: Vec<char>,
    out: Vec<i64>,
    done: bool,
}

impl Computer {
    fn run(&mut self) {
        while !self.done {
            //println!("{:?}", self);
            self.step();
        }
    }

    fn step(&mut self) {
        if self.ip >= self.instr.len() {
            self.done = true;
            return;
        }

        let instr = self.instr[self.ip];
        let arg = self.instr[self.ip + 1];

        match instr.into() {
            Instr::ADV => {
                self.a = self.a / (2i64.pow(self.combo(arg) as u32));
            },
            Instr::BXL => {
                self.b = self.b ^ self.literal(arg);
            },
            Instr::BST => {
                self.b = self.combo(arg) % 8;
            },
            Instr::JNZ => {
                if self.a != 0 { 
                    self.ip = self.literal(arg) as usize;
                    return // Do not increase IP
                }
            },
            Instr::BXC => {
                self.b = self.c ^ self.b;
            },
            Instr::OUT => {
                self.out.push(self.combo(arg) % 8);
            },
            Instr::BDV => {
                self.b = self.a / (2i64.pow(self.combo(arg) as u32));
            },
            Instr::CDV => {
                self.c = self.a / (2i64.pow(self.combo(arg) as u32));
            },
        }

        self.ip += 2;
    }

    fn literal(&self, c: char) -> i64 {
        c.to_digit(10).unwrap() as i64
    }

    fn combo(&self, c: char) -> i64 {
        match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => self.a,
            '5' => self.b,
            '6' => self.c,
            '7' => panic!("combo 7 invalid"),
            _ => panic!("combo invalid"),
        }
    }
}

impl From<&str> for Computer {
    fn from(value: &str) -> Self {
        let reg = Regex::new(r"(?s)Register A: (\d+)\s*
Register B: (\d+)\s*
Register C: (\d+)\s*

Program: ([\d,]+)").unwrap();
        
        let (_, [a,b,c,prog]) = reg.captures(value).unwrap().extract();
        Self {
            a: a.parse().unwrap(),
            b: b.parse().unwrap(),
            c: c.parse().unwrap(),
            ip: 0,
            instr: prog.split(",").map(|i| i.chars().next().unwrap().into()).collect(),
            out: Vec::new(),
            done: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl From<char> for Instr {
    fn from(value: char) -> Self {
        match value {
            '0' => Self::ADV,
            '1' => Self::BXL,
            '2' => Self::BST,
            '3' => Self::JNZ,
            '4' => Self::BXC,
            '5' => Self::OUT,
            '6' => Self::BDV,
            '7' => Self::CDV,
            _ => panic!("Do not know instruction"),
        }
    }
}

fn parse(s: &str) -> Computer {
    Computer::from(s)
}

fn part1(c: &Computer) -> String {
    let mut c = c.clone();
    c.run();
    c.out.iter().map(|i| i.to_string()).join(",")
}

fn part2(c: &Computer) -> String {
    String::new()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

#[test]
fn test_part1() {
    advent_of_code_24::test1(TEST_INPUT, String::from("4,6,3,5,6,3,5,2,1,0"), parse, part1);
}

#[test]
fn test_part2() {
    advent_of_code_24::test1(TEST_INPUT, String::from(""), parse, part2);
}
