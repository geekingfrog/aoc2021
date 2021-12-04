use itertools::Itertools;

pub fn solve() -> (usize, usize) {
    let depths: Vec<usize> = include_str!("../resources/day01.txt")
        .lines()
        .map(|i| i.parse().unwrap())
        .collect();

    let count1 = itertools::zip(&depths, &depths[1..])
        .filter(|(a, b)| b > a)
        .count();

    let it: Vec<usize> = depths
        .into_iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .collect();

    let count2 = itertools::zip(&it, &it[1..]).filter(|(a, b)| b > a).count();

    (count1, count2)
}
