pub fn solve() -> (usize, usize) {
    let puzzle = include_str!("../resources/day06.txt");
    (solve1(puzzle), solve2(puzzle))
}

fn solve1(input: &str) -> usize {
    solve_n(input, 80)
}

fn solve2(input: &str) -> usize {
    solve_n(input, 256)
}

fn solve_n(input: &str, limit: usize) -> usize {
    let mut fishes = Fishes::from_input(input);
    for _ in 0..limit {
        fishes.next_gen()
    }
    fishes.states.iter().sum()
}

struct Fishes {
    states: [usize; 9],
}

impl Fishes {
    fn from_input(input: &str) -> Self {
        let mut states = [0; 9];
        for c in input.chars().filter(|c| c.is_numeric()) {
            let n = char::to_digit(c, 10).unwrap() as usize;
            states[n] += 1;
        }
        Self { states }
    }

    fn next_gen(&mut self) {
        let to_spawn = self.states[0];
        for i in 0..8 {
            self.states[i] = self.states[i+1]
        }
        self.states[6] += to_spawn;
        self.states[8] = to_spawn;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST: &str = "3,4,3,1,2";

    #[test]
    fn test_solve1() {
        assert_eq!(5934, solve1(TEST))
    }

    #[test]
    fn test_solve2() {
        assert_eq!(26984457539, solve2(TEST))
    }
}
