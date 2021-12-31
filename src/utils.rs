use std::{ops::Neg, str::FromStr};

use nom::{
    character::complete::digit1,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

pub struct Grid<T> {
    pub points: Vec<T>,
    pub width: usize,
    pub height: usize,
}

pub type Point = (usize, usize);

impl<T> Grid<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            None
        } else {
            let idx = self.width * y + x;
            self.points.get(idx)
        }
    }

    pub fn idx(&self, point: (usize, usize)) -> usize {
        let (x, y) = point;
        self.width * y + x
    }

    pub fn neighbours(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        let mut ns = Vec::with_capacity(4);
        if x > 0 {
            ns.push((x - 1, y))
        }
        if x < self.width - 1 {
            ns.push((x + 1, y))
        }
        if y > 0 {
            ns.push((x, y - 1))
        }
        if y < self.height - 1 {
            ns.push((x, y + 1))
        }
        ns.into_iter()
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                f.write_fmt(format_args!("{}", self.get(x, y).unwrap()))?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

pub fn parse_i32(raw: &str) -> IResult<&str, i32> {
    parse_signed::<i32>(raw)
}

/// parse a signed digit
pub fn parse_signed<F>(raw: &str) -> IResult<&str, F>
where
    F: FromStr + Neg<Output = F>,
    <F as FromStr>::Err: std::fmt::Debug,
{
    map(
        tuple((opt(nom::character::complete::char('-')), digit1)),
        |(sign, ds): (Option<char>, &str)| {
            let x: F = ds.parse().unwrap();
            match sign {
                None => x,
                Some(_) => x.neg(),
            }
        },
    )(raw)
}
