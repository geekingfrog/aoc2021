use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn solve() -> (usize, usize) {
    let grid = Grid::from_str(include_str!("../resources/day15.txt"));
    (solve1(&grid), solve2(&grid))
}

fn solve1(grid: &Grid) -> usize {
    let (_path, cost) = grid.dijkstra((0, 0), (grid.width - 1, grid.height - 1));
    cost
}

fn solve2(grid: &Grid) -> usize {
    let new_grid = extend_grid(grid);
    solve1(&new_grid)
}

type Grid = crate::utils::Grid<usize>;
type Point = (usize, usize);

impl Grid {
    fn from_str(raw: &str) -> Self {
        let width = raw.split_terminator('\n').next().unwrap().len();
        let height = raw.split_terminator('\n').count();
        let points = raw
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as _))
            .collect();
        Self {
            points,
            width,
            height,
        }
    }

    /// shortest path from `start` to `end`.
    fn dijkstra(&self, start: Point, end: Point) -> (Vec<Point>, usize) {
        let mut sources: Vec<Option<(Point, usize)>> = std::iter::repeat(None)
            .take(self.width * self.height)
            .collect();
        sources[self.idx(start)] = Some((start, 0));

        let mut to_explore = BinaryHeap::from([Reverse((0, start))]);

        while let Some(x) = to_explore.pop() {
            let (dist, node) = x.0;
            for n in self.neighbours(node.0, node.1) {
                let val = self.get(n.0, n.1).unwrap();
                let new_dist = dist + val;
                match sources[self.idx(n)] {
                    Some((_parent, existing_dist)) => {
                        if new_dist <= existing_dist {
                            sources[self.idx(n)] = Some((node, new_dist));
                        }
                    }
                    None => {
                        sources[self.idx(n)] = Some((node, new_dist));
                        to_explore.push(Reverse((new_dist, n)))
                    }
                }
            }
        }

        let mut pos = end;
        let mut path = vec![end];
        while pos != start {
            let (parent, _cost) = sources[self.idx(pos)].unwrap();
            pos = parent;
            path.push(pos);
        }

        path.reverse();
        (path, sources[self.idx(end)].unwrap().1)
    }
}

fn extend_grid(grid: &Grid) -> Grid {
    let mut new_points = Vec::with_capacity(grid.width * grid.height * 25);
    for y in 0..grid.height * 5 {
        for x in 0..grid.width * 5 {
            let val = grid.get(x % grid.width, y % grid.height).unwrap();
            let offset = x / grid.width + y / grid.height;
            let mut new_val = val + offset;
            if new_val > 9 {
                new_val -= 9;
            }
            new_points.push(new_val);
        }
    }
    Grid {
        points: new_points,
        width: grid.width * 5,
        height: grid.height * 5,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

    #[test]
    fn test_solve1() {
        let grid = Grid::from_str(TEST_INPUT);
        assert_eq!(40, solve1(&grid));
    }

    #[test]
    fn test_solve2() {
        let grid = Grid::from_str(TEST_INPUT);
        assert_eq!(315, solve2(&grid));
    }
}
