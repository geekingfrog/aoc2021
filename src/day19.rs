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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Mul<&[[i32; 3]; 3]> for Point {
    type Output = Self;

    fn mul(self, rhs: &[[i32; 3]; 3]) -> Self::Output {
        Point {
            x: self.x * rhs[0][0] + self.y * rhs[0][1] + self.z * rhs[0][2],
            y: self.x * rhs[1][0] + self.y * rhs[1][1] + self.z * rhs[1][2],
            z: self.x * rhs[2][0] + self.y * rhs[2][1] + self.z * rhs[2][2],
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

type M3 = [[i32; 3]; 3];

// const MATRICES: [M3; 24] = [
//     // x "up"
//     [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
//     [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
//     [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
//     [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
//     [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
//     [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
//     [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
//     [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
//     // y "up"
//     // [[0, 1, 0], [1, 0, 0], [0, 0, 1]],
//     [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
//     [[0, 0, -1], [1, 0, 0], [0, 1, 0]],
//     [[0, -1, 0], [1, 0, 0], [0, 0, -1]],
//     [[0, 0, 1], [1, 0, 0], [0, -1, 0]],
//     [[0, 1, 0], [-1, 0, 0], [0, 0, -1]],
//     [[0, 0, -1], [-1, 0, 0], [0, -1, 0]],
//     [[0, -1, 0], [-1, 0, 0], [0, 0, 1]],
//     [[0, 0, 1], [-1, 0, 0], [0, 1, 0]],
//     // z "up"
//     [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
//     [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
//     [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
//     [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
//     [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
//     [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
//     [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
//     [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
// ];

const MATRICES: [M3; 24] = [
    // x up
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
    [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
    [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
    [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
    // x down
    [[-1, 0, 0], [0, 1, 0], [0, 0, 1]],
    [[-1, 0, 0], [0, 0, -1], [0, 1, 0]],
    [[-1, 0, 0], [0, -1, 0], [0, 0, -1]],
    [[-1, 0, 0], [0, 0, 1], [0, -1, 0]],
    // y up
    [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
    [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
    [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
    [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
    // y down
    [[0, 0, 1], [-1, 0, 0], [0, 1, 0]],
    [[0, 1, 0], [-1, 0, 0], [0, 0, -1]],
    [[0, 0, -1], [-1, 0, 0], [0, -1, 0]],
    [[0, -1, 0], [-1, 0, 0], [0, 0, 1]],
    // z up
    [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
    [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
    [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
    [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
    // z down
    [[0, 1, 0], [0, 0, 1], [-1, 0, 0]],
    [[0, 0, -1], [0, 1, 0], [-1, 0, 0]],
    [[0, -1, 0], [0, 0, -1], [-1, 0, 0]],
    [[0, 0, 1], [0, -1, 0], [-1, 0, 0]],
];

fn scanner_permutations(scanner: &Scanner) -> [(M3, Vec<Point>); 24] {
    MATRICES
        .iter()
        .map(|m| {
            (
                *m,
                scanner.points.clone().into_iter().map(|p| p * m).collect(),
            )
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn overlap(source_points: &[Point], scanner: &(M3, Vec<Point>)) -> Option<(Point, M3, Vec<Point>)> {
    let (scanner_perm, scanner_points) = scanner;
    for source_point in source_points {
        for scanner_point in scanner_points {
            let delta = source_point - scanner_point;
            let matching_points: Vec<_> = scanner_points
                .iter()
                .cloned()
                .filter(|scanner_point| {
                    source_points
                        .iter()
                        .any(|source_point| *source_point - scanner_point == delta)
                })
                .collect();

            if matching_points.len() >= 12 {
                return Some((delta, *scanner_perm, matching_points));
            }
        }
    }
    None
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

    // const TEST_INPUT: &str = include_str!("../resources/day19_test.txt");
    const TEST_INPUT: &str = include_str!("../resources/day19.txt");

    #[test]
    #[ignore]
    fn test_parser() {
        let puzzle = parse_puzzle(TEST_INPUT);
        println!("{:?}", puzzle);
        assert_eq!(5, puzzle.len());
        assert_eq!(0, puzzle[0].id);
        assert_eq!(25, puzzle[0].points.len());
    }

    #[test]
    fn test_matrix_mul_id() {
        assert_eq!(
            Point {
                x: 10,
                y: 20,
                z: 30
            },
            Point {
                x: 10,
                y: 20,
                z: 30
            } * &[[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        );
    }

    #[test]
    fn test_matrix_mul_0() {
        assert_eq!(
            Point {
                x: -20,
                y: 10,
                z: -30
            },
            Point {
                x: 10,
                y: 20,
                z: 30
            } * &[[0, -1, 0], [1, 0, 0], [0, 0, -1]],
        );
    }

    #[test]
    fn test_delta_1() {
        let puzzle = parse_puzzle(TEST_INPUT);
        let matched = scanner_permutations(&puzzle[1])
            .iter()
            .find_map(|perm| overlap(&puzzle[0].points, perm));
        let expected_delta = Point {
            x: 68,
            y: -1246,
            z: -43,
        };
        let (delta, perm, _) = matched.unwrap();
        println!("delta: {}, perm: {:?}", delta, perm);
        assert_eq!(expected_delta, delta);
    }

    #[test]
    fn test_delta_2() {
        let puzzle = parse_puzzle(TEST_INPUT);
        // let matched = [0, 1, 3, 4].into_iter().find_map(|i| {
        //     println!("matching 2 with {}", i);
        //     scanner_permutations(&puzzle[2])
        //         .iter()
        //         .find_map(|perm| overlap(&puzzle[i].points, perm))
        // });

        let matched = scanner_permutations(&puzzle[2])
            .iter()
            .find_map(|perm| overlap(&puzzle[4].points, perm));

        let expected_delta = Point {
            x: -20,
            y: -1133,
            z: -1061,
        };
        let delta_1 = Point {
            x: 68,
            y: -1246,
            z: -43,
        };
        // scanner 1 matrix:
        // [[-1, 0, 0], [0, 1, 0], [0, 0, -1]]

        // scanner 4 matrix (relative to 1):
        // [[0, 1, 0], [0, 0, -1], [-1, 0, 0]]
        let (delta, perm, _) = matched.unwrap();
        println!("delta: {}, perm: {:?}", delta, perm);
        assert_eq!(expected_delta, delta - delta_1);
    }

    #[test]
    fn test_delta_3() {
        let puzzle = parse_puzzle(TEST_INPUT);
        let matched = scanner_permutations(&puzzle[3])
            .iter()
            .find_map(|perm| overlap(&puzzle[1].points, perm));
        let expected_delta = Point {
            x: -20,
            y: -1133,
            z: -1061,
        };
        let delta_1 = Point {
            x: 68,
            y: -1246,
            z: -43,
        };
        // scanner 1 matrix:
        // [[-1, 0, 0], [0, 1, 0], [0, 0, -1]]

        // scanner 4 matrix (relative to 1):
        // [[0, 1, 0], [0, 0, -1], [-1, 0, 0]]
        let (delta, perm, _) = matched.unwrap();
        println!("delta: {}, perm: {:?}", delta, perm);
        assert_eq!(expected_delta, delta - delta_1);
    }

    #[test]
    fn test_delta_4() {
        let puzzle = parse_puzzle(TEST_INPUT);
        let matched = [0, 1, 2, 3].into_iter().find_map(|i| {
            println!("matching 4 with {}", i);
            scanner_permutations(&puzzle[4])
                .iter()
                // 1
                .find_map(|perm| overlap(&puzzle[i].points, perm))
        });
        let expected_delta = Point {
            x: -20,
            y: -1133,
            z: -1061,
        };
        let delta_1 = Point {
            x: 68,
            y: -1246,
            z: -43,
        };
        // scanner 1 matrix:
        // [[-1, 0, 0], [0, 1, 0], [0, 0, -1]]

        // scanner 4 matrix (relative to 1):
        // [[0, 1, 0], [0, 0, -1], [-1, 0, 0]]
        let (delta, perm, _) = matched.unwrap();
        println!("delta: {}, perm: {:?}", delta, perm);
        assert_eq!(expected_delta, delta - delta_1);
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
