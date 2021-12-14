use std::collections::BTreeMap;

use itertools::Itertools;
use nom::bytes::complete::{tag, take};
use nom::character::complete as character;
use nom::combinator::{all_consuming, map};
use nom::multi::separated_list0;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;

pub fn solve() -> (usize, usize) {
    let puzzle = parse_puzzle(include_str!("../resources/day14.txt"));
    (solve1(&puzzle), solve2(&puzzle))
}

fn solve1(puzzle: &Puzzle) -> usize {
    run_steps(puzzle, 10)
}

fn solve2(puzzle: &Puzzle) -> usize {
    run_steps(puzzle, 40)
}

fn run_steps(puzzle: &Puzzle, n: usize) -> usize {
    let mut pairs: BTreeMap<(char, char), usize> = puzzle
        .polymer
        .iter()
        .zip(puzzle.polymer.iter().skip(1))
        .fold(BTreeMap::new(), |mut acc, (&x, &y)| {
            *(acc.entry((x, y)).or_insert(0)) += 1;
            acc
        });

    for _ in 0..n {
        let mut new_pairs = BTreeMap::new();
        for (&p, &c) in pairs.iter() {
            if let Some(r) = puzzle.rules.iter().find(|r| r.from == p) {
                *(new_pairs.entry((r.from.0, r.to)).or_insert(0)) += c;
                *(new_pairs.entry((r.to, r.from.1)).or_insert(0)) += c;
            } else {
                *(new_pairs.entry(p).or_insert(0)) += c;
            }
        }
        pairs = new_pairs;
    }

    // when counting the characters from the pairs, only counts the second one
    // this will miss one occurence for the first char, so fix it there.
    let mut count = count_chars(&pairs);
    *(count.get_mut(&puzzle.polymer[0]).unwrap()) += 1;

    match count.into_iter().map(|(_, c)| c).minmax() {
        itertools::MinMaxResult::MinMax(min, max) => max - min,
        _ => unreachable!(),
    }
}

fn count_chars(pairs: &BTreeMap<(char, char), usize>) -> BTreeMap<char, usize> {
    pairs.iter().fold(BTreeMap::new(), |mut acc, ((_, y), c)| {
        *(acc.entry(*y).or_insert(0)) += c;
        acc
    })
}

#[derive(Debug, PartialEq, Eq)]
struct Rule {
    from: (char, char),
    to: char,
}

#[derive(Debug)]
struct Puzzle {
    polymer: Vec<char>,
    rules: Vec<Rule>,
}

fn parse_puzzle(raw: &str) -> Puzzle {
    let (polymer, rules) = all_consuming(separated_pair(
        parse_polymer,
        character::line_ending,
        terminated(parse_rules, character::line_ending),
    ))(raw)
    .unwrap()
    .1;
    Puzzle { polymer, rules }
}

fn parse_polymer(input: &str) -> IResult<&str, Vec<char>> {
    map(
        terminated(character::alpha1, character::line_ending),
        |s: &str| s.chars().collect(),
    )(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list0(
        character::line_ending,
        map(
            separated_pair(take(2usize), tag(" -> "), take(1usize)),
            |(x, y): (&str, &str)| {
                let chrs = x.chars().take(2).collect::<Vec<_>>();
                Rule {
                    from: (chrs[0], chrs[1]),
                    to: y.chars().next().unwrap(),
                }
            },
        ),
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";
    #[test]
    fn test_parse() {
        assert_eq!(parse_polymer("NNCB\n").unwrap().1, vec!['N', 'N', 'C', 'B']);
        let rules = parse_rules("CH -> B\n").unwrap().1;
        assert_eq!(
            rules[0],
            Rule {
                from: ('C', 'H'),
                to: 'B'
            }
        );
        let puzzle = parse_puzzle(TEST_INPUT);
        assert_eq!(puzzle.polymer, vec!['N', 'N', 'C', 'B']);
        assert_eq!(puzzle.rules.len(), 16);
    }

    #[test]
    fn test_solve1() {
        let puzzle = parse_puzzle(TEST_INPUT);
        assert_eq!(1588, solve1(&puzzle));
        let puzzle = parse_puzzle(include_str!("../resources/day14.txt"));
        assert_eq!(3048, solve1(&puzzle));
    }


    #[test]
    fn test_solve2() {
        let puzzle = parse_puzzle(TEST_INPUT);
        assert_eq!(2188189693529, solve2(&puzzle));
    }
}
