pub fn solve() -> (usize, usize) {
    let lines = parse_puzzle(include_str!("../resources/day08.txt"));
    (solve1(&lines), solve2(&lines))
}

fn solve1(lines: &[Line]) -> usize {
    lines
        .iter()
        .map(|l| {
            l.output
                .iter()
                .filter(|w| {
                    w.count_ones() == 2
                        || w.count_ones() == 3
                        || w.count_ones() == 4
                        || w.count_ones() == 7
                })
                .count()
        })
        .sum()
}

fn solve2(lines: &[Line]) -> usize {
    lines.iter().map(decode_entry).sum()
}

fn decode_entry(line: &Line) -> usize {
    let one = *line.input.iter().find(|w| w.count_ones() == 2).unwrap();
    let four = *line.input.iter().find(|w| w.count_ones() == 4).unwrap();
    let seven = *line.input.iter().find(|w| w.count_ones() == 3).unwrap();
    let eight = *line.input.iter().find(|w| w == &&127).unwrap();

    let (zero, six, nine) = {
        let mut zero = None;
        let mut six = None;
        let mut nine = None;
        for d in line.input.iter().filter(|w| w.count_ones() == 6) {
            if d | four == *d {
                nine = Some(*d)
            } else if d | one == eight {
                six = Some(*d)
            } else {
                zero = Some(*d)
            }
        }
        (zero.unwrap(), six.unwrap(), nine.unwrap())
    };

    let (two, three, five) = {
        let mut two = None;
        let mut three = None;
        let mut five = None;
        for d in line.input.iter().filter(|w| w.count_ones() == 5) {
            if d | four == eight {
                two = Some(*d)
            } else if d | seven == *d {
                three = Some(*d)
            } else {
                five = Some(*d)
            }
        }
        (two.unwrap(), three.unwrap(), five.unwrap())
    };

    let digits: Vec<(usize, u8)> = [zero, one, two, three, four, five, six, seven, eight, nine]
        .into_iter()
        .enumerate()
        .collect();

    line.output.iter().fold(0, |r, d| {
        let num = digits.iter().find(|(_idx, n)| n == d).unwrap().0;
        r * 10 + num
    })
}

#[derive(Debug)]
struct Line {
    input: [u8; 10],
    output: [u8; 4],
}

fn parse_puzzle(raw: &str) -> Vec<Line> {
    raw.split_terminator('\n')
        .map(parse_line)
        .collect()
}

fn parse_line(l: &str) -> Line {
    let mut ls = l.split('|');
    let input = ls
        .next()
        .unwrap()
        .split_terminator(' ')
        .map(parse_word)
        .filter(|w| w > &0)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let output = ls
        .next()
        .unwrap()
        .split_terminator(' ')
        .map(parse_word)
        .filter(|w| w > &0)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    Line { input, output }
}

fn parse_word(w: &str) -> u8 {
    let mut r = 0;
    for c in w.chars() {
        match c {
            'a' => r |= 1,
            'b' => r |= 1 << 1,
            'c' => r |= 1 << 2,
            'd' => r |= 1 << 3,
            'e' => r |= 1 << 4,
            'f' => r |= 1 << 5,
            'g' => r |= 1 << 6,
            _ => unreachable!(),
        }
    }
    r
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    #[test]
    fn test_solve1() {
        assert_eq!(26, solve1(&parse_puzzle(TEST_INPUT)))
    }

    #[test]
    fn test_parse_word() {
        assert_eq!(127, parse_word("abcdefg"))
    }

    #[test]
    fn test_parse_line() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        let l = &parse_puzzle(input)[0];
        assert_eq!(l.output[0], 127)
    }

    #[test]
    fn test_decode_entry() {
        let line = parse_line(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        assert_eq!(decode_entry(&line), 5353)
    }

    #[test]
    fn test_solve2() {
        assert_eq!(61229, solve2(&parse_puzzle(TEST_INPUT)))
    }
}
