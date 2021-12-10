pub fn solve() -> (usize, usize) {
    let puzzle = parse_puzzle(include_str!("../resources/day10.txt"));
    (solve1(&puzzle), solve2(&puzzle))
}

fn solve1(puzzle: &Vec<Vec<char>>) -> usize {
    puzzle
        .iter()
        .filter_map(|l| match check_line(l) {
            LineStatus::Incomplete(_) => None,
            LineStatus::Corrupted(c) => match c {
                ')' => Some(3),
                ']' => Some(57),
                '}' => Some(1197),
                '>' => Some(25137),
                _ => unreachable!(),
            },
        })
        .sum()
}

fn solve2(puzzle: &Vec<Vec<char>>) -> usize {
    let mut scores: Vec<_> = puzzle
        .iter()
        .filter_map(|l| match check_line(l) {
            LineStatus::Incomplete(s) => Some(s.complete_score()),
            LineStatus::Corrupted(_) => None,
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Stack(Vec<char>);

impl Stack {
    fn push(mut self, c: char) -> Result<Self, LineStatus> {
        match c {
            '(' | '[' | '{' | '<' => {
                self.0.push(c);
                Ok(self)
            }
            ')' | ']' | '}' | '>' => {
                if let Some(c2) = self.0.pop() {
                    if (c == ')' && c2 != '(')
                        || (c == ']' && c2 != '[')
                        || (c == '}' && c2 != '{')
                        || (c == '>' && c2 != '<')
                    {
                        Err(LineStatus::Corrupted(c))
                    } else {
                        Ok(self)
                    }
                } else {
                    Err(LineStatus::Corrupted(c))
                }
            }
            _ => unreachable!("oops {}", c),
        }
    }

    fn complete_score(&self) -> usize {
        self.0.iter().rev().fold(0, |acc, c| match c {
            '(' => acc * 5 + 1,
            '[' => acc * 5 + 2,
            '{' => acc * 5 + 3,
            '<' => acc * 5 + 4,
            _ => unreachable!("oops {}", c),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum LineStatus {
    Incomplete(Stack),
    Corrupted(char),
}

fn check_line(line: &[char]) -> LineStatus {
    match line.iter().try_fold(Stack::default(), |s, &c| s.push(c)) {
        Ok(s) => LineStatus::Incomplete(s),
        Err(ls) => ls,
    }
}

fn parse_puzzle(raw: &str) -> Vec<Vec<char>> {
    raw.split_terminator('\n')
        .map(|l| l.chars().collect())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn test_check_line() {
        let line: Vec<char> = r#"{([(<{}[<>[]}>{[]{[(<()>"#.chars().collect();
        assert_eq!(check_line(&line[..]), LineStatus::Corrupted('}'));
    }

    #[test]
    fn test_solve1() {
        assert_eq!(26397, solve1(&parse_puzzle(TEST_INPUT)));
    }

    #[test]
    fn test_complete() {
        let input: Vec<char> = "[({(<(())[]>[[{[]{<()<>>".chars().collect();
        let stack = input
            .iter()
            .try_fold(Stack::default(), |s, &c| s.push(c))
            .unwrap();
        assert_eq!(288957, stack.complete_score());
    }

    #[test]
    fn test_solve2() {
        assert_eq!(288957, solve2(&parse_puzzle(TEST_INPUT)));
    }
}
