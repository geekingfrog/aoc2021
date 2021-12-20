use itertools::Itertools;
use std::collections::BTreeSet;

pub fn solve() -> (usize, usize) {
    let p = Puzzle::from_str(include_str!("../resources/day20.txt"));
    (solve1(p.clone()), solve2(p))
}

fn solve1(puzzle: Puzzle) -> usize {
    let p = puzzle.enhance_n(2);
    p.image.len()
}

fn solve2(puzzle: Puzzle) -> usize {
    puzzle.enhance_n(50).image.len()
}

#[derive(Debug, Clone)]
struct Puzzle {
    alg: Vec<bool>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    image: BTreeSet<(isize, isize)>,
}

impl Puzzle {
    fn from_str(raw: &str) -> Self {
        let mut ls = raw.lines();
        let alg = ls
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => unreachable!("unknown char: {}", c),
            })
            .collect();

        // skip empty line
        ls.next();
        let mut max_x = 0;
        let mut max_y = 0;

        let image = ls
            .enumerate()
            .flat_map(|(y, xs)| {
                max_x = xs.len() - 1;
                max_y = y;
                xs.chars().enumerate().map(move |(x, c)| match c {
                    '#' => ((x as _, y as _), true),
                    '.' => ((x as _, y as _), false),
                    _ => unreachable!("unknown char: {}", c),
                })
            })
            .fold(BTreeSet::new(), |mut acc, (p, b)| {
                if b {
                    acc.insert(p);
                };
                acc
            });

        Puzzle {
            alg,
            min_x: 0,
            max_x: max_x.try_into().unwrap(),
            min_y: 0,
            max_y: max_y.try_into().unwrap(),
            image,
        }
    }

    fn enhance_n(self, n: usize) -> Self {
        let mut p = self;

        for i in 0..n {
            let mut image = BTreeSet::new();
            let default = if p.alg[0] {
                if i % 2 == 0 {
                    p.alg[p.alg.len() - 1]
                } else {
                    p.alg[0]
                }
            } else {
                false
            };

            for y in p.min_y - 1..=p.max_y + 1 {
                for x in p.min_x - 1..=p.max_x + 1 {
                    if p.is_next_pixel_lit(x, y, default) {
                        image.insert((x, y));
                    }
                }
            }

            p.min_x -= 1;
            p.min_y -= 1;
            p.max_x += 1;
            p.max_y += 1;

            p.image = image;
        }
        p
    }

    fn is_next_pixel_lit(&self, x: isize, y: isize, default: bool) -> bool {
        let idx = [y - 1, y, y + 1]
            .into_iter()
            .cartesian_product([x - 1, x, x + 1])
            .fold(0, |acc, (y, x)| {
                let out_of_bound = x < self.min_x
                    || x > self.max_x
                    || y < self.min_y
                    || y > self.max_y;
                let x = if (out_of_bound && default) || self.image.contains(&(x,y)) {
                    1
                } else {
                    0
                };

                (acc << 1) + x
            });
        self.alg[idx]
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // self.alg.iter().try_for_each(|b| {
        //     if *b {
        //         f.write_str("#")?;
        //     } else {
        //         f.write_str(".")?;
        //     };
        //     std::fmt::Result::Ok(())
        // })?;
        //
        // f.write_str("\n")?;
        // f.write_str("\n")?;

        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                if self.image.contains(&(x, y)) {
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

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test_solve1() {
        assert_eq!(35, solve1(Puzzle::from_str(TEST_INPUT)));
    }

    #[test]
    #[ignore]
    fn test_solve2() {
        assert_eq!(3351, solve2(Puzzle::from_str(TEST_INPUT)));
    }
}
