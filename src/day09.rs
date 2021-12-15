use std::collections::BTreeSet;

use itertools::Itertools;

pub fn solve() -> (usize, usize) {
    let grid = parse_grid(include_str!("../resources/day09.txt"));
    (solve1(&grid), solve2(&grid))
}

fn solve1(grid: &Grid) -> usize {
    grid.low_points()
        .map(|(x, y)| *grid.get(x, y).unwrap() as usize + 1)
        .sum()
}

fn solve2(grid: &Grid) -> usize {
    let mut bassins_len = grid
        .low_points()
        .map(|(x, y)| grid.bassin_coords(x, y).len())
        .collect::<Vec<_>>();

    bassins_len.sort_unstable();
    bassins_len.iter().rev().take(3).product()
}

type Grid = crate::utils::Grid<u8>;

impl Grid {
    fn low_points(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.height)
            .cartesian_product(0..self.width)
            .filter_map(|(y, x)| {
                let val = self.get(x, y).unwrap();
                if self
                    .neighbours(x, y)
                    .filter_map(|(x, y)| self.get(x, y))
                    .min()
                    .unwrap()
                    > val
                {
                    Some((x, y))
                } else {
                    None
                }
            })
    }

    fn bassin_coords(&self, x: usize, y: usize) -> BTreeSet<(usize, usize)> {
        let mut seen = BTreeSet::new();
        let mut to_check = vec![(x, y)];
        while let Some((x, y)) = to_check.pop() {
            seen.insert((x, y));
            for n in self.neighbours(x, y) {
                let (nx, ny) = n;
                if !seen.contains(&n) && (self.get(nx, ny).copied() != Some(9)) {
                    to_check.push(n)
                }
            }
        }
        seen
    }
}

fn parse_grid(raw: &str) -> Grid {
    let width = raw.split_terminator('\n').next().unwrap().len();
    let height = raw.split_terminator('\n').count();
    let points = raw
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as _))
        .collect();
    Grid {
        points,
        width,
        height,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678
";

    #[test]
    fn test_solve1() {
        assert_eq!(15, solve1(&parse_grid(TEST_INPUT)))
    }

    #[test]
    fn test_bassin_coords() {
        let grid = parse_grid(TEST_INPUT);
        assert_eq!(3, grid.bassin_coords(0, 0).len(), "top left");
        assert_eq!(9, grid.bassin_coords(9, 0).len(), "top right");
        assert_eq!(14, grid.bassin_coords(2, 2).len(), "middle");
        assert_eq!(9, grid.bassin_coords(6, 4).len(), "bottom right");
    }

    #[test]
    fn test_solve2() {
        let grid = parse_grid(TEST_INPUT);
        assert_eq!(1134, solve2(&grid));
    }
}
