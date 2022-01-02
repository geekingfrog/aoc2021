use ahash::AHashMap;
use std::fmt::{Display, Formatter};
use std::iter::Once;

type Map<K, V> = AHashMap<K, V>;

pub fn solve() -> (usize, usize) {
    let grid = parse_input(include_str!("../resources/day23.txt"));
    (solve1(&grid), solve2(&grid))
}

fn solve1(pods: &[[Pod; 4]; 2]) -> usize {
    let cavern = Cavern::from_input(pods);
    move_all_pods_rec_cavern(cavern, &mut Map::default(), 0, usize::MAX)
}

fn solve2(pods: &[[Pod; 4]; 2]) -> usize {
    use Pod::*;
    let pods = [pods[0], [D, C, B, A], [D, B, A, C], pods[1]];
    let cavern = Cavern::from_input(&pods);
    move_all_pods_rec_cavern(cavern, &mut Map::default(), 0, usize::MAX)
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

    fn room_idx(&self) -> usize {
        match self {
            Pod::A => 0,
            Pod::B => 1,
            Pod::C => 2,
            Pod::D => 3,
        }
    }
}

impl std::convert::From<usize> for Pod {
    fn from(n: usize) -> Self {
        match n {
            0 => Pod::A,
            1 => Pod::B,
            2 => Pod::C,
            3 => Pod::D,
            _ => unreachable!("invalid pod number {}", n),
        }
    }
}

impl Display for Pod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Pod::A => f.write_str("A"),
            Pod::B => f.write_str("B"),
            Pod::C => f.write_str("C"),
            Pod::D => f.write_str("D"),
        }
    }
}



#[derive(Debug, Clone, Copy)]
enum Loc {
    H(usize),
    R { col: usize, row: usize },
}

#[derive(Clone, Copy, Eq)]
struct Cavern<const N: usize> {
    hallway: [Option<Pod>; 7],
    rooms: [[Option<Pod>; N]; 4],
}

impl<const N: usize> PartialEq for Cavern<N> {
    fn eq(&self, other: &Self) -> bool {
        self.hallway == other.hallway && self.rooms == other.rooms
    }
}

impl<const N: usize> std::hash::Hash for Cavern<N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_usize().hash(state);
    }
}

impl<const N: usize> Cavern<N> {
    // input given row-wise
    fn from_input(input: &[[Pod; 4]; N]) -> Self {
        let hallway = [None; 7];
        let mut rooms = [[None; N]; 4];

        for (row, r) in input.iter().enumerate() {
            for (col, p) in r.iter().enumerate() {
                rooms[col][row] = Some(*p);
            }
        }

        Self { hallway, rooms }
    }

    fn to_usize(self) -> usize {
        use Pod::*;
        let rooms = self.rooms.iter().flat_map(|r| r.iter());
        self.hallway.iter().chain(rooms).fold(0, |acc, p| {
            acc * 5
                + match p {
                    Some(A) => 1,
                    Some(B) => 2,
                    Some(C) => 3,
                    Some(D) => 4,
                    None => 0,
                }
        })
    }

    fn get(&self, loc: Loc) -> Option<Pod> {
        match loc {
            Loc::H(i) => self.hallway[i],
            Loc::R { col, row } => self.rooms[col][row],
        }
    }

    fn set(&mut self, loc: Loc, p: Option<Pod>) {
        match loc {
            Loc::H(h) => self.hallway[h] = p,
            Loc::R { col, row } => self.rooms[col][row] = p,
        }
    }

    fn is_finished(&self) -> bool {
        use Pod::*;
        self.rooms[0].iter().all(|p| p == &Some(A))
            && self.rooms[1].iter().all(|p| p == &Some(B))
            && self.rooms[2].iter().all(|p| p == &Some(C))
            && self.rooms[3].iter().all(|p| p == &Some(D))
    }

    fn pods(&self) -> impl Iterator<Item = (Loc, Pod)> + '_ {
        let rooms = self.rooms.iter().enumerate().flat_map(|(col, rooms)| {
            rooms
                .iter()
                .enumerate()
                .filter_map(move |(row, r)| r.map(|r| (Loc::R { col, row }, r)))
        });
        self.hallway
            .iter()
            .enumerate()
            .filter_map(|(h_idx, p)| p.map(|p| (Loc::H(h_idx), p)))
            .chain(rooms)
    }

    fn dests(&self, loc: Loc) -> PodStep {
        let pod = match self.get(loc) {
            Some(p) => p,
            None => return PodStep::Empty,
        };
        match loc {
            // in hallway, may move to final room only (terms & conditions apply)
            Loc::H(h_idx) => self.move_to_room(pod, h_idx),

            // in a room, may move to hallway under some circumstances
            Loc::R { col, row } => {
                self.move_to_hallway(pod, col, row)
            }
        }
    }

    fn move_to_room(&self, pod: Pod, h_idx: usize) -> PodStep {
        // check all pods in the target room are the right ones
        // can only move there if none of them will have to
        // move later on
        let ri = pod.room_idx();
        let should_move = self.rooms[ri]
            .iter()
            .all(|room_pod| room_pod.map(|p2| p2 == pod).unwrap_or(true));
        let can_move = (h_idx.min(ri + 2)..(h_idx.max(ri + 2)))
            .all(|h2| h2 == h_idx || self.hallway[h2].is_none());

        if should_move && can_move {
            let target_idx = self.rooms[ri]
                .iter()
                .enumerate()
                .rev()
                .find(|(_, p)| p.is_none())
                .map(|x| x.0);
            match target_idx {
                Some(i) => {
                    let loc = Loc::R { col: ri, row: i };
                    let dist = hdist(ri, h_idx) + i + 1;
                    PodStep::ToRoom(std::iter::once((loc, dist * pod.energy())))
                }
                None => PodStep::Empty,
            }
        } else {
            PodStep::Empty
        }
    }

    fn move_to_hallway(&self, pod: Pod, col: usize, row: usize) -> PodStep {
        let ri = pod.room_idx();

        let should_move = col != ri
            || self.rooms[col][(row + 1)..N].iter().any(|p2| match p2 {
                Some(p2) => p2.room_idx() != col,
                None => true,
            });
        let can_move = (0..row).is_empty() || self.rooms[col][0..row].iter().all(|x| x.is_none());

        if should_move && can_move {
            let left = (0..(col + 2))
                .rev()
                .take_while(|h_idx| self.hallway[*h_idx].is_none())
                .map(|h_idx| {
                    let dist = row + 1 + hdist(col, h_idx);
                    (Loc::H(h_idx), dist * pod.energy())
                })
                .collect();

            let right = ((col + 2)..self.hallway.len())
                .take_while(|h_idx| self.hallway[*h_idx].is_none())
                .map(|h_idx| {
                    let dist = row + 1 + hdist(col, h_idx);
                    (Loc::H(h_idx), dist * pod.energy())
                })
                .collect();

            PodStep::ToHallway {
                left,
                li: Some(0),
                right,
                ri: None,
            }
        } else {
            PodStep::Empty
        }
    }

    fn move_pod(mut self, from: Loc, to: Loc) -> Self {
        let p = self.get(from);
        self.set(from, None);
        self.set(to, p);
        self
    }
}

// assuming the pod is in the hallway, in front of a room
// what's the distance until index h_idx in the hallway?
fn hdist(ri: usize, h_idx: usize) -> usize {
    let d = if h_idx == 0 || h_idx == 6 { 1 } else { 0 };

    let base = if h_idx <= ri + 1 {
        ri + 1 - h_idx
    } else {
        h_idx - (ri + 2)
    };
    base * 2 + 1 - d
}

impl<const N: usize> Display for Cavern<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn fmt_pod(f: &mut Formatter<'_>, p: Option<Pod>) -> std::fmt::Result {
            match p {
                Some(p) => write!(f, "{}", p)?,
                None => write!(f, ".")?,
            }
            Ok(())
        }

        f.write_str("#############\n")?;
        f.write_str("#")?;
        fmt_pod(f, self.hallway[0])?;
        fmt_pod(f, self.hallway[1])?;
        f.write_str(".")?;
        fmt_pod(f, self.hallway[2])?;
        f.write_str(".")?;
        fmt_pod(f, self.hallway[3])?;
        f.write_str(".")?;
        fmt_pod(f, self.hallway[4])?;
        f.write_str(".")?;
        fmt_pod(f, self.hallway[5])?;
        fmt_pod(f, self.hallway[6])?;
        f.write_str("#\n")?;

        for i in 0..N {
            if i == 0 {
                f.write_str("###")?;
            } else {
                f.write_str("  #")?;
            }

            for r in 0..4 {
                fmt_pod(f, self.rooms[r][i])?;
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

#[derive(Clone)]
enum PodStep {
    Empty,
    ToRoom(Once<(Loc, usize)>),
    ToHallway {
        left: Vec<(Loc, usize)>,
        li: Option<usize>,
        right: Vec<(Loc, usize)>,
        ri: Option<usize>,
    },
}

impl Iterator for PodStep {
    /// destination_index, energy spent
    type Item = (Loc, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            PodStep::Empty => None,
            PodStep::ToRoom(x) => x.next(),
            PodStep::ToHallway {
                left,
                right,
                li,
                ri,
            } => {
                match li {
                    Some(i) if *i < left.len() => {
                        let r = left[*i];
                        *i += 1;
                        return Some(r);
                    }
                    Some(_) => {
                        *li = None;
                        *ri = Some(0);
                    }
                    _ => (),
                };
                match ri {
                    Some(i) if *i < right.len() => {
                        let r = right[*i];
                        *i += 1;
                        Some(r)
                    }
                    _ => None,
                }
            }
        }
    }
}

fn move_all_pods_rec_cavern<const N: usize>(
    cavern: Cavern<N>,
    mut cache: &mut Map<Cavern<N>, usize>,
    cost: usize,
    min_cost: usize,
) -> usize {
    if let Some(c2) = cache.get(&cavern) {
        if *c2 <= cost {
            return min_cost;
        }
    }

    cache.insert(cavern, cost);
    if cavern.is_finished() {
        return min_cost.min(cost);
    }

    let mut min_cost = min_cost;
    for (loc, _p) in cavern.pods() {
        for (dest_idx, add_cost) in cavern.dests(loc) {
            let g = cavern.move_pod(loc, dest_idx);
            let c = cost + add_cost;
            if c >= min_cost {
                continue;
            }
            match cache.get(&g) {
                Some(c2) if c2 > &c => {
                    min_cost = min_cost.min(move_all_pods_rec_cavern(g, &mut cache, c, min_cost));
                }
                None => min_cost = min_cost.min(move_all_pods_rec_cavern(g, &mut cache, c, min_cost)),
                _ => (),
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
    use super::*;

    const TEST_INPUT: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";

    fn prn_cds<const N: usize>(c: &Cavern<N>, l: Loc) {
        let p = c.get(l).unwrap();
        let ds = c.dests(l).collect::<Vec<_>>();
        println!("{} - {:?}", p, ds);
    }

    // #[test]
    // #[ignore]
    // fn test_cavern() {
    //     let cavern = Cavern::from_input(&parse_input(TEST_INPUT));
    //     println!("{}", cavern);
    //
    //     prn_cds(&cavern, Loc::R { col: 1, row: 0 });
    //     let cavern = cavern.move_pod(Loc::R { col: 1, row: 0 }, Loc::H(3));
    //     println!("{}", cavern);
    //
    //     assert!(false);
    // }

    #[test]
    fn test_finished_cavern() {
        use Pod::*;
        let c = Cavern::from_input(&[[A, B, C, D], [A, B, C, D]]);
        println!("{}", c);
        assert!(c.is_finished());
    }

    #[test]
    #[ignore]
    fn test_stuff() {
        use Loc::*;
        let parsed = parse_input(TEST_INPUT);
        let cavern = Cavern::from_input(&parsed);
        println!("{}", cavern);

        let cavern = cavern.move_pod(R { col: 2, row: 0 }, H(0));
        println!("{}", cavern);
        prn_cds(&cavern, H(0));
        prn_cds(&cavern, R { col: 1, row: 0 });

        assert!(false);
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
