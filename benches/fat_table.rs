mod helpers;

use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use helpers::fs_setup::BenchFs;

fn bench_fat_chain_traversal(c: &mut Criterion) {
    let bench_fs = BenchFs::new();
    let mut group = c.benchmark_group("fat_chain_traversal");
    let data = vec![0xAAu8; 256 * 1024]; // 256KB file spans many clusters

    group.bench_function("uncached", |b| {
        let mut vfat = bench_fs.open_vfat();
        let fname = "fat-chain.bin".to_string();
        {
            let mut root = vfat.get_root().unwrap();
            let mut file = root.create_file(fname.clone()).unwrap();
            file.write(&data).unwrap();
        }
        b.iter(|| {
            // Reading the entire file traverses the full cluster chain in the FAT
            let entry = vfat
                .get_from_absolute_path(format!("/{}", fname).into())
                .unwrap();
            let mut file = entry.into_file().unwrap();
            let mut buf = vec![0u8; 256 * 1024];
            file.read(&mut buf).unwrap();
        });
    });

    group.bench_function("cached", |b| {
        let mut vfat = bench_fs.open_vfat_cached(64);
        let fname = "fat-chainc.bin".to_string();
        {
            let mut root = vfat.get_root().unwrap();
            let mut file = root.create_file(fname.clone()).unwrap();
            file.write(&data).unwrap();
        }
        b.iter(|| {
            let entry = vfat
                .get_from_absolute_path(format!("/{}", fname).into())
                .unwrap();
            let mut file = entry.into_file().unwrap();
            let mut buf = vec![0u8; 256 * 1024];
            file.read(&mut buf).unwrap();
        });
    });

    group.finish();
}

fn bench_cluster_allocation(c: &mut Criterion) {
    let bench_fs = BenchFs::new_with_size(Some(200));
    let mut group = c.benchmark_group("cluster_allocation");
    group.sample_size(10);
    let data = vec![0xBBu8; 64 * 1024]; // 64KB forces several cluster allocations

    group.bench_function("uncached", |b| {
        let mut vfat = bench_fs.open_vfat();
        let mut counter = 0u32;
        b.iter(|| {
            let name = format!("alloc-{}.bin", counter);
            counter += 1;
            let mut root = vfat.get_root().unwrap();
            let mut file = root.create_file(name).unwrap();
            file.write(&data).unwrap();
        });
    });

    group.bench_function("cached", |b| {
        let mut vfat = bench_fs.open_vfat_cached(64);
        let mut counter = 0u32;
        b.iter(|| {
            let name = format!("allocc-{}.bin", counter);
            counter += 1;
            let mut root = vfat.get_root().unwrap();
            let mut file = root.create_file(name).unwrap();
            file.write(&data).unwrap();
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
    targets = bench_fat_chain_traversal, bench_cluster_allocation
}
criterion_main!(benches);
