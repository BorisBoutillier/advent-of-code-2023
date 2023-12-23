use aoc_commons::Part;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_22::*;

fn bench_part2(c: &mut Criterion) {
    c.bench_function("part2", |b| {
        b.iter(|| solver(Part::Part2, black_box(include_str!("../input.txt"))))
    });
}

criterion_group!(benches, bench_part2);
criterion_main!(benches);
