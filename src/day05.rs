use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::all_consuming;
use nom::combinator::{map, opt};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated};
use std::cmp;
use std::collections::BTreeMap;

pub fn solve() -> (usize, usize) {
    let puzzle = parse_puzzle(include_str!("../resources/day05.txt"));
    (solve1(&puzzle), solve2(&puzzle))
}

fn solve1(puzzle: &[(Point, Point)]) -> usize {
    // let mut grid = HashmapGrid::default();
    let mut grid = VecGrid::from_lines(puzzle);
    for (from, to) in puzzle {
        if (from.x == to.x) || (from.y == to.y) {
            grid.set_line(from, to);
        }
    }
    grid.count_intersections()
}

fn solve2(puzzle: &[(Point, Point)]) -> usize {
    // let mut grid = HashmapGrid::default();
    let mut grid = VecGrid::from_lines(puzzle);
    for (from, to) in puzzle {
        grid.set_line(from, to);
    }
    grid.count_intersections()
}

fn parse_puzzle(input: &str) -> Vec<(Point, Point)> {
    let result: nom::IResult<&str, _> = all_consuming(terminated(
        separated_list1(line_ending, parse_line),
        opt(line_ending),
    ))(input);
    result.expect("correct parser").1
}

fn parse_line(input: &str) -> nom::IResult<&str, (Point, Point)> {
    separated_pair(parse_point, tag(" -> "), parse_point)(input)
}

fn parse_point(input: &str) -> nom::IResult<&str, Point> {
    map(
        separated_pair(
            nom::character::complete::i32,
            tag(","),
            nom::character::complete::i32,
        ),
        |(x, y)| Point { x, y },
    )(input)
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Point {
    // technically unsigned, but having i32 avoid some casting down the line
    x: i32,
    y: i32,
}

impl Point {
    fn line_to(&self, to: &Point) -> PointLine {
        let dx = match self.x.cmp(&to.x) {
            cmp::Ordering::Less => 1,
            cmp::Ordering::Equal => 0,
            cmp::Ordering::Greater => -1,
        };

        let dy = match self.y.cmp(&to.y) {
            cmp::Ordering::Less => 1,
            cmp::Ordering::Equal => 0,
            cmp::Ordering::Greater => -1,
        };

        PointLine {
            should_stop: false,
            dx,
            dy,
            x: self.x,
            y: self.y,
            final_x: to.x,
            final_y: to.y,
        }
    }
}

struct PointLine {
    should_stop: bool,
    dx: i32,
    dy: i32,
    x: i32,
    y: i32,
    final_x: i32,
    final_y: i32,
}

impl std::iter::Iterator for PointLine {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.should_stop {
            return None;
        };

        let p = Some(Point {
            x: self.x,
            y: self.y,
        });
        if (self.x == self.final_x) && (self.y == self.final_y) {
            self.should_stop = true;
        }
        self.x += self.dx;
        self.y += self.dy;

        p
    }
}

trait Aoc {
    fn set_line(&mut self, from: &Point, to: &Point);
    fn count_intersections(&self) -> usize;
}

// Keep this around for posterity. It works, but the implementation
// using a vector as the underlying storage is 94% faster
#[derive(Default)]
struct HashmapGrid {
    points: BTreeMap<Point, u32>,
}

impl Aoc for HashmapGrid {
    fn set_line(&mut self, from: &Point, to: &Point) {
        for p in from.line_to(to) {
            let e = self.points.entry(p).or_default();
            *e += 1;
        }
    }

    fn count_intersections(&self) -> usize {
        self.points.iter().filter(|(_, v)| **v > 1).count()
    }
}

impl std::fmt::Debug for HashmapGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let max_x = self.points.keys().map(|p| p.x).max().unwrap();
        let max_y = self.points.keys().map(|p| p.y).max().unwrap();
        for y in 0..=max_y {
            for x in 0..=max_x {
                let val = self
                    .points
                    .get(&Point { x, y })
                    .map(|n| format!("{}", n))
                    .unwrap_or_else(|| ".".to_string());
                f.write_str(&val)?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

struct VecGrid {
    max_x: usize,
    max_y: usize,
    points: Vec<u32>,
}

impl VecGrid {
    fn from_lines(puzzle: &[(Point, Point)]) -> Self {
        let max_x = puzzle
            .iter()
            .map(|(p1, p2)| cmp::max(p1.x, p2.x))
            .max()
            .expect("at least one line");
        let max_y = puzzle
            .iter()
            .map(|(p1, p2)| cmp::max(p1.y, p2.y))
            .max()
            .expect("at least one line");

        let n = (max_x + 1) * (max_y + 1);
        Self {
            max_x: max_x as _,
            max_y: max_y as _,
            points: vec![0; n as usize],
        }
    }
}

impl std::fmt::Debug for VecGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let idx = y * self.max_x + x;
                let val = self
                    .points
                    .get(idx)
                    .map(|n| {
                        if *n == 0 {
                            ".".to_string()
                        } else {
                            format!("{}", n)
                        }
                    })
                    .unwrap();
                f.write_str(&val)?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Aoc for VecGrid {
    fn set_line(&mut self, from: &Point, to: &Point) {
        for p in from.line_to(to) {
            let idx = p.y as usize * self.max_x + p.x as usize;
            let e = self.points.get_mut(idx).unwrap();
            *e += 1
        }
    }

    fn count_intersections(&self) -> usize {
        self.points.iter().filter(|v| **v > 1).count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_parser() {
        assert_eq!(
            Ok(("", (Point { x: 8, y: 0 }, Point { x: 0, y: 8 }))),
            parse_line("8,0 -> 0,8")
        );
    }

    #[test]
    fn test_solve1() {
        assert_eq!(5, solve1(&parse_puzzle(TEST_INPUT)));
    }

    #[test]
    fn test_solve2() {
        assert_eq!(12, solve2(&parse_puzzle(TEST_INPUT)));
    }
}
