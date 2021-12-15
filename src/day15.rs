use crate::utils::Point;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn solve() -> (usize, usize) {
    let grid = Grid::from_str(include_str!("../resources/day15.txt"));
    (solve1(&grid), solve2(&grid))
}

fn solve1(grid: &Grid) -> usize {
    let (_path, cost) = grid.dijkstra((0, 0), (grid.width - 1, grid.height - 1));
    cost
    // let path = grid.astar((0, 0), (grid.width - 1, grid.height - 1));
    // path.into_iter()
    //     .skip(1)
    //     .map(|p| grid.points[grid.idx(p)])
    //     .sum()
}

fn solve2(grid: &Grid) -> usize {
    let new_grid = extend_grid(grid);
    solve1(&new_grid)
}

type Grid = crate::utils::Grid<usize>;

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
        let mut parents: Vec<Option<(Point, usize)>> = std::iter::repeat(None)
            .take(self.width * self.height)
            .collect();
        parents[self.idx(start)] = Some((start, 0));

        let mut to_explore = BinaryHeap::from([Reverse((0, start))]);

        while let Some(x) = to_explore.pop() {
            let (dist, node) = x.0;
            for n in self.neighbours(node.0, node.1) {
                let val = self.get(n.0, n.1).unwrap();
                let new_dist = dist + val;
                match parents[self.idx(n)] {
                    Some((_parent, existing_dist)) => {
                        if new_dist <= existing_dist {
                            parents[self.idx(n)] = Some((node, new_dist));
                        }
                    }
                    None => {
                        parents[self.idx(n)] = Some((node, new_dist));
                        to_explore.push(Reverse((new_dist, n)))
                    }
                }
            }
        }

        let mut pos = end;
        let mut path = vec![end];
        while pos != start {
            let (parent, _cost) = parents[self.idx(pos)].unwrap();
            pos = parent;
            path.push(pos);
        }

        path.reverse();
        (path, parents[self.idx(end)].unwrap().1)
    }

    // Kept for posterity, but doesn't return the correct result, unless the
    // heuristic coefficient is set to 1, and in this case it visits as many
    // nodes as dijsktra, so no speed gain.
    #[allow(dead_code)]
    fn astar(&self, start: Point, end: Point) -> Vec<Point> {
        // by how much to multiply the manathan distance from a node to the end
        // setting it to 9 means to be maximally negative about every node.
        // setting it to 1 assumes that all node have a weight of 1, and so
        // a* will visit roughly the same number of node as dijkstra
        let heuristic_coef = 2;

        let mut visited = 0;
        let mut open_set = BinaryHeap::from([Reverse((
            heuristic_coef * (end.0 - start.0 + 1 + end.1 - start.1 + 1),
            start,
        ))]);
        let mut parents: Vec<Option<Point>> = std::iter::repeat(None)
            .take(self.width * self.height)
            .collect();
        let mut g_scores: Vec<Option<usize>> = std::iter::repeat(None)
            .take(self.width * self.height)
            .collect();
        g_scores[0] = Some(0);

        while let Some(x) = open_set.pop() {
            visited += 1;
            let (_f_score, current) = x.0;
            if current == end {
                break;
            }
            let node_score = g_scores[self.idx(current)].unwrap();
            for n in self.neighbours(current.0, current.1) {
                let neighbour_score = node_score + self.points[self.idx(n)];
                match g_scores[self.idx(n)] {
                    Some(s) => {
                        if neighbour_score < s {
                            parents[self.idx(n)] = Some(current);
                            g_scores[self.idx(n)] = Some(neighbour_score);
                        }
                    }
                    None => {
                        parents[self.idx(n)] = Some(current);
                        g_scores[self.idx(n)] = Some(neighbour_score);
                        let nf_score =
                            neighbour_score + heuristic_coef * (end.0 - n.0 - 1 + end.0 - n.1 - 1);
                        open_set.push(Reverse((nf_score, n)));
                    }
                }
            }
        }

        println!(
            "astar visited {} nodes out of {}",
            visited,
            self.width * self.height
        );
        let mut pos = end;
        let mut path = vec![end];
        while pos != start {
            let parent = parents[self.idx(pos)].unwrap();
            pos = parent;
            path.push(pos);
        }

        path.reverse();

        path
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
        let grid = Grid::from_str(include_str!("../resources/day15.txt"));
        assert_eq!(583, solve1(&grid));
    }

    #[test]
    fn test_solve2() {
        let grid = Grid::from_str(TEST_INPUT);
        assert_eq!(315, solve2(&grid));
        let grid = Grid::from_str(include_str!("../resources/day15.txt"));
        assert_eq!(2927, solve2(&grid));
    }
}
