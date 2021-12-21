use nom::character::complete as character;
use nom::sequence::tuple;
use nom::IResult;
use nom::{bytes::complete::tag, sequence::preceded};

pub fn solve() -> (usize, usize) {
    let puzzle = parse_puzzle(include_str!("../resources/day21.txt"));
    (solve1(puzzle), solve2(puzzle))
}

fn solve1(puzzle: (usize, usize)) -> usize {
    let (mut current_p, mut other_p) = puzzle;
    let mut current_score = 0usize;
    let mut other_score = 0usize;
    let mut d = 0;
    let mut n = 0;

    while other_score < 1000 {
        for _ in [0, 0, 0] {
            d += 1;
            n += 1;
            if d > 100 {
                d = 1;
            }
            current_p += d;
        }
        while current_p > 10 {
            current_p -= 10;
        }
        current_score += current_p;
        let (x, y) = (current_score, current_p);
        current_score = other_score;
        current_p = other_p;
        other_score = x;
        other_p = y;
    }
    n * current_score
}

const PROBAS: [(u8, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn solve2(puzzle: (usize, usize)) -> usize {
    let initial_state = GameState {
        current: Player::One,
        states: [
            PlayerState {
                pos: puzzle.0 as u8,
                score: 0,
            },
            PlayerState {
                pos: puzzle.1 as u8,
                score: 0,
            },
        ],
        occurrences: 1,
    };

    let mut states = vec![initial_state];
    let mut victories = vec![0; 2];

    while let Some(st) = states.pop() {
        let idx = match st.current {
            Player::One => 0,
            Player::Two => 1,
        };
        let ps = &st.states[idx];

        if st.states[0].score >= 21 {
            victories[0] += st.occurrences;
            continue;
        } else if st.states[1].score >= 21 {
            victories[1] += st.occurrences;
            continue;
        }

        for (i, count) in &PROBAS {
            let mut pos = ps.pos + i;
            if pos > 10 {
                pos -= 10;
            };
            let score = ps.score + pos;
            let new_state = PlayerState {
                pos,
                score,
            };

            let mut new_st = GameState{
                current: !st.current,
                states: st.states,
                occurrences: st.occurrences * count,
            };
            new_st.states[idx] = new_state;
            states.push(new_st);
        }
    }

    if victories[0] > victories[1] {
        victories[0]
    } else {
        victories[1]
    }
}

#[derive(Debug, Clone, Copy)]
struct GameState {
    current: Player,
    states: [PlayerState; 2],
    occurrences: usize,
}

#[derive(Debug, Clone, Copy)]
struct PlayerState {
    pos: u8,
    score: u8,
}

#[derive(Debug, Clone, Copy)]
enum Player {
    One,
    Two,
}

impl std::ops::Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}

fn parse_puzzle(raw: &str) -> (usize, usize) {
    let mut ls = raw.lines();
    let p1 = parse_line(ls.next().unwrap());
    let p2 = parse_line(ls.next().unwrap());
    (p1 as _, p2 as _)
}

fn parse_line(raw: &str) -> u8 {
    let r: IResult<&str, u8> = preceded(
        tuple((tag("Player "), character::u8, tag(" starting position: "))),
        character::u8,
    )(raw);
    r.unwrap().1
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "Player 1 starting position: 4
Player 2 starting position: 8
";

    #[test]
    fn test_parse() {
        assert_eq!((4, 8), parse_puzzle(TEST_INPUT))
    }

    #[test]
    fn test_solve1() {
        assert_eq!(739785, solve1(parse_puzzle(TEST_INPUT)));
    }

    #[test]
    #[ignore]  // very slow in debug mode /o\
    fn test_solve2() {
        assert_eq!(444356092776315, solve2(parse_puzzle(TEST_INPUT)));
    }
}
