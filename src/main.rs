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
use aoc2021::day24;
use aoc2021::day25;

fn main() {
    let arg = env::args().nth(1).and_then(|s| s.parse::<u8>().ok());

    match arg {
        Some(x) => match run_day(x) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1)
            }
        },
        None => {
            let mut total = std::time::Duration::default();
            for i in 1..=25 {
                let start = std::time::Instant::now();
                run_day(i).unwrap();
                let elapsed = start.elapsed();
                total += elapsed;
                // println!(
                //     r#"{{"day": {}, "total_ms": {}, "delta_ms": {}}},"#,
                //     i,
                //     total.as_millis(),
                //     elapsed.as_millis(),
                // );
                println!(
                    "total so far: {}ms (+{}ms)",
                    total.as_millis(),
                    elapsed.as_millis()
                );
            }
            println!("total time: {}ms", total.as_millis());
        }
    }
}

fn run_day(day: u8) -> Result<(), String> {
    match day {
        1 => print_day("day01", day01::solve()),
        2 => print_day("day02", day02::solve()),
        3 => print_day("day03", day03::solve()),
        4 => print_day("day04", day04::solve()),
        5 => print_day("day05", day05::solve()),
        6 => print_day("day06", day06::solve()),
        7 => print_day("day07", day07::solve()),
        8 => print_day("day08", day08::solve()),
        9 => print_day("day09", day09::solve()),
        10 => print_day("day10", day10::solve()),
        11 => print_day("day11", day11::solve()),
        12 => print_day("day12", day12::solve()),
        13 => print_day("day13", day13::solve()),
        14 => print_day("day14", day14::solve()),
        15 => print_day("day15", day15::solve()),
        16 => print_day("day16", day16::solve()),
        17 => print_day("day17", day17::solve()),
        18 => print_day("day18", day18::solve()),
        19 => print_day("day19", day19::solve()),
        20 => print_day("day20", day20::solve()),
        21 => print_day("day21", day21::solve()),
        22 => print_day("day22", day22::solve()),
        23 => print_day("day23", day23::solve()),
        24 => print_day("day24", day24::solve()),
        25 => print_day("day25", day25::solve()),
        _ => return Err(format!("invalid day {}", day)),
    }
    Ok(())
}

fn print_day<D1, D2>(tag: &str, results: (D1, D2))
where
    D1: std::fmt::Display,
    D2: std::fmt::Display,
{
    let (r1, r2) = results;
    println!("{} part 1: {}\n{} part 2: {}", tag, r1, tag, r2);
}
