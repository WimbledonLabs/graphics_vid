use criterion::{black_box, criterion_group, criterion_main, Criterion};
use graphics_vid::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let width = 1920;
    let height = 1080;
    let mut buffer = vec![(0.0,0.0,0.0,0.0); width*height];
    let mut ibuffer = vec![0; width*height];
    c.bench_function("simple line", |b| b.iter(|| {
        let p0 = (black_box(0), black_box(0));
        let p1 = (black_box(1919), black_box(1079));
        wu_line(
            black_box((1.0, 1.0, 1.0, 1.0)),
            p0,
            p1,
            width,
            &mut buffer,
        );
    }));

    c.bench_function("gamma_correction", |b| b.iter(|| {
        gamma_correct_buffer(&buffer, &mut ibuffer);
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
