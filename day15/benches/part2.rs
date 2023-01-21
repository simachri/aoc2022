use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day15::calculate_tuning_frequency;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");
    println!("input: {}", input);
    c.bench_function("part 2 - input 4_000_000", |b| {
        b.iter(|| calculate_tuning_frequency(input, black_box(4_000_000)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
