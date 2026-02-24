mod helpers;

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use helpers::fs_setup::BenchFs;

fn bench_traverse_shallow(c: &mut Criterion) {
    let bench_fs = BenchFs::new();
    let mut group = c.benchmark_group("path_traversal_shallow");

    group.bench_function("uncached", |b| {
        let mut vfat = bench_fs.open_vfat();
        b.iter(|| {
            vfat.get_from_absolute_path("/hello.txt".into()).unwrap();
        });
    });

    group.bench_function("cached", |b| {
        let mut vfat = bench_fs.open_vfat_cached(64);
        b.iter(|| {
            vfat.get_from_absolute_path("/hello.txt".into()).unwrap();
        });
    });

    group.finish();
}

fn bench_traverse_deep(c: &mut Criterion) {
    let bench_fs = BenchFs::new();
    let mut group = c.benchmark_group("path_traversal_deep");

    group.bench_function("uncached", |b| {
        let mut vfat = bench_fs.open_vfat();
        b.iter(|| {
            vfat.get_from_absolute_path("/folder/some/deep/nested/folder/file".into())
                .unwrap();
        });
    });

    group.bench_function("cached", |b| {
        let mut vfat = bench_fs.open_vfat_cached(64);
        b.iter(|| {
            vfat.get_from_absolute_path("/folder/some/deep/nested/folder/file".into())
                .unwrap();
        });
    });

    group.finish();
}

fn bench_config() -> Criterion {
    Criterion::default()
        .measurement_time(Duration::from_secs(2))
        .warm_up_time(Duration::from_secs(1))
}

criterion_group! {
    name = benches;
    config = bench_config();
    targets = bench_traverse_shallow, bench_traverse_deep
}
criterion_main!(benches);
