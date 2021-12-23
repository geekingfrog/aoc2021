use std::{cmp::max, fmt::Display};

pub fn solve() -> (usize, usize) {
    (solve1(), solve2())
}

fn solve1() -> usize {
    todo!()
}

fn solve2() -> usize {
    todo!()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug)]
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
        self.points[12] == Some(A)
            && self.points[13] == Some(A)
            && self.points[14] == Some(A)
            && self.points[15] == Some(A)
            && self.points[16] == Some(B)
            && self.points[17] == Some(B)
            && self.points[18] == Some(B)
            && self.points[19] == Some(B)
            && self.points[20] == Some(C)
            && self.points[21] == Some(C)
            && self.points[22] == Some(C)
            && self.points[23] == Some(C)
            && self.points[24] == Some(D)
            && self.points[25] == Some(D)
            && self.points[26] == Some(D)
            && self.points[27] == Some(D)
    }

    // possible destinations for the pod at index `idx`
    fn destinations(&self, idx: usize) -> Vec<usize> {
        let pod = match &self.points[idx] {
            Some(p) => p,
            None => unreachable!("Attempted to move non-existent pod at index {}", idx),
        };

        let silo_dest_idx = match pod {
            Pod::A => 11,
            Pod::B => 15,
            Pod::C => 19,
            Pod::D => 23,
        };

        // in hallway -> must move to final silo
        if idx < 11 {
            let pos = (silo_dest_idx..(silo_dest_idx + 4))
                .into_iter()
                .rev()
                .find(|i| self.points[*i].is_none());
            match pos {
                Some(pos) => vec![pos],
                None => vec![],
            }
        } else {
            // in silo, must move to hallway or final silo
            let silo_start_idx = ((idx - 11) / 4) * 2 + 2;

            // println!(
            //     "pod {:?} silo starts at {}",
            //     self.points[idx], silo_start_idx
            // );
            let mut dests = vec![];
            let left = [0, 1, 3, 5, 7, 9, 10]
                .into_iter()
                .rev()
                .skip_while(|i| *i > silo_start_idx)
                .take_while(|i| self.points[*i].is_none());
            dests.extend(left);
            // println!("dest after left {:?}", dests);

            let right = [0, 1, 3, 5, 7, 9, 10]
                .into_iter()
                .skip_while(|i| *i < silo_start_idx)
                .take_while(|i| self.points[*i].is_none());
            dests.extend(right);
            // println!("dest after right {:?}", dests);

            let in_silo = (silo_dest_idx..(silo_dest_idx + 4))
                .into_iter()
                .rev()
                .filter(|&i| i != idx && self.points[i].is_none());
            dests.extend(in_silo);
            dests
        }
    }

    fn move_pod(mut self, from: usize, to: usize) -> (usize, Self) {
        let p = self.points[from].expect("non empty space");
        self.points[from] = None;
        self.points[to] = Some(p);
        // due to the way moves are constrained, the only possible
        // moves are from silo to hallway and from hallway to silo
        // which is the same when it comes to computing distance
        let (from, to) = if from < 11 { (from, to) } else { (to, from) };

        let h_idx = ((to - 11) / 4) * 2 + 2;
        let depth = (to - 11) % 4;
        let h_dist = if from > h_idx {
            from - h_idx + 1
        } else {
            h_idx - from + 1
        };

        ((h_dist + depth) * p.energy(), self)
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
    use super::*;

    const TEST_INPUT: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";

    #[test]
    fn test_stuff() {
        use Pod::*;
        let parsed = parse_input(TEST_INPUT);
        let raw = [parsed[0], parsed[1], [A, B, C, D], [A, B, C, D]];
        let grid = Grid::from_input(raw);
        println!("{}", grid);

        let (d, grid) = grid.move_pod(15, 1);
        println!("{}", grid);
        assert_eq!(400, d);

        // println!("{:?}", grid.destinations(12));
        // assert!(!grid.destinations(12).contains(&0));
        // println!("{:?}", grid.destinations(1));

        let (d, grid) = grid.move_pod(19, 7);
        println!("{}", grid);
        assert_eq!(d, 20);
        println!("{} - {:?}", grid.points[11].unwrap(), grid.destinations(11));
        println!("{} - {:?}", grid.points[16].unwrap(), grid.destinations(16));

        assert_eq!(vec![3, 5, 15], grid.destinations(11));
        assert_eq!(vec![3, 5], grid.destinations(16));
    }

    #[test]
    fn test_solve1() {
        todo!()
    }

    #[test]
    fn test_solve2() {
        todo!()
    }
}
