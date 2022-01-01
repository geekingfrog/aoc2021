use std::cmp::Ordering;
use std::collections::{HashMap, BinaryHeap};
use std::fmt::Display;

pub fn solve() -> (usize, usize) {
    let grid = parse_input(include_str!("../resources/day23.txt"));
    (solve1(&grid), solve2(&grid))
}

fn solve1(pods: &[[Pod; 4]; 2]) -> usize {
    // return 0;
    use Pod::*;
    let raw = [pods[0], pods[1], [A, B, C, D], [A, B, C, D]];
    let grid = Grid::from_input(raw);
    move_all_pods(grid)
}

fn solve2(pods: &[[Pod; 4]; 2]) -> usize {
    use Pod::*;
    let raw = [pods[0], [D, C, B, A], [D, B, A, C], pods[1]];
    let grid = Grid::from_input(raw);
    move_all_pods(grid)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
enum Pod {
    A,
    B,
    C,
    D,
}

impl Pod {
    fn energy(&self) -> usize {
        match self {
            Pod::A => 1,
            Pod::B => 10,
            Pod::C => 100,
            Pod::D => 1000,
        }
    }
}

impl Display for Pod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pod::A => f.write_str("A"),
            Pod::B => f.write_str("B"),
            Pod::C => f.write_str("C"),
            Pod::D => f.write_str("D"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Grid {
    points: [Option<Pod>; 27],
}

impl Grid {
    fn from_input(input: [[Pod; 4]; 4]) -> Self {
        let mut points = [None; 27];
        points[11] = Some(input[0][0]);
        points[12] = Some(input[1][0]);
        points[13] = Some(input[2][0]);
        points[14] = Some(input[3][0]);
        points[15] = Some(input[0][1]);
        points[16] = Some(input[1][1]);
        points[17] = Some(input[2][1]);
        points[18] = Some(input[3][1]);
        points[19] = Some(input[0][2]);
        points[20] = Some(input[1][2]);
        points[21] = Some(input[2][2]);
        points[22] = Some(input[3][2]);
        points[23] = Some(input[0][3]);
        points[24] = Some(input[1][3]);
        points[25] = Some(input[2][3]);
        points[26] = Some(input[3][3]);
        Self { points }
    }

    fn is_finished(&self) -> bool {
        use Pod::*;
        self.points[11] == Some(A)
            && self.points[12] == Some(A)
            && self.points[13] == Some(A)
            && self.points[14] == Some(A)
            && self.points[15] == Some(B)
            && self.points[16] == Some(B)
            && self.points[17] == Some(B)
            && self.points[18] == Some(B)
            && self.points[19] == Some(C)
            && self.points[20] == Some(C)
            && self.points[21] == Some(C)
            && self.points[22] == Some(C)
            && self.points[23] == Some(D)
            && self.points[24] == Some(D)
            && self.points[25] == Some(D)
            && self.points[26] == Some(D)
    }

    // possible destinations for the pod at index `idx`
    fn dests(&self, idx: usize) -> Vec<(usize, usize)> {
        let pod = match &self.points[idx] {
            Some(p) => p,
            None => return vec![],
        };

        let (dest_room_idx, dest_room_exit) = match pod {
            Pod::A => (11, 2),
            Pod::B => (15, 4),
            Pod::C => (19, 6),
            Pod::D => (23, 8),
        };

        if idx < 11 {
            // in hallway, must move to final room
            let path_blocked = (idx.min(dest_room_exit)..=idx.max(dest_room_exit))
                .into_iter()
                .any(|i| i != idx && self.points[i].is_some());
            // println!(
            //     "path blocked from {} to {}: {}",
            //     idx, dest_room_exit, path_blocked
            // );
            if path_blocked {
                return vec![];
            }

            // further restrict to only move there if all pods
            // already there won't move
            let move_makes_sense =
                (dest_room_idx..(dest_room_idx + 4)).all(|i| match self.points[i] {
                    Some(p) => &p == pod,
                    None => true,
                });

            if !move_makes_sense {
                return vec![];
            }

            let pos = (dest_room_idx..(dest_room_idx + 4))
                .into_iter()
                .rev()
                .find(|i| self.points[*i].is_none());
            match pos {
                Some(pos) => {
                    let hdist = idx.max(dest_room_exit) - idx.min(dest_room_exit);
                    // println!("hdist: {}", hdist);
                    let vdist = pos - dest_room_idx + 1;
                    // println!("pos: {} - vdist: {}", pos, vdist);
                    let cost = (hdist + vdist) * pod.energy();
                    vec![(pos, cost)]
                }
                None => vec![],
            }
        } else {
            // in a room, move to the hallway
            // moving to a room can be done in another step

            let room_idx = (idx - 11) / 4;
            let room_exit_idx = room_idx * 2 + 2;
            let room_start_idx = room_idx * 4 + 11;

            // println!(
            //     "from idx {}, room exit is at {} and room starts at {}",
            //     idx, room_exit_idx, room_start_idx
            // );

            let mut can_exit = true;
            let mut move_makes_sense = idx == room_start_idx + 3;
            for i in room_start_idx..(room_start_idx+4) {
                match i.cmp(&idx) {
                    Ordering::Less => {
                        can_exit = can_exit && self.points[i].is_none();
                    },
                    Ordering::Equal => (),
                    Ordering::Greater => {
                        move_makes_sense = move_makes_sense || self.points[i] != Some(*pod);
                    },
                }
            }

            // println!(
            //     "idx {}({}) can exit? {} and makes sense? {}",
            //     idx, pod, can_exit, move_makes_sense
            // );

            if can_exit && move_makes_sense {
                let mut dests = vec![];
                let vdist = idx - room_start_idx + 1;
                let left = [0, 1, 3, 5, 7, 9, 10]
                    .into_iter()
                    .rev()
                    .skip_while(|i| *i > room_exit_idx)
                    .take_while(|i| self.points[*i].is_none())
                    .map(|i| {
                        let hdist = room_exit_idx - i;
                        (i, (hdist + vdist) * pod.energy())
                    });
                dests.extend(left);

                // println!("dest after left {:?}", dests);

                let right = [0, 1, 3, 5, 7, 9, 10]
                    .into_iter()
                    .skip_while(|i| *i < room_exit_idx)
                    .take_while(|i| self.points[*i].is_none())
                    .map(|i| {
                        let hdist = i - room_exit_idx;
                        (i, (hdist + vdist) * pod.energy())
                    });

                dests.extend(right);
                // println!("dest after right: {:?}", dests);

                dests
            } else {
                vec![]
            }
        }
    }

    fn move_pod(mut self, from: usize, to: usize) -> Self {
        let p = self.points[from].expect("non empty space");
        self.points[from] = None;
        self.points[to] = Some(p);
        self
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("#############\n")?;
        f.write_str("#")?;
        for i in 0..11 {
            match self.points[i] {
                Some(p) => write!(f, "{}", p)?,
                None => write!(f, ".")?,
            }
        }
        f.write_str("#\n")?;
        for i in 0..4 {
            if i == 0 {
                f.write_str("###")?;
            } else {
                f.write_str("  #")?;
            }
            for j in 0..4 {
                match self.points[11 + j * 4 + i] {
                    Some(p) => write!(f, "{}", p)?,
                    None => write!(f, ".")?,
                };
                f.write_str("#")?;
            }
            if i == 0 {
                f.write_str("##\n")?;
            } else {
                f.write_str("\n")?;
            }
        }
        f.write_str("  #########\n")?;
        Ok(())
    }
}

#[derive(Eq, PartialEq)]
struct Node(usize, Grid);

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

fn move_all_pods(grid: Grid) -> usize {
    let mut states = BinaryHeap::from([Node(0, grid)]);
    let mut costs = HashMap::new();
    let mut min_cost = usize::MAX;

    while let Some(Node(cost, grid)) = states.pop() {
        if grid.is_finished() {
            min_cost = min_cost.min(cost);
        }

        costs.insert(grid, cost);
        for (idx, p) in grid.points.iter().enumerate() {
            if p.is_none() {
                continue;
            }

            for (dest_idx, add_cost) in grid.dests(idx) {
                let g = grid.move_pod(idx, dest_idx);
                let c = cost + add_cost;
                match costs.get(&g) {
                    Some(c2) if c2 > &c => {
                        costs.insert(g, c);
                        states.push(Node(c,g));
                    },
                    None => states.push(Node(c, g)),
                    _ => ()
                }
            }
        }
    }
    min_cost
}

fn parse_input(raw: &str) -> [[Pod; 4]; 2] {
    use Pod::*;
    raw.lines()
        .skip(2)
        .take(2)
        .map(|l| {
            l.chars()
                .filter_map(|c| match c {
                    'A' => Some(A),
                    'B' => Some(B),
                    'C' => Some(C),
                    'D' => Some(D),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    const TEST_INPUT: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";

    // #[test]
    // fn test_stuff() {
    //     use Pod::*;
    //     let parsed = parse_input(TEST_INPUT);
    //     let raw = [parsed[0], parsed[1], [A, B, C, D], [A, B, C, D]];
    //     let grid = Grid::from_input(raw);
    //     println!("{}", grid);
    //
    //     let (d, grid) = grid.move_pod(15, 1);
    //     println!("{}", grid);
    //     assert_eq!(400, d);
    //     println!("{} - {:?}", grid.points[19].unwrap(), grid.destinations(19));
    //     assert!(grid.destinations(19).contains(&15));
    //
    //     let (d, grid) = grid.move_pod(19, 9);
    //     println!("{}", grid);
    //     assert_eq!(d, 40);
    //     println!("{} - {:?}", grid.points[11].unwrap(), grid.destinations(11));
    //     println!("{} - {:?}", grid.points[16].unwrap(), grid.destinations(16));
    //     println!("{} - {:?}", grid.points[1].unwrap(), grid.destinations(1));
    //
    //     let (d, grid) = grid.move_pod(23, 19); // illegal move but :shrug:
    //     println!("{}", grid);
    //     assert_eq!(d, 4000);
    //
    //     //
    //     // assert_eq!(vec![3, 5, 15], grid.destinations(11));
    //     // assert_eq!(vec![3, 5], grid.destinations(16));
    //
    //     assert!(false);
    // }

    // #[test]
    // fn test_stuff() {
    //     use Pod::*;
    //     let parsed = parse_input(TEST_INPUT);
    //     let raw = [parsed[0], parsed[1], [A, B, C, D], [A, B, C, D]];
    //     let grid = Grid::from_input(raw);
    //     println!("{}", grid);
    //
    //     println!("{} - {:?}", grid.points[11].unwrap(), grid.dests(11));
    //     let grid = grid.move_pod(11, 1);
    //     let grid = grid.move_pod(12, 10);
    //     let grid = grid.move_pod(15, 9);
    //     println!("{}", grid);
    //     println!("{}-{} - {:?}", 13, grid.points[13].unwrap(), grid.dests(13));
    //     println!("{}-{} - {:?}", 1, grid.points[1].unwrap(), grid.dests(1));
    //     println!("{}-{} - {:?}", 10, grid.points[10].unwrap(), grid.dests(10));
    //
    //     let grid = grid.move_pod(1, 15);
    //     println!("{}", grid);
    //     prn_ds(&grid, 15);
    //
    //     assert!(false);
    // }

    fn prn_ds(grid: &Grid, idx: usize) {
        println!(
            "{}-{} - {:?}",
            idx,
            grid.points[idx].unwrap(),
            grid.dests(idx)
        );
    }

    #[test]
    fn test_dests_to_room() {
        let mut grid = Grid::default();
        grid.points[3] = Some(Pod::A);
        println!("{}", grid);
        prn_ds(&grid, 3);
        assert_eq!(vec![(14, 5)], grid.dests(3));
    }

    #[test]
    fn test_dests_to_hall() {
        let mut grid = Grid::default();
        grid.points[18] = Some(Pod::A);
        println!("{}", grid);
        prn_ds(&grid, 18);
        let ds: HashSet<_> = grid.dests(18).into_iter().map(|x| x.0).collect();
        assert_eq!(HashSet::from([0, 1, 3, 5, 7, 9, 10]), ds);
    }

    #[test]
    fn test_dests_hall_2() {
        use Pod::*;
        let mut grid = Grid::default();
        grid.points[9] = Some(C);
        grid.points[10] = Some(A);
        grid.points[13] = Some(A);
        grid.points[14] = Some(B);
        grid.points[15] = Some(B);
        grid.points[16] = Some(D);
        grid.points[17] = Some(B);
        grid.points[18] = Some(B);

        println!("{}", grid);
        prn_ds(&grid, 15);
        let ds: HashSet<_> = grid.dests(15).into_iter().map(|x| x.0).collect();
        assert_eq!(HashSet::from([0, 1, 3, 5, 7]), ds);
    }

    #[test]
    fn test_solve1() {
        assert_eq!(12521, solve1(&parse_input(TEST_INPUT)))
    }

    #[test]
    fn test_solve2() {
        assert_eq!(44169, solve2(&parse_input(TEST_INPUT)))
    }
}
