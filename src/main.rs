use std::env;

use aoc2021::day01;
use aoc2021::day02;
use aoc2021::day03;
use aoc2021::day04;
use aoc2021::day05;
use aoc2021::day06;
use aoc2021::day07;
use aoc2021::day08;
use aoc2021::day09;
use aoc2021::day10;
use aoc2021::day11;
use aoc2021::day12;
use aoc2021::day13;
use aoc2021::day14;
use aoc2021::day15;
use aoc2021::day16;
use aoc2021::day17;
use aoc2021::day18;
use aoc2021::day19;
use aoc2021::day20;
use aoc2021::day21;
use aoc2021::day22;
use aoc2021::day23;
// use aoc2021::day24;
// use aoc2021::day25;

fn main() {
    let arg: Option<usize> = env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok());

    match arg {
        Some(1) => print_day("day01", day01::solve()),
        Some(2) => print_day("day02", day02::solve()),
        Some(3) => print_day("day03", day03::solve()),
        Some(4) => print_day("day04", day04::solve()),
        Some(5) => print_day("day05", day05::solve()),
        Some(6) => print_day("day06", day06::solve()),
        Some(7) => print_day("day07", day07::solve()),
        Some(8) => print_day("day08", day08::solve()),
        Some(9) => print_day("day09", day09::solve()),
        Some(10) => print_day("day10", day10::solve()),
        Some(11) => print_day("day11", day11::solve()),
        Some(12) => print_day("day12", day12::solve()),
        Some(13) => print_day("day13", day13::solve()),
        Some(14) => print_day("day14", day14::solve()),
        Some(15) => print_day("day15", day15::solve()),
        Some(16) => print_day("day16", day16::solve()),
        Some(17) => print_day("day17", day17::solve()),
        Some(18) => print_day("day18", day18::solve()),
        Some(19) => print_day("day19", day19::solve()),
        Some(20) => print_day("day20", day20::solve()),
        Some(21) => print_day("day21", day21::solve()),
        Some(22) => print_day("day22", day22::solve()),
        Some(23) => print_day("day23", day23::solve()),
        // Some(24) => print_day("day24", day24::solve()),
        // Some(25) => print_day("day25", day25::solve()),
        Some(x) => {
            eprintln!("Not solved yet for day {}", x);
            std::process::exit(1);
        },
        None => {
            let start = std::time::Instant::now();
            print_day("day01", day01::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day02", day02::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day03", day03::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day04", day04::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day05", day05::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day06", day06::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day07", day07::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day08", day08::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day09", day09::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day10", day10::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day11", day11::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day12", day12::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day13", day13::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day14", day14::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day15", day15::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day16", day16::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day17", day17::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day18", day18::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day19", day19::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day20", day20::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day21", day21::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day22", day22::solve());
            println!("so far: {}", start.elapsed().as_millis());
            print_day("day23", day23::solve());
            // print_day("day24", day24::solve());
            // print_day("day25", day25::solve());
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
