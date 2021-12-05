use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space0, space1};
use nom::combinator::all_consuming;
use nom::multi::{count, separated_list1};
use nom::sequence::{preceded, terminated, tuple};
use std::collections::BTreeSet;

pub fn solve() -> (usize, usize) {
    let puzzle = parse_puzzle(include_str!("../resources/day04.txt"));
    (solve1(&puzzle), solve2(&puzzle))
}

fn solve1(puzzle: &Puzzle) -> usize {
    let mut drawn_nums: Vec<u8> = vec![];
    for n in &puzzle.numbers {
        drawn_nums.push(*n);
        for grid in &puzzle.grids {
            if grid.has_won(&drawn_nums) {
                return get_magic_num(&drawn_nums, &grid, *n);
            }
        }
    }
    unreachable!()
}

fn solve2(puzzle: &Puzzle) -> usize {
    let mut drawn_nums = vec![];
    let mut remaining_grids: BTreeSet<&Grid> = puzzle.grids.iter().collect();
    for n in &puzzle.numbers {
        drawn_nums.push(*n);
        for grid in &puzzle.grids {
            if grid.has_won(&drawn_nums) {
                remaining_grids.remove(&grid);
                if remaining_grids.is_empty() {
                    return get_magic_num(&drawn_nums, &grid, *n);
                }
            }
        }
    }
    unreachable!()
}

fn get_magic_num(drawn_nums: &Vec<u8>, grid: &Grid, n: u8) -> usize {
    let s: usize = grid
        .nums
        .iter()
        .filter_map(|k| {
            if !drawn_nums.contains(k) {
                Some(*k as usize)
            } else {
                None
            }
        })
        .sum::<usize>();
    return s * (n as usize);
}

#[derive(Debug)]
struct Puzzle {
    numbers: Vec<u8>,
    grids: Vec<Grid>,
}

fn parse_puzzle(input: &str) -> Puzzle {
    let r: nom::IResult<_, _> = all_consuming(tuple((
        terminated(
            separated_list1(tag(","), nom::character::complete::u8),
            line_ending,
        ),
        preceded(line_ending, separated_list1(line_ending, parse_grid)),
    )))(input);
    let (_rest, (numbers, grids)) = r.expect("valid parser");
    Puzzle { numbers, grids }
}

fn parse_grid(input: &str) -> nom::IResult<&str, Grid> {
    let parse_row = separated_list1(space1, preceded(space0, nom::character::complete::u8));
    nom::combinator::map(
        count(terminated(parse_row, line_ending), 5),
        |nums: Vec<Vec<u8>>| {
            let nums: Vec<_> = nums.into_iter().flatten().collect();
            Grid {
                nums: nums.try_into().expect("correct size"),
            }
        },
    )(input)
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
struct Grid {
    nums: [u8; 25],
}

impl Grid {
    fn has_won(&self, drawn_nums: &Vec<u8>) -> bool {
        self.has_won_manual(drawn_nums)
    }

    fn has_won_manual(&self, drawn_nums: &Vec<u8>) -> bool {
        for row in 0..5 {
            let mut r = true;
            for col in 0..5 {
                let idx = row * 5 + col;
                let hit = drawn_nums.contains(&self.nums[idx]);
                r = r && hit;
                if !r {
                    break;
                }
            }
            if r {
                return true;
            }
        }

        for col in 0..5 {
            let mut r = true;
            for row in 0..5 {
                let idx = row * 5 + col;
                let hit = drawn_nums.contains(&self.nums[idx]);
                r = r && hit;
                if !r {
                    break;
                }
            }
            if r {
                return true;
            }
        }

        false
    }

    // nicer, but slower version.
    #[allow(dead_code)]
    fn has_won_iter(&self, drawn_nums: &Vec<u8>) -> bool {
        let row = self
            .nums
            .iter()
            .chunks(5)
            .into_iter()
            .any(|row| row.into_iter().all(|n| drawn_nums.contains(n)));
        if row {
            return true;
        }
        let col = (0..5).into_iter().any(|offset| {
            self.nums
                .iter()
                .skip(offset)
                .step_by(5)
                .all(|n| drawn_nums.contains(n))
        });
        col
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    #[test]
    fn test_solve1() {
        assert_eq!(4512, solve1(&parse_puzzle(TEST)));
    }

    #[test]
    fn test_solve2() {
        assert_eq!(1924, solve2(&parse_puzzle(TEST)));
    }

    #[test]
    fn test_has_won() {
        let grid = Grid {
            nums: [
                3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16,
                12, 6,
            ],
        };
        let drawns = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13]
            .into_iter()
            .collect();
        assert!(grid.has_won(&drawns));
    }
}
