mod helpers;

use std::time::Duration;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use helpers::fs_setup::BenchFs;
use vfat_rs::io::SeekFrom;

const SMALL_SIZE: usize = 16;
const MEDIUM_SIZE: usize = 4 * 1024;
const LARGE_SIZE: usize = 256 * 1024;

fn bench_file_write(c: &mut Criterion) {
    let bench_fs = BenchFs::new();
    let mut group = c.benchmark_group("file_write");
    group.sample_size(10);

    for &(label, size) in &[
        ("small_16B", SMALL_SIZE),
        ("medium_4KB", MEDIUM_SIZE),
        ("large_256KB", LARGE_SIZE),
    ] {
        let data = vec![0xABu8; size];
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("uncached", label), |b| {
            let mut vfat = bench_fs.open_vfat();
            let mut counter = 0u32;
            b.iter(|| {
                let name = format!("wr-{}-{}.bin", label, counter);
                counter += 1;
                let mut root = vfat.get_root().unwrap();
                let mut file = root.create_file(name).unwrap();
                file.write(&data).unwrap();
            });
        });

        group.bench_function(BenchmarkId::new("cached", label), |b| {
            let mut vfat = bench_fs.open_vfat_cached(64);
            let mut counter = 0u32;
            b.iter(|| {
                let name = format!("wrc-{}-{}.bin", label, counter);
                counter += 1;
                let mut root = vfat.get_root().unwrap();
                let mut file = root.create_file(name).unwrap();
                file.write(&data).unwrap();
            });
        });
    }
    group.finish();
}

fn bench_file_read(c: &mut Criterion) {
    let bench_fs = BenchFs::new();
    let mut group = c.benchmark_group("file_read");

    for &(label, size) in &[
        ("small_16B", SMALL_SIZE),
        ("medium_4KB", MEDIUM_SIZE),
        ("large_256KB", LARGE_SIZE),
    ] {
        let data = vec![0xCDu8; size];
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("uncached", label), |b| {
            let mut vfat = bench_fs.open_vfat();
            let fname = format!("rd-{}.bin", label);
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
                let mut buf = vec![0u8; size];
                file.read(&mut buf).unwrap();
            });
        });

        group.bench_function(BenchmarkId::new("cached", label), |b| {
            let mut vfat = bench_fs.open_vfat_cached(64);
            let fname = format!("rdc-{}.bin", label);
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
                let mut buf = vec![0u8; size];
                file.read(&mut buf).unwrap();
            });
        });
    }
    group.finish();
}

fn bench_file_seek(c: &mut Criterion) {
    let bench_fs = BenchFs::new();
    let mut group = c.benchmark_group("file_seek");
    let size = LARGE_SIZE;
    let data = vec![0xEFu8; size];

    group.bench_function("uncached", |b| {
        let mut vfat = bench_fs.open_vfat();
        let fname = "seek-test.bin".to_string();
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
            file.seek(SeekFrom::Start(size as u64 / 2)).unwrap();
            let mut buf = [0u8; 16];
            file.read(&mut buf).unwrap();
        });
    });

    group.bench_function("cached", |b| {
        let mut vfat = bench_fs.open_vfat_cached(64);
        let fname = "seekc-test.bin".to_string();
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
            file.seek(SeekFrom::Start(size as u64 / 2)).unwrap();
            let mut buf = [0u8; 16];
            file.read(&mut buf).unwrap();
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
    targets = bench_file_write, bench_file_read, bench_file_seek
}
criterion_main!(benches);
