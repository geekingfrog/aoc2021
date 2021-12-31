use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete as character;
use nom::combinator::{all_consuming, map, opt};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

use crate::utils::parse_signed;

pub fn solve() -> (isize, isize) {
    let prog = parse_prog(include_str!("../resources/day24.txt"));
    let constraints = get_constraints(&prog);
    (
        fold_digits(&solve1(&constraints)),
        fold_digits(&solve2(&constraints)),
    )
}

fn solve1(constraints: &[Constraint]) -> [i8; 14] {
    let mut digits = [0; 14];
    for (a, b, c) in constraints {
        if c > &0 {
            digits[*a] = 9;
            digits[*b] = 9 - c;
        } else {
            digits[*a] = 9 + c;
            digits[*b] = 9;
        }
    }
    digits
}

fn solve2(constraints: &[Constraint]) -> [i8; 14] {
    let mut digits = [0; 14];
    for (a, b, c) in constraints {
        if c > &0 {
            digits[*a] = 1 + c;
            digits[*b] = 1;
        } else {
            digits[*a] = 1;
            digits[*b] = 1 - c;
        }
    }
    digits
}

/// (a, b, c) is read as input[a] = input[b] + c
type Constraint = (usize, usize, i8);

fn get_constraints(prog: &[Ins]) -> Vec<Constraint> {
    // grab some magic values
    let xs = (0..14)
        .into_iter()
        .map(|i| get_ns(&prog[(i * 18 + 5) as usize]))
        .collect::<Vec<_>>();
    let ys = (0..14)
        .into_iter()
        .map(|i| get_ns(&prog[(i * 18 + 15) as usize]))
        .collect::<Vec<_>>();
    let zs = (0..14)
        .into_iter()
        .map(|i| get_ns(&prog[(i * 18 + 4) as usize]));

    let mut result = vec![];
    let mut stack = vec![];
    for (idx, z) in zs.enumerate() {
        match z {
            1 => stack.push(idx),
            26 => {
                let prev_idx = stack.pop().unwrap();
                let cst = ys[prev_idx] + xs[idx];
                let constraint = (idx, prev_idx, cst);
                result.push(constraint);
            }
            _ => unreachable!("invalid div z ? {}", z),
        }
    }

    result
}

fn fold_digits(ds: &[i8]) -> isize {
    ds.iter().fold(0, |acc, d| acc * 10 + (*d as isize))
}

// grab hardcoded value in the bytecode
fn get_ns(ins: &Ins) -> i8 {
    match ins {
        Ins::Add(_, Var::N(n)) => *n,
        Ins::Div(_, Var::N(n)) => *n,
        Ins::Mul(_, Var::N(n)) => *n,
        Ins::Mod(_, Var::N(n)) => *n,
        Ins::Eql(_, Var::N(n)) => *n,
        _ => unreachable!("oops {:?}", ins),
    }
}

#[derive(Debug)]
enum Reg {
    W,
    X,
    Y,
    Z,
}

#[derive(Debug)]
enum Var {
    R(Reg),
    N(i8),
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

    alt((map(parse_reg, R), map(parse_signed, N)))(raw)
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Default)]
    struct Alu {
        regs: [isize; 4],
    }

    impl Alu {
        fn run_prog(&mut self, inputs: &[i8; 14], instructions: &[Ins]) {
            let mut input_idx = 0;
            for (i, ins) in instructions.iter().enumerate() {
                println!("{:03} - {:?} - {:?}", i + 1, self, ins);
                match ins {
                    Ins::Inp(r) => {
                        self.regs[r.to_idx()] = inputs[input_idx] as isize;
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
                Var::N(n) => *n as isize,
            }
        }
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

    const TEST_INPUT: &str = include_str!("../resources/day24.txt");

    #[test]
    fn test_parse() {
        let prog = parse_prog(TEST_INPUT);
        assert_eq!(252, prog.len());
    }

    #[test]
    fn test_solve1() {
        let prog = parse_prog(TEST_INPUT);
        let inputs = solve1(&get_constraints(&prog));
        let mut alu = Alu::default();
        alu.run_prog(&inputs, &prog);
        assert_eq!(alu.regs[3], 0);
    }

    #[test]
    fn test_solve2() {
        let prog = parse_prog(TEST_INPUT);
        let inputs = solve2(&get_constraints(&prog));
        let mut alu = Alu::default();
        alu.run_prog(&inputs, &prog);
        assert_eq!(alu.regs[3], 0);
    }

}
