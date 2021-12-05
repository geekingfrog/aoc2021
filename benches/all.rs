use aoc2021::day01;
use aoc2021::day02;
use aoc2021::day03;
use aoc2021::day04;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[allow(dead_code)]
fn separate(c: &mut Criterion) {
    c.bench_function("day01", |b| b.iter(|| black_box(day01::solve())));
    c.bench_function("day02", |b| b.iter(|| black_box(day02::solve())));
    c.bench_function("day03", |b| b.iter(|| black_box(day03::solve())));
    c.bench_function("day04", |b| b.iter(|| black_box(day04::solve())));
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
            })
        })
    });
}

criterion_group!(benches, separate);
// criterion_group!(benches, all_at_once);
// criterion_group!(benches, separate, all_at_once);
criterion_main!(benches);
