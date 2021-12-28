use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete as character;
use nom::combinator::{map, opt};
use nom::multi::{count, separated_list1};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;
use std::collections::{BTreeSet, VecDeque};
use std::ops::Neg;

pub fn solve() -> (usize, i32) {
    let scanners = parse_puzzle(include_str!("../resources/day19.txt"));
    let table = create_trans_table(&scanners);
    (solve1(&table, &scanners), solve2(&table, &scanners))
}

fn solve1(table: &[(usize, Point, M3)], scanners: &[Scanner]) -> usize {
    let fixed_scanners = scanners.iter().cloned().map(|s| Scanner {
        id: s.id,
        points: relative_to_0(table, s.id as usize, s.points),
    });
    let points = fixed_scanners
        .flat_map(|s| s.points)
        .collect::<BTreeSet<_>>();
    points.len()
}

fn solve2(table: &[(usize, Point, M3)], scanners: &[Scanner]) -> i32 {
    let origs = (0..scanners.len())
        .into_iter()
        .map(|i| relative_to_0(table, i, vec![Point::default()])[0])
        .collect::<Vec<_>>();

    origs
        .iter()
        .cartesian_product(&origs)
        .map(|(a, b)| dist(a, b))
        .max()
        .unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
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

#[derive(Debug, Clone)]
struct Scanner {
    id: u8,
    points: Vec<Point>,
}

type M3 = [[i32; 3]; 3];

fn m3_mul(a: M3, b: M3) -> M3 {
    [
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0] + a[0][2] * b[2][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1] + a[0][2] * b[2][1],
            a[0][0] * b[0][2] + a[0][1] * b[1][2] + a[0][2] * b[2][2],
        ],
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0] + a[1][2] * b[2][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1] + a[1][2] * b[2][1],
            a[1][0] * b[0][2] + a[1][1] * b[1][2] + a[1][2] * b[2][2],
        ],
        [
            a[2][0] * b[0][0] + a[2][1] * b[1][0] + a[2][2] * b[2][0],
            a[2][0] * b[0][1] + a[2][1] * b[1][1] + a[2][2] * b[2][1],
            a[2][0] * b[0][2] + a[2][1] * b[1][2] + a[2][2] * b[2][2],
        ],
    ]
}

fn scanner_permutations(scanner: &Scanner) -> [(M3, Vec<Point>); 24] {
    let rots: [M3; 8] = [
        [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
        [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
        [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
        [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
        [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
        [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
        [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
    ];

    let perms = [
        [[1, 0, 0], [0, 1, 0], [0, 0, 1]],  // x, y, z
        [[0, 1, 0], [0, 0, 1], [1, 0, 0]],  // y, z, x
        [[0, 0, 1], [0, 1, 0], [-1, 0, 0]], // z, y, -x
    ];

    rots.into_iter()
        .cartesian_product(perms)
        .map(|(a, b)| {
            let m = m3_mul(a, b);
            (
                m,
                scanner.points.clone().into_iter().map(|p| p * &m).collect(),
            )
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn overlap(source_points: &[Point], scanner_points: &[Point]) -> Option<Point> {
    for source_point in source_points {
        for scanner_point in scanner_points {
            let delta = source_point - scanner_point;
            let matching_count = scanner_points
                .iter()
                .filter(|scanner_point| {
                    source_points
                        .iter()
                        .any(|source_point| source_point - *scanner_point == delta)
                })
                .count();

            if matching_count >= 12 {
                return Some(delta);
            }
        }
    }
    None
}

fn create_trans_table(scanners: &[Scanner]) -> Vec<(usize, Point, M3)> {
    let n = scanners.len();
    let id_matrix = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
    let mut table = vec![(0, Point::default(), id_matrix); n];
    let mut seen = vec![false; n];
    let mut to_check = VecDeque::from([0]);

    while let Some(source) = to_check.pop_front() {
        if seen[source] {
            continue;
        }

        seen[source] = true;
        for i in 0..n {
            if seen[i] {
                continue;
            }
            for perm in scanner_permutations(&scanners[source]) {
                if let Some(delta) = overlap(&scanners[i].points, &perm.1) {
                    to_check.push_back(i);
                    table[i] = (source, delta, perm.0);
                    break;
                }
            }
        }
    }

    table
}

// inverse simple rotation matrix
fn inv(m: M3) -> M3 {
    [
        [m[0][0], m[1][0], m[2][0]],
        [m[0][1], m[1][1], m[2][1]],
        [m[0][2], m[1][2], m[2][2]],
    ]
}

fn relative_to_0(
    table: &[(usize, Point, M3)],
    source_idx: usize,
    points: Vec<Point>,
) -> Vec<Point> {
    if source_idx == 0 {
        return points;
    }

    let (next_idx, delta, m) = table[source_idx];
    let inv_m = inv(m);
    let points = points
        .into_iter()
        .map(|p| (p - delta) * &inv_m)
        .collect::<Vec<_>>();
    relative_to_0(table, next_idx, points)
}

fn dist(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
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
    fn test_solve1() {
        let puzzle = parse_puzzle(TEST_INPUT);
        let table = create_trans_table(&puzzle);
        assert_eq!(79, solve1(&table, &puzzle));
    }

    #[test]
    fn test_solve2() {
        let puzzle = parse_puzzle(TEST_INPUT);
        let table = create_trans_table(&puzzle);
        assert_eq!(3621, solve2(&table, &puzzle));
    }
}
