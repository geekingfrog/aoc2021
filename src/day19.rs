#![allow(dead_code, unused_variables)]
use nom::bytes::complete::tag;
use nom::character::complete as character;
use nom::combinator::{map, opt};
use nom::multi::{count, separated_list1};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;
use std::ops::Neg;

pub fn solve() -> (usize, usize) {
    let scanners = parse_puzzle(include_str!("../resources/day19.txt"));
    (solve1(&scanners), solve2(&scanners))
}

fn solve1(scanners: &[Scanner]) -> usize {
    0
}

fn solve2(scanners: &[Scanner]) -> usize {
    0
}

// fn overlap(s0: &Scanner, s1: &Scanner) -> Vec<Point> {
//     let mut result = vec![];
//     for p in &s0.points {
//         let delta = *p - s1.points[0];
//         let c = s1.points.iter().skip(1).filter(|&p2| {
//             (p - p2).is_zero()
//         }).count();
//         if c >= 11 {
//             result.push(*p);
//         }
//     }
//     result
// }

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0 && self.z == 0
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Sub<&Point> for Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        self - *rhs
    }
}

impl std::ops::Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        self.x.fmt(f)?;
        f.write_str(",")?;
        self.y.fmt(f)?;
        f.write_str(",")?;
        self.z.fmt(f)?;
        f.write_str(")")?;
        Ok(())
    }
}

#[derive(Debug)]
struct Scanner {
    id: u8,
    points: Vec<Point>,
}

fn parse_puzzle(raw: &str) -> Vec<Scanner> {
    separated_list1(count(character::line_ending, 2), scanner_parser)(raw)
        .unwrap()
        .1
}

fn scanner_parser(raw: &str) -> IResult<&str, Scanner> {
    map(
        tuple((
            scanner_id_parser,
            separated_list1(character::line_ending, point_parser),
        )),
        |(id, points)| Scanner { id, points },
    )(raw)
}

fn scanner_id_parser(raw: &str) -> IResult<&str, u8> {
    map(
        terminated(
            delimited(tag("--- scanner "), character::digit1, tag(" ---")),
            character::line_ending,
        ),
        |ds: &str| ds.parse().unwrap(),
    )(raw)
}

fn point_parser(raw: &str) -> IResult<&str, Point> {
    map(
        tuple((
            parse_i32,
            preceded(character::char(','), parse_i32),
            preceded(character::char(','), parse_i32),
        )),
        |(x, y, z)| Point { x, y, z },
    )(raw)
}

fn parse_i32(raw: &str) -> IResult<&str, i32> {
    map(
        tuple((opt(character::char('-')), character::digit1)),
        |(sign, ds): (Option<char>, &str)| {
            let x: i32 = ds.parse().unwrap();
            match sign {
                None => x,
                Some(_) => x.neg(),
            }
        },
    )(raw)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../resources/day19_test.txt");

    #[test]
    fn test_parser() {
        let puzzle = parse_puzzle(TEST_INPUT);
        println!("{:?}", puzzle);
        assert_eq!(5, puzzle.len());
        assert_eq!(0, puzzle[0].id);
        assert_eq!(25, puzzle[0].points.len());
    }

    // #[test]
    // #[ignore]
    // fn test_solve1() {
    //     assert_eq!(123, solve1());
    // }
    //
    // #[test]
    // #[ignore]
    // fn test_solve2() {
    //     assert_eq!(123, solve2());
    // }
}
