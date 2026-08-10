//! Benchmarks for scale operations

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use makepad_d3::scale::{Scale, LinearScale, CategoryScale, ScaleExt, TickOptions};

fn linear_scale_benchmark(c: &mut Criterion) {
    let scale = LinearScale::new()
        .with_domain(0.0, 1000.0)
        .with_range(0.0, 800.0);

    c.bench_function("linear_scale_10000", |b| {
        b.iter(|| {
            for i in 0..10000 {
                black_box(scale.scale(i as f64));
            }
        })
    });

    c.bench_function("linear_invert_10000", |b| {
        b.iter(|| {
            for i in 0..10000 {
                black_box(scale.invert(i as f64 * 0.08));
            }
        })
    });
}

fn linear_ticks_benchmark(c: &mut Criterion) {
    let scale = LinearScale::new()
        .with_domain(0.0, 1000.0)
        .with_range(0.0, 800.0);

    let options = TickOptions::default();

    c.bench_function("linear_ticks", |b| {
        b.iter(|| {
            black_box(scale.ticks(&options));
        })
    });
}

fn category_scale_benchmark(c: &mut Criterion) {
    let labels: Vec<String> = (0..100).map(|i| format!("Label {}", i)).collect();
    let scale = CategoryScale::new()
        .with_labels(labels)
        .with_range(0.0, 1000.0);

    c.bench_function("category_scale_100", |b| {
        b.iter(|| {
            for i in 0..100 {
                black_box(scale.scale(i as f64));
            }
        })
    });
}

criterion_group!(benches, linear_scale_benchmark, linear_ticks_benchmark, category_scale_benchmark);
criterion_main!(benches);
