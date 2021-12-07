pub fn solve() -> (usize, usize) {
    let puzzle = parse_puzzle(include_str!("../resources/day07.txt"));
    (solve1(&puzzle), solve2(&puzzle))
}

fn solve1(input: &[i32]) -> usize {
    let m = input.iter().max().unwrap();
    (0..=*m)
        .into_iter()
        .map(|pos| {
            let fuel: i32 = input.iter().map(|p| (p - pos).abs()).sum();
            fuel
        })
        .min()
        .unwrap() as _
}

fn solve2(input: &[i32]) -> usize {
    let m = input.iter().max().unwrap();
    (0..=*m)
        .into_iter()
        .map(|pos| {
            let fuel: i32 = input
                .iter()
                .map(|p| {
                    let n = (p - pos).abs();
                    n * (n + 1) / 2
                })
                .sum();
            fuel
        })
        .min()
        .unwrap() as _
}

fn parse_puzzle(input: &str) -> Vec<i32> {
    input
        .split(',')
        .filter_map(|x| {
            let x = x.strip_suffix('\n').unwrap_or(x);
            if x.is_empty() {
                None
            } else {
                Some(x.parse::<i32>().unwrap())
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(&parse_puzzle(TEST)[..]), 37)
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(&parse_puzzle(TEST)[..]), 168)
    }
}
