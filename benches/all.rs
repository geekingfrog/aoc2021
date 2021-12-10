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
            })
        })
    });
}

// criterion_group!(benches, separate);
// criterion_group!(benches, all_at_once);
criterion_group!(benches, separate, all_at_once);
criterion_main!(benches);
