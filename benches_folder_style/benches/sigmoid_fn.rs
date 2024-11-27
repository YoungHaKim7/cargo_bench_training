use criterion::{black_box, criterion_group, criterion_main, Criterion};

use benches_folder_style::{fast_sigmoid, with_atan, with_erf, with_exp, with_fabs, with_sqrt};

fn all(c: &mut Criterion) {
    c.bench_function("atan fn", |b| b.iter(|| with_atan(black_box(20.0))));
    c.bench_function("exp fn", |b| b.iter(|| with_exp(black_box(20.0))));
    c.bench_function("sqrt fn", |b| b.iter(|| with_sqrt(black_box(20.0))));
    c.bench_function("erf fn", |b| b.iter(|| with_erf(black_box(20.0))));
    c.bench_function("fabs fn", |b| b.iter(|| with_fabs(black_box(20.0))));
    c.bench_function("fast sigmoid tan fn", |b| {
        b.iter(|| fast_sigmoid(black_box(20.0)))
    });
}

criterion_group!(g, all);
criterion_main!(g);
