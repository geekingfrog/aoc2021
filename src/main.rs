use aoc2021::day01;
use aoc2021::day02;
use aoc2021::day03;
use aoc2021::day04;
use std::env;

fn main() {
    let arg = env::args()
        .skip(1)
        .next()
        .and_then(|s| usize::from_str_radix(s.as_ref(), 10).ok());

    match arg {
        Some(1) => print_day("day01", day01::solve()),
        Some(2) => print_day("day02", day02::solve()),
        Some(3) => print_day("day03", day03::solve()),
        Some(4) => print_day("day04", day04::solve()),
        Some(x) => {
            eprintln!("Not solved yet for day {}", x);
            std::process::exit(1);
        },
        None => {
            print_day("day01", day01::solve());
            print_day("day02", day02::solve());
            print_day("day03", day03::solve());
            print_day("day04", day04::solve());
        }
    }
}

fn print_day<D1, D2>(tag: &str, results: (D1, D2))
where
    D1: std::fmt::Display,
    D2: std::fmt::Display,
{
    let (r1, r2) = results;
    println!("{} part 1: {}\n{} part 2: {}", tag, r1, tag, r2);
}
