mod helpers;

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use helpers::fs_setup::BenchFs;

fn bench_dir_create_file(c: &mut Criterion) {
    let bench_fs = BenchFs::new();
    let mut group = c.benchmark_group("dir_create_file");
    group.sample_size(10);

    group.bench_function("uncached", |b| {
        let mut vfat = bench_fs.open_vfat();
        let mut counter = 0u32;
        b.iter(|| {
            let name = format!("cf-{}.txt", counter);
            counter += 1;
            let mut root = vfat.get_root().unwrap();
            root.create_file(name).unwrap();
        });
    });

    group.bench_function("cached", |b| {
        let mut vfat = bench_fs.open_vfat_cached(64);
        let mut counter = 0u32;
        b.iter(|| {
            let name = format!("cfc-{}.txt", counter);
            counter += 1;
            let mut root = vfat.get_root().unwrap();
            root.create_file(name).unwrap();
        });
    });

    group.finish();
}

fn bench_dir_create_directory(c: &mut Criterion) {
    let bench_fs = BenchFs::new();
    let mut group = c.benchmark_group("dir_create_directory");
    group.sample_size(10);

    group.bench_function("uncached", |b| {
        let mut vfat = bench_fs.open_vfat();
        let mut counter = 0u32;
        b.iter(|| {
            let name = format!("cd-{}", counter);
            counter += 1;
            let mut root = vfat.get_root().unwrap();
            root.create_directory(name).unwrap();
        });
    });

    group.bench_function("cached", |b| {
        let mut vfat = bench_fs.open_vfat_cached(64);
        let mut counter = 0u32;
        b.iter(|| {
            let name = format!("cdc-{}", counter);
            counter += 1;
            let mut root = vfat.get_root().unwrap();
            root.create_directory(name).unwrap();
        });
    });

    group.finish();
}

fn bench_dir_list_contents(c: &mut Criterion) {
    let bench_fs = BenchFs::new();
    let mut group = c.benchmark_group("dir_list_contents");

    group.bench_function("uncached", |b| {
        let mut vfat = bench_fs.open_vfat();
        {
            let mut root = vfat.get_root().unwrap();
            for i in 0..20 {
                root.create_file(format!("list-{}.txt", i)).unwrap();
            }
        }
        b.iter(|| {
            let root = vfat.get_root().unwrap();
            root.contents().unwrap();
        });
    });

    group.bench_function("cached", |b| {
        let mut vfat = bench_fs.open_vfat_cached(64);
        {
            let mut root = vfat.get_root().unwrap();
            for i in 0..20 {
                root.create_file(format!("listc-{}.txt", i)).unwrap();
            }
        }
        b.iter(|| {
            let root = vfat.get_root().unwrap();
            root.contents().unwrap();
        });
    });

    group.finish();
}

fn bench_dir_delete_file(c: &mut Criterion) {
    let bench_fs = BenchFs::new();
    let mut group = c.benchmark_group("dir_delete_file");

    group.bench_function("uncached", |b| {
        let mut vfat = bench_fs.open_vfat();
        b.iter(|| {
            let name = "del-reuse.txt".to_string();
            {
                let mut root = vfat.get_root().unwrap();
                root.create_file(name.clone()).unwrap();
            }
            let mut root = vfat.get_root().unwrap();
            root.delete(name).unwrap();
        });
    });

    group.bench_function("cached", |b| {
        let mut vfat = bench_fs.open_vfat_cached(64);
        b.iter(|| {
            let name = "delc-reuse.txt".to_string();
            {
                let mut root = vfat.get_root().unwrap();
                root.create_file(name.clone()).unwrap();
            }
            let mut root = vfat.get_root().unwrap();
            root.delete(name).unwrap();
        });
    });

    group.finish();
}

fn bench_dir_contains(c: &mut Criterion) {
    let bench_fs = BenchFs::new();
    let mut group = c.benchmark_group("dir_contains");

    group.bench_function("uncached", |b| {
        let mut vfat = bench_fs.open_vfat();
        b.iter(|| {
            let root = vfat.get_root().unwrap();
            root.contains("hello.txt").unwrap();
        });
    });

    group.bench_function("cached", |b| {
        let mut vfat = bench_fs.open_vfat_cached(64);
        b.iter(|| {
            let root = vfat.get_root().unwrap();
            root.contains("hello.txt").unwrap();
        });
    });

    group.finish();
}

fn bench_dir_rename(c: &mut Criterion) {
    let bench_fs = BenchFs::new();
    let mut group = c.benchmark_group("dir_rename");

    group.bench_function("uncached", |b| {
        let mut vfat = bench_fs.open_vfat();
        // Create the file once, then rename back and forth
        {
            let mut root = vfat.get_root().unwrap();
            root.create_file("ren-a.txt".to_string()).unwrap();
        }
        let mut forward = true;
        b.iter(|| {
            let mut root = vfat.get_root().unwrap();
            if forward {
                root.rename("ren-a.txt".to_string(), "/ren-b.txt".into())
                    .unwrap();
            } else {
                root.rename("ren-b.txt".to_string(), "/ren-a.txt".into())
                    .unwrap();
            }
            forward = !forward;
        });
    });

    group.bench_function("cached", |b| {
        let mut vfat = bench_fs.open_vfat_cached(64);
        {
            let mut root = vfat.get_root().unwrap();
            root.create_file("renc-a.txt".to_string()).unwrap();
        }
        let mut forward = true;
        b.iter(|| {
            let mut root = vfat.get_root().unwrap();
            if forward {
                root.rename("renc-a.txt".to_string(), "/renc-b.txt".into())
                    .unwrap();
            } else {
                root.rename("renc-b.txt".to_string(), "/renc-a.txt".into())
                    .unwrap();
            }
            forward = !forward;
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
    targets = bench_dir_create_file,
        bench_dir_create_directory,
        bench_dir_list_contents,
        bench_dir_delete_file,
        bench_dir_contains,
        bench_dir_rename
}
criterion_main!(benches);
