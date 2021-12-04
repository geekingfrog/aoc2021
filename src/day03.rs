#[allow(dead_code)]
const TEST: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

pub fn solve() -> (usize, usize) {
    let input = include_str!("../resources/day03.txt");
    let v0: Vec<(usize, usize)> = input
        .split("\n")
        .next()
        .unwrap()
        .chars()
        .map(|_| (0, 0))
        .collect();
    let k = (1 << v0.len()) - 1;
    let final_v = input.split("\n").filter(|l| !l.is_empty()).fold(v0, f);
    let gamma = to_n(&final_v);
    let epsilon = !gamma & k;
    let result1 = gamma * epsilon;

    let result2 = solve2();
    // println!("day03 part 1: {}\nday03 part 2: {}", result1, result2);
    (result1, result2)
}

fn f(v: Vec<(usize, usize)>, s: &str) -> Vec<(usize, usize)> {
    v.into_iter()
        .zip(s.chars())
        .map(|((zero, one), c)| match c {
            '0' => (zero + 1, one),
            '1' => (zero, one + 1),
            _ => unreachable!(),
        })
        .collect()
}

fn to_n(v: &Vec<(usize, usize)>) -> usize {
    v.iter()
        .map(|(zero, one)| if zero >= one { 0 } else { 1 })
        .fold(0, |acc, d| acc * 2 + d)
}

fn solve2() -> usize {
    let input = include_str!("../resources/day03.txt");
    let nums: Vec<usize> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .collect();

    let k: usize = input.split("\n").next().unwrap().chars().count();
    let masks: Vec<usize> = (0..k).into_iter().map(|i| 1 << (k - i - 1)).collect();

    let o2_cmp = |a, b| a > b;
    let co2_cmp = |a, b| a <= b;

    let o2 = filter_2(&masks, &nums, o2_cmp);
    let co2 = filter_2(&masks, &nums, co2_cmp);
    o2 * co2
}

fn filter_2<F>(masks: &Vec<usize>, nums: &Vec<usize>, f: F) -> usize
where
    F: Fn(usize, usize) -> bool,
{
    let mut candidates = nums.clone();
    for m in masks {
        let (zero, one) = {
            let ones = &candidates.iter().filter(|n| *n & m > 0).count();
            (candidates.len() - ones, *ones)
        };

        candidates = candidates
            .into_iter()
            .filter(|n| {
                if f(zero, one) {
                    ((!*n) & m) > 0
                } else {
                    *n & m > 0
                }
            })
            .collect();
        if candidates.len() == 1 {
            return *candidates.first().unwrap();
        }
    }
    unreachable!();
}
