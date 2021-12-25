use std::iter::Rev;
use std::ops::Range;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete as character;
use nom::combinator::{all_consuming, map, opt};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

use crate::utils::parse_i32;

pub fn solve() -> (usize, usize) {
    let prog = parse_prog(include_str!("../resources/day24.txt"));
    // println!("prog:\n{:?}", prog);
    (solve1(&prog), solve2())
}

fn solve1(prog: &[Ins]) -> usize {
    for i in ranges(14) {
        let mut alu = Alu::default();
        alu.run_prog(&i, prog);
        if alu.regs[3] == 0 {
            return i.iter().fold(0, |acc, d| acc * 10 + *d as usize);
        }
    }
    0
}

fn solve2() -> usize {
    0
}

#[derive(Debug, Default)]
struct Alu {
    regs: [isize; 4],
}

impl Alu {
    fn run_prog(&mut self, inputs: &[isize], instructions: &[Ins]) {
        let mut input_idx = 0;
        for ins in instructions {
            println!("{:?} - {:?}", self, ins);
            match ins {
                Ins::Inp(r) => {
                    self.regs[r.to_idx()] = inputs[input_idx];
                    input_idx += 1;
                }
                Ins::Add(r, v) => {
                    self.regs[r.to_idx()] += self.get(v);
                }
                Ins::Mul(r, v) => {
                    self.regs[r.to_idx()] *= self.get(v);
                }
                Ins::Div(r, v) => {
                    self.regs[r.to_idx()] /= self.get(v);
                }
                Ins::Mod(r, v) => {
                    self.regs[r.to_idx()] %= self.get(v);
                }
                Ins::Eql(r, v) => {
                    let i = r.to_idx();
                    if self.regs[i] == self.get(v) {
                        self.regs[i] = 1;
                    } else {
                        self.regs[i] = 0;
                    }
                }
            }
        }
    }

    fn get(&self, v: &Var) -> isize {
        match v {
            Var::R(r) => self.regs[r.to_idx()],
            Var::N(n) => *n,
        }
    }
}

#[derive(Debug)]
enum Reg {
    W,
    X,
    Y,
    Z,
}

impl Reg {
    fn to_idx(&self) -> usize {
        match self {
            Reg::W => 0,
            Reg::X => 1,
            Reg::Y => 2,
            Reg::Z => 3,
        }
    }
}

#[derive(Debug)]
enum Var {
    R(Reg),
    N(isize),
}

#[derive(Debug)]
enum Ins {
    Inp(Reg),
    Add(Reg, Var),
    Mul(Reg, Var),
    Div(Reg, Var),
    Mod(Reg, Var),
    Eql(Reg, Var),
}

fn parse_prog(raw: &str) -> Vec<Ins> {
    let r: IResult<&str, _> = all_consuming(terminated(
        separated_list1(character::line_ending, parse_ins),
        opt(character::line_ending),
    ))(raw);
    r.unwrap().1
}

fn parse_ins(raw: &str) -> IResult<&str, Ins> {
    use Ins::*;

    alt((
        map(preceded(tag("inp "), parse_reg), Inp),
        map(
            preceded(
                tag("add "),
                separated_pair(parse_reg, character::char(' '), parse_var),
            ),
            |(r, v)| Add(r, v),
        ),
        map(
            preceded(
                tag("mul "),
                separated_pair(parse_reg, character::char(' '), parse_var),
            ),
            |(r, v)| Mul(r, v),
        ),
        map(
            preceded(
                tag("div "),
                separated_pair(parse_reg, character::char(' '), parse_var),
            ),
            |(r, v)| Div(r, v),
        ),
        map(
            preceded(
                tag("mod "),
                separated_pair(parse_reg, character::char(' '), parse_var),
            ),
            |(r, v)| Mod(r, v),
        ),
        map(
            preceded(
                tag("eql "),
                separated_pair(parse_reg, character::char(' '), parse_var),
            ),
            |(r, v)| Eql(r, v),
        ),
    ))(raw)
}

fn parse_reg(raw: &str) -> IResult<&str, Reg> {
    use Reg::*;

    alt((
        map(character::char('w'), |_| W),
        map(character::char('x'), |_| X),
        map(character::char('y'), |_| Y),
        map(character::char('z'), |_| Z),
    ))(raw)
}

fn parse_var(raw: &str) -> IResult<&str, Var> {
    use Var::*;

    alt((map(parse_reg, R), map(parse_i32, |n| N(n as isize))))(raw)
}

struct NumGen {
    lo: isize,
    hi: isize,
    its: Vec<Rev<Range<isize>>>,
    ns: Vec<isize>,
}

fn ranges(n: usize) -> NumGen {
    let lo = 1;
    let hi = 10;
    let mut its = vec![(lo..hi).rev(); n];
    for i in its.iter_mut().take(n - 1) {
        i.next();
    }

    NumGen {
        lo,
        hi,
        its,
        ns: vec![hi - 1; n],
    }
}

impl Iterator for NumGen {
    type Item = Vec<isize>;

    fn next(&mut self) -> Option<Self::Item> {
        for i in (0..self.its.len()).into_iter().rev() {
            match self.its[i].next() {
                Some(d) => {
                    self.ns[i] = d;
                    return Some(self.ns.clone());
                }
                None => {
                    self.its[i] = (self.lo..self.hi).rev();
                    self.its[i].next();
                    self.ns[i] = self.hi - 1;
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "";

    // #[test]
    // fn test_stuff() {
    //     let vs = ranges(2).into_iter().collect::<Vec<_>>();
    //     let r: Vec<i32> = vec![];
    //     assert_eq!(vec![r], vs);
    // }

    #[test]
    fn test_parse() {
        let prog = parse_prog(include_str!("../resources/day24.txt"));
        assert_eq!(252, prog.len());

        let mut alu = Alu::default();
        alu.run_prog(&[9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9], &prog);
        println!("{:?}", alu);
        assert!(false);
    }

    #[test]
    fn test_solve1() {
        todo!()
    }

    #[test]
    fn test_solve2() {
        todo!()
    }
}
