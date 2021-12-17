use itertools::Itertools;
use nom::character::complete::char;
use nom::IResult;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt},
    sequence::{preceded, tuple},
};
use std::ops::Neg;

pub fn solve() -> (i32, usize) {
    let ta = TargetArea::from_str(include_str!("../resources/day17.txt"));
    (solve1(&ta), solve2(&ta))
}

fn solve1(ta: &TargetArea) -> i32 {
    (-ta.y0..-ta.y1)
        .into_iter()
        .map(|vy0| y_trajectory(vy0, ta.y0, ta.y1))
        .take_while(|x| x.is_some())
        .map(|x| x.unwrap())
        .max()
        .unwrap()
}

fn solve2(ta: &TargetArea) -> usize {
    (0..=ta.x1)
        .cartesian_product(ta.y1..=-ta.y1)
        .into_iter()
        .filter(|(vx0, vy0)| trajectory(*vx0, *vy0, ta).is_some())
        .count()
}

fn trajectory(vx0: i32, vy0: i32, ta: &TargetArea) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;
    let mut vx = vx0;
    let mut vy = vy0;
    let mut max_y = 0;

    loop {
        x += vx;
        y += vy;
        match vx.cmp(&0) {
            std::cmp::Ordering::Less => vx += 1,
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => vx -= 1,
        };
        vy -= 1;
        if vy == 0 {
            max_y = y;
        }

        if x >= ta.x0 && x <= ta.x1 && y <= ta.y0 && y >= ta.y1 {
            return Some(max_y);
        }
        if x > ta.x1 {
            return None;
        }
        if x < ta.x0 && vx <= 0 {
            return None;
        }
        if y < ta.y1 {
            return None;
        }
    }
}

fn y_trajectory(vy0: i32, ty0: i32, ty1: i32) -> Option<i32> {
    let mut max_y = 0;
    let mut vy = vy0;
    let mut y = 0;
    loop {
        y += vy;
        vy -= 1;
        if vy == 0 {
            max_y = y;
        }
        if y <= ty0 && y >= ty1 {
            return Some(max_y);
        }
        if y < ty1 {
            return None;
        }
    }
}

// fn fy(vy0: i32, n: i32) -> i32 {
//     n * vy0 - n * (n - 1) / 2
// }

#[derive(Debug)]
struct TargetArea {
    x0: i32,
    x1: i32,
    y0: i32,
    y1: i32,
}

impl TargetArea {
    fn from_str(raw: &str) -> Self {
        let (x0, x1, y0, y1) = preceded(
            tag("target area: "),
            tuple((
                preceded(tag("x="), parse_i32),
                preceded(tag(".."), parse_i32),
                preceded(tag(", y="), parse_i32),
                preceded(tag(".."), parse_i32),
            )),
        )(raw)
        .unwrap()
        .1;

        assert!(y0 < 0);
        assert!(y1 < 0);

        // ensure x0 and y0 are the closest to 0 compared to x1 and y1
        let (x0, x1) = if x0 >= x1 { (x1, x0) } else { (x0, x1) };
        let (y0, y1) = if y0 >= y1 { (y0, y1) } else { (y1, y0) };
        Self { x0, x1, y0, y1 }
    }
}

fn parse_i32(raw: &str) -> IResult<&str, i32> {
    map(
        tuple((opt(char('-')), digit1)),
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

    const TEST_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_solve1() {
        let ta = TargetArea::from_str(TEST_INPUT);
        assert_eq!(45, solve1(&ta));
    }

    #[test]
    fn test_solve2() {
        let ta = TargetArea::from_str(TEST_INPUT);
        assert_eq!(112, solve2(&ta));
    }
}
