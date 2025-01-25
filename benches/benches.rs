use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;

pub fn imodulo(num1: i32, num2: i32) -> i32 {
    num1 % num2
}

pub fn fmodulo(num1: f32, num2: f32) -> f32 {
    num1 % num2
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("imodulo", |b| {
        b.iter(|| imodulo(black_box(random::<i32>()), black_box(random::<i32>())))
    });
    c.bench_function("fmodulo", |b| {
        b.iter(|| fmodulo(black_box(random::<f32>()), black_box(random::<f32>())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
