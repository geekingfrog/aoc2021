use std::collections::BTreeSet;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete as character;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::{multi::separated_list1, sequence::separated_pair, IResult};

pub fn solve() -> (usize, String) {
    let puzzle = parse_puzzle(include_str!("../resources/day13.txt"));
    (solve1(&puzzle), solve2(&puzzle))
}

fn solve1(puzzle: &Puzzle) -> usize {
    let grid = GridMap::from_puzzle(puzzle);
    grid.fold(&puzzle.folds[0]).count_points()
}

fn solve2(puzzle: &Puzzle) -> String {
    let final_grid = GridMap::from_puzzle(puzzle)
        .fold_all(&puzzle.folds);
    // the answer is "BCZRCEAB" but I can't be arsed to code an OCR
    format!("\n{}", final_grid)
}

type Point = (u32, u32);

struct GridMap {
    points: BTreeSet<Point>,
}

impl GridMap {
    fn from_puzzle(puzzle: &Puzzle) -> Self {
        Self {
            points: puzzle.points.iter().cloned().collect(),
        }
    }
}

impl std::fmt::Display for GridMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.points.iter().map(|p| p.0).max().unwrap();
        let height = self.points.iter().map(|p| p.1).max().unwrap();
        for y in 0..=height {
            for x in 0..=width {
                if self.points.contains(&(x, y)) {
                    f.write_str("#")?;
                } else {
                    f.write_str(".")?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

trait Aoc {
    fn fold(self, f: &Fold) -> Self;
    fn fold_all(self, f: &[Fold]) -> Self
    where
        Self: Sized,
    {
        f.iter().fold(self, |acc, f| acc.fold(f))
    }
    fn count_points(&self) -> usize;
}

impl Aoc for GridMap {
    fn fold(self, f: &Fold) -> Self {
        match f {
            Fold::X(n) => {
                let mut points = BTreeSet::new();
                for (x, y) in &self.points {
                    if x > n {
                        points.insert((2 * n - x, *y));
                    } else {
                        points.insert((*x, *y));
                    }
                }
                Self { points }
            }
            Fold::Y(n) => {
                let mut points = BTreeSet::new();
                for (x, y) in &self.points {
                    if y > n {
                        points.insert((*x, 2 * n - y));
                    } else {
                        points.insert((*x, *y));
                    }
                }
                Self { points }
            }
        }
    }

    fn count_points(&self) -> usize {
        self.points.len()
    }
}

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(u32),
    Y(u32),
}

#[derive(Debug, Clone)]
struct Puzzle {
    points: Vec<Point>,
    folds: Vec<Fold>,
}

fn parse_puzzle(raw: &str) -> Puzzle {
    let result = separated_pair(parse_points, character::char('\n'), parse_folds)(raw);
    let (points, folds) = result.unwrap().1;
    Puzzle { points, folds }
}

fn parse_points(raw: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(
        character::line_ending,
        separated_pair(character::u32, character::char(','), character::u32),
    )(raw)
}

fn parse_folds(raw: &str) -> IResult<&str, Vec<Fold>> {
    preceded(
        character::line_ending,
        separated_list1(
            character::line_ending,
            preceded(tag("fold along "), parse_fold),
        ),
    )(raw)
}

fn parse_fold(raw: &str) -> IResult<&str, Fold> {
    map(
        separated_pair(
            alt((character::char('x'), character::char('y'))),
            character::char('='),
            character::u32,
        ),
        |(c, n)| match c {
            'x' => Fold::X(n),
            'y' => Fold::Y(n),
            _ => unreachable!("Unknown char: {}", c),
        },
    )(raw)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

    #[test]
    fn test_parse() {
        let p = parse_puzzle(TEST_INPUT);
        assert_eq!(p.points.len(), 18);
        assert_eq!(p.points[0], (6, 10));
        assert_eq!(p.folds.len(), 2);
        assert!(matches!(p.folds[0], Fold::Y(7)));
    }

    #[test]
    fn test_solve1() {
        let p = parse_puzzle(TEST_INPUT);
        assert_eq!(17, solve1(&p));
    }

    #[test]
    fn test_solve2() {
        let p = parse_puzzle(TEST_INPUT);
        assert_eq!("16".to_string(), solve2(&p));
    }
}
