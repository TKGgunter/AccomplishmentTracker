use at_wasm::levenshtein_distance::{levenshtein_dist_word_array, levenshtein_dist_word_recursive};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
fn bench_levenshtein_array(c: &mut Criterion) {
    let mut g = c.benchmark_group("array");
    for (i, it) in [("123", "abc"), ("cat", "cats"), ("1234325", "asf23")]
        .iter()
        .enumerate()
    {
        g.bench_function(BenchmarkId::from_parameter(i), |b| {
            b.iter(|| levenshtein_dist_word_array(black_box(it.0), black_box(it.1)))
        });
    }
    g.finish();
}

fn bench_levenshtein_recursive(c: &mut Criterion) {
    let mut g = c.benchmark_group("recursive");
    for (i, it) in [("123", "abc"), ("cat", "cats"), ("1234325", "asf23")]
        .iter()
        .enumerate()
    {
        g.bench_function(BenchmarkId::from_parameter(i), |b| {
            b.iter(|| levenshtein_dist_word_recursive(black_box(it.0), black_box(it.1)))
        });
    }
    g.finish();
}
criterion_group!(
    benches,
    bench_levenshtein_array,
    bench_levenshtein_recursive
);
criterion_main!(benches);
