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
    let mut grid = Grid::default();
    for (from, to) in puzzle {
        if (from.x == to.x) || (from.y == to.y) {
            grid.set_line(&from, &to);
        }
    }
    grid.count_intersections()
}

fn solve2(puzzle: &[(Point, Point)]) -> usize {
    let mut grid = Grid::default();
    for (from, to) in puzzle {
        grid.set_line(&from, &to);
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
            nom::character::complete::u32,
            tag(","),
            nom::character::complete::u32,
        ),
        |(x, y)| Point { x, y },
    )(input)
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Default)]
struct Grid {
    points: BTreeMap<Point, u32>,
}

impl Grid {
    fn set_line(&mut self, from: &Point, to: &Point) {
        let dx = match from.x.cmp(&to.x) {
            cmp::Ordering::Less => 1,
            cmp::Ordering::Equal => 0,
            cmp::Ordering::Greater => -1,
        };

        let dy = match from.y.cmp(&to.y) {
            cmp::Ordering::Less => 1,
            cmp::Ordering::Equal => 0,
            cmp::Ordering::Greater => -1,
        };

        let mut x = from.x as i32;
        let mut y = from.y as i32;
        loop {
            let e = self
                .points
                .entry(Point {
                    x: x as _,
                    y: y as _,
                })
                .or_default();
            *e += 1;

            if x == to.x as _ && y == to.y as _ {
                break;
            }
            x += dx;
            y += dy;
        }
    }

    fn count_intersections(&self) -> usize {
        self.points.iter().filter(|(_, v)| **v > 1).count()
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let max_x = self.points.keys().map(|p| p.x).max().unwrap();
        let max_y = self.points.keys().map(|p| p.y).max().unwrap();
        for y in 0..=max_y {
            for x in 0..=max_x {
                let val = self
                    .points
                    .get(&Point { x, y })
                    .map(|n| format!("{}", n))
                    .unwrap_or(".".to_string());
                f.write_str(&val)?;
            }
            f.write_str("\n")?;
        }
        Ok(())
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
