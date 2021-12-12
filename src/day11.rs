use itertools::Itertools;

pub fn solve() -> (usize, usize) {
    let grid = parse(include_str!("../resources/day11.txt"));
    (solve1(&grid), solve2(&grid))
}

fn solve1(grid: &Grid) -> usize {
    let mut c = 0;
    let mut g = grid.clone();
    for _ in 0..100 {
        let (flashed, g2) = next_grid(g);
        g = g2;
        c += flashed;
    }
    c
}

fn solve2(grid: &Grid) -> usize {
    let mut step = 0;
    let mut grid = grid.clone();
    loop {
        if grid.iter().all(|&c| c == 0) {
            break;
        }
        let (_, g2) = next_grid(grid);
        grid = g2;
        step += 1;
    }
    step
}

type Grid = [u8; 100];

fn parse(raw: &str) -> Grid {
    raw.chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut ns = Vec::with_capacity(9);
    for dx in [x.checked_sub(1), Some(x), x.checked_add(1)] {
        for dy in [y.checked_sub(1), Some(y), y.checked_add(1)] {
            if let Some(px) = dx {
                if let Some(py) = dy {
                    if (px != x || py != y) && px < 10 && py < 10 {
                        ns.push((px, py))
                    }
                }
            }
        }
    }
    ns
}

fn next_grid(mut grid: Grid) -> (usize, Grid) {
    let mut flashed = Vec::new();
    let mut to_visit: Vec<(usize, usize)> = (0..10).cartesian_product(0..10).collect();

    while let Some((x, y)) = to_visit.pop() {
        let idx = y * 10 + x;
        grid[idx] += 1;
        if grid[idx] > 9 {
            if !flashed.contains(&idx) {
                flashed.push(idx);
                for (x2, y2) in neighbours(x, y) {
                    to_visit.push((x2, y2));
                }
            }
        }
    }

    let flash_count = flashed.len();
    for idx in flashed {
        grid[idx] = 0;
    }

    (flash_count, grid)
}

#[allow(dead_code)]
fn dbg_grid(grid: &Grid) {
    for y in 0..10 {
        for x in 0..10 {
            print!("{}", grid[y * 10 + x])
        }
        print!("\n");
    }
    print!("\n");
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    #[test]
    fn test_neighbourgs() {
        let v: Vec<(usize, usize)> = vec![(0, 1), (1, 0), (1, 1)];
        assert_eq!(v, neighbours(0, 0))
    }

    #[test]
    fn test_next_grid() {
        let g0 = parse(TEST_INPUT);
        let (c1, g1) = next_grid(g0);
        dbg_grid(&g1);
        assert_eq!(g1[0], 6);
        assert_eq!(c1, 0, "step 1");
        let (c2, _g2) = next_grid(g1);
        dbg_grid(&_g2);
        assert_eq!(c2, 35, "step 2");
    }

    #[test]
    fn test_solve1() {
        assert_eq!(1656, solve1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_solve2() {
        assert_eq!(195, solve2(&parse(TEST_INPUT)));
    }
}
