use std::fmt::Display;

pub fn solve() -> (usize, usize) {
    let grid = Grid::from_str(include_str!("../resources/day25.txt"));
    (solve1(grid), solve2())
}

fn solve1(mut grid: Grid) -> usize {
    let mut has_moved = true;
    let mut n = 0;
    while has_moved {
        n += 1;
        has_moved = false;
        for dir in [Dir::East, Dir::South] {
            let idx_to_move = (0..grid.points.len())
                .into_iter()
                .filter_map(|i| grid.can_move(i, dir))
                .collect::<Vec<_>>();
            if !idx_to_move.is_empty() {
                has_moved = true
            };
            for (i, j) in idx_to_move {
                grid.points[j] = grid.points[i];
                grid.points[i] = None;
            }
        }
    }
    n
}

fn solve2() -> usize {
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    East,
    South,
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dir::East => f.write_str(">"),
            Dir::South => f.write_str("v"),
        }
    }
}

impl std::convert::TryFrom<char> for Dir {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '>' => Ok(Dir::East),
            'v' => Ok(Dir::South),
            _ => Err(format!("unknown char: {}", c)),
        }
    }
}

impl std::ops::Not for Dir {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Dir::East => Dir::South,
            Dir::South => Dir::East,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    points: Vec<Option<Dir>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in 0..self.height {
            for i in 0..self.width {
                let idx = j * self.width + i;
                match &self.points[idx] {
                    Some(d) => write!(f, "{}", d)?,
                    None => f.write_str(".")?,
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Grid {
    fn from_str(raw: &str) -> Self {
        let mut points = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for line in raw.lines() {
            width = line.len();
            height += 1;
            for c in line.chars() {
                points.push(c.try_into().ok());
            }
        }
        Self {
            width,
            height,
            points,
        }
    }

    // if can move, returns Some(idx, next_idx)
    fn can_move(&self, idx: usize, dir: Dir) -> Option<(usize, usize)> {
        let y = idx / self.width;
        let x = idx % self.width;
        // println!("idx: {} - ({},{})", idx, x, y);
        match self.points[idx] {
            Some(Dir::East) if dir == Dir::East => {
                let next_idx = if x == self.width - 1 {
                    idx + 1 - self.width
                } else {
                    idx + 1
                };
                match self.points[next_idx] {
                    None => Some((idx, next_idx)),
                    Some(_) => None,
                }
            }
            Some(Dir::South) if dir == Dir::South => {
                let next_idx = if y == self.height - 1 {
                    x
                } else {
                    idx + self.width
                };
                match self.points[next_idx] {
                    None => Some((idx, next_idx)),
                    Some(_) => None,
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
";

    #[test]
    fn test_solve1() {
        let grid = Grid::from_str(TEST_INPUT);
        assert_eq!(58, solve1(grid));
    }

    #[test]
    fn test_solve2() {
        todo!()
    }
}
