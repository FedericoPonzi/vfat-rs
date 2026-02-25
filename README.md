# VFAT / FAT32

[![CI](https://github.com/FedericoPonzi/vfat-rs/actions/workflows/CI.yml/badge.svg)](https://github.com/FedericoPonzi/vfat-rs/actions/workflows/CI.yml)
[![crates.io](https://img.shields.io/crates/v/vfat-rs.svg)](https://crates.io/crates/vfat-rs)

A simple VFAT implementation written in rust, and mostly tested against Linux's vfat driver.

It aims to be straightforward to understand and easy to use in a custom kernel.

It supports all the basic operations:

* File and directory creation,
* File reading and writing,
* Directory and file deletion,
* Metadata updates,
* Backup FAT writing
* Deleted clusters and directory entry slots are reused.

It needs better support for defragmentation — there is no defragmentation tool to consolidate free space.

## no_std

This component was first developed with `no_std` in mind. `std` is supported behind a feature flag, and it is used
for integration testing.

## Using it in your kernel

You can pool the library from [crates.io](https://crates.io/crates/vfat-rs):
```
cargo add vfat-rs
```

The exported apis are in the api module. The OS should provide:

* An implementation for the `TimeManagerTrait`. This is used for timestamping file creation and update.
* An implementation for the device trait. This is used to interact with the disk.
* `alloc` support — the library relies on the `alloc` crate (but not `std`) for heap-allocated types; like Box, Arc and String.

## Run example

The example runs in userspace. To run the example, first create a vfat fs using the script `tests/setup.sh`. This script:

* creates a vfat fs,
* mounts it,
* writes a bunch of files and directories (using your kernel's driver)
* unmounts it.

This file is also used for running integration tests. Your user needs sudo access for `mount` and `unmount` commands:

```text
fponzi ALL=(ALL) NOPASSWD: /usr/bin/mount,/usr/bin/umount
```

Then, you're ready to run the example file using:

```bash
cargo run --example simple --features std
```

### Benchmarks

CI runs benchmarks on every push to `master` and publishes historical results with trend charts to [the benchmark dashboard](https://gh.fponzi.me/vfat-rs/dev/bench/). If a benchmark regresses by more than 50%, an alert comment is created automatically.

On CI (file-backed block device), typical numbers are: ~378 MB/s cached reads and ~149 MB/s cached writes for 256KB files, with directory operations (list, delete, contains, rename) completing in 3–14 µs. The caching layer provides 2–23x speedups depending on the operation. Actual performance will depend on the `BlockDevice` implementation provided by your OS.

