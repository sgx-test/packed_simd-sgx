//! Benchmarks intersection between rays and planes
#![feature(stdsimd)]

use aobench_lib::*;
use criterion::*;
use intersection::Isect;
use aobench_lib::scene::Test;

fn hit_scalar(c: &mut Criterion) {
    let mut scene = Test::default();
    c.bench(
        "scalar",
        Benchmark::new("ao_hit", move |b| {
            b.iter(|| {
                let mut isect = Isect::default();
                let isect = black_box(&mut isect);
                let s = black_box(&mut scene);
                let mut v = ambient_occlusion::scalar(s, isect);
                black_box(&mut v);
            })
        })
        .throughput(Throughput::Elements(1)),
    );
}

fn hit_vector(c: &mut Criterion) {
    let mut scene = Test::default();

    c.bench(
        "vector",
        Benchmark::new("ao_hit", move |b| {
            b.iter(|| {
                let mut isect = Isect::default();
                let isect = black_box(&mut isect);
                let s = black_box(&mut scene);
                let mut v = ambient_occlusion::vector(s, isect);
                black_box(&mut v);
            })
        })
        .throughput(Throughput::Elements(1)),
    );
}

criterion_group!(benches, hit_scalar, hit_vector);
criterion_main!(benches);
