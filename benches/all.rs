use criterion::{black_box, criterion_group, criterion_main, Criterion};

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
// use aoc2021::day19;
use aoc2021::day20;
use aoc2021::day21;
use aoc2021::day22;

#[allow(dead_code)]
fn separate(c: &mut Criterion) {
    c.bench_function("day01", |b| b.iter(|| black_box(day01::solve())));
    c.bench_function("day02", |b| b.iter(|| black_box(day02::solve())));
    c.bench_function("day03", |b| b.iter(|| black_box(day03::solve())));
    c.bench_function("day04", |b| b.iter(|| black_box(day04::solve())));
    c.bench_function("day05", |b| b.iter(|| black_box(day05::solve())));
    c.bench_function("day06", |b| b.iter(|| black_box(day06::solve())));
    c.bench_function("day07", |b| b.iter(|| black_box(day07::solve())));
    c.bench_function("day08", |b| b.iter(|| black_box(day08::solve())));
    c.bench_function("day09", |b| b.iter(|| black_box(day09::solve())));
    c.bench_function("day10", |b| b.iter(|| black_box(day10::solve())));
    c.bench_function("day11", |b| b.iter(|| black_box(day11::solve())));
    c.bench_function("day12", |b| b.iter(|| black_box(day12::solve())));
    c.bench_function("day13", |b| b.iter(|| black_box(day13::solve())));
    c.bench_function("day14", |b| b.iter(|| black_box(day14::solve())));
    c.bench_function("day15", |b| b.iter(|| black_box(day15::solve())));
    c.bench_function("day16", |b| b.iter(|| black_box(day16::solve())));
    c.bench_function("day17", |b| b.iter(|| black_box(day17::solve())));
    c.bench_function("day18", |b| b.iter(|| black_box(day18::solve())));
    // c.bench_function("day19", |b| b.iter(|| black_box(day19::solve())));
    c.bench_function("day20", |b| b.iter(|| black_box(day20::solve())));
    c.bench_function("day21", |b| b.iter(|| black_box(day21::solve())));
    c.bench_function("day22", |b| b.iter(|| black_box(day22::solve())));
}

#[allow(dead_code)]
fn all_at_once(c: &mut Criterion) {
    c.bench_function("all", |b| {
        b.iter(|| {
            black_box({
                day01::solve();
                day02::solve();
                day03::solve();
                day04::solve();
                day05::solve();
                day06::solve();
                day07::solve();
                day08::solve();
                day09::solve();
                day10::solve();
                day11::solve();
                day12::solve();
                day13::solve();
                day14::solve();
                day15::solve();
                day16::solve();
                day17::solve();
                day18::solve();
                // day19::solve();
                day20::solve();
                day21::solve();
                day22::solve();
            })
        })
    });
}

// criterion_group!(benches, separate);
// criterion_group!(benches, all_at_once);
criterion_group!(benches, separate, all_at_once);
criterion_main!(benches);
