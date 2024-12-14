use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_2024::day14::{part1, part2};

fn bench_part1(c: &mut Criterion) {
    let input = include_str!("../input/2024/day14.txt");
    c.bench_function(&format!("{}:part1", file!()), |b| {
        b.iter(|| part1(black_box(input)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let input = include_str!("../input/2024/day14.txt");
    c.bench_function(&format!("{}:part2", file!()), |b| {
        b.iter(|| part2(black_box(input)))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
