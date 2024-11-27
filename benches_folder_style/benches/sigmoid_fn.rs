use criterion::{criterion_group, criterion_main, Criterion};

use benches_folder_style::{fast_sigmoid, with_atan, with_erf, with_exp, with_fabs, with_sqrt};

fn all(c: &mut Criterion) {
    with_atan(c);
    with_exp(c);
    with_sqrt(c);
    with_erf(c);
    with_fabs(c);
    fast_sigmoid(c);
}

criterion_group!(g, all);
criterion_main!(g);
