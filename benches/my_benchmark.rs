use criterion::{black_box, criterion_group, criterion_main, Criterion};
use graphics_vid::wu_line;

pub fn criterion_benchmark(c: &mut Criterion) {
    let width = 1920;
    let height = 1080;
    let mut buffer = vec![0; 1920*1080];
    c.bench_function("simple line", |b| b.iter(|| {
        let p0 = (black_box(0), black_box(0));
        let p1 = (black_box(1919), black_box(1079));
        wu_line(
            black_box(0xffff_ffff),
            p0,
            p1,
            width,
            height,
            &mut buffer,
        );
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
