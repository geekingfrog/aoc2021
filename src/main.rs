use aoc2021::day01;
use aoc2021::day02;
use aoc2021::day03;

fn main() {
    print_day("day01", day01::solve());
    print_day("day02", day02::solve());
    print_day("day03", day03::solve());
}

fn print_day<D1, D2>(tag: &str, results: (D1, D2))
where
    D1: std::fmt::Display,
    D2: std::fmt::Display,
{
    let (r1, r2) = results;
    println!("{} part 1: {}\n{} part 2: {}", tag, r1, tag, r2);
}
