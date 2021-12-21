use itertools::Itertools;

pub fn solve() -> (usize, usize) {
    let p = Puzzle::from_str(include_str!("../resources/day20.txt"));
    (solve1(p.clone()), solve2(p))
}

fn solve1(puzzle: Puzzle) -> usize {
    let p = puzzle.enhance_n(2);
    p.image.into_iter().filter(|b| *b).count()
}

fn solve2(puzzle: Puzzle) -> usize {
    puzzle
        .enhance_n(50)
        .image
        .into_iter()
        .filter(|b| *b)
        .count()
}

#[derive(Debug, Clone)]
struct Puzzle {
    alg: Vec<bool>,
    side: usize,
    image: Vec<bool>,
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
        let mut side = 0;

        let image = ls
            .enumerate()
            .flat_map(|(y, xs)| {
                side = y + 1;
                xs.chars().map(move |c| match c {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!("unknown char: {}", c),
                })
            })
            .collect();

        Puzzle { alg, image, side }
    }

    fn enhance_n(self, n: usize) -> Self {
        let mut p = self;

        for i in 0..n {
            // println!("\n--------- enhancing iteration {}\n", i);
            let capa = (p.side + 2) * (p.side + 2);
            let mut image = Vec::with_capacity(capa);
            let default = if p.alg[0] {
                if i % 2 == 0 {
                    p.alg[p.alg.len() - 1]
                } else {
                    p.alg[0]
                }
            } else {
                false
            };

            for y in 0..((p.side + 2) as isize) {
                for x in 0..((p.side + 2) as isize) {
                    image.push(p.is_next_pixel_lit(x - 1, y - 1, default))
                }
            }

            p.side += 2;
            p.image = image;
        }
        p
    }

    fn is_next_pixel_lit(&self, x: isize, y: isize, default: bool) -> bool {
        let idx = [y - 1, y, y + 1]
            .into_iter()
            .cartesian_product([x - 1, x, x + 1])
            .fold(0, |acc, (y, x)| {
                let out_of_bound =
                    x < 0 || y < 0 || x >= self.side as isize || y >= self.side as isize;
                // println!(
                //     "checking ({},{}) oob? {}, side: {}",
                //     x, y, out_of_bound, self.side
                // );
                let x = if out_of_bound {
                    if default {
                        1
                    } else {
                        0
                    }
                } else {
                    let idx = (y as usize) * self.side + (x as usize);
                    if self.image[idx] {
                        1
                    } else {
                        0
                    }
                };

                (acc << 1) + x
            });
        self.alg[idx]
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.alg.iter().try_for_each(|b| {
            if *b {
                f.write_str("#")?;
            } else {
                f.write_str(".")?;
            };
            std::fmt::Result::Ok(())
        })?;

        f.write_str("\n")?;
        f.write_str("\n")?;

        for y in 0..self.side {
            for x in 0..self.side {
                let lit = self.image[y * self.side + x];
                if lit {
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
    fn test_solve2() {
        assert_eq!(3351, solve2(Puzzle::from_str(TEST_INPUT)));
    }
}
