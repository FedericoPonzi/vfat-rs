# vfat-fuse-integration

A small **read/write** [FUSE](https://www.kernel.org/doc/html/latest/filesystems/fuse.html)
driver that mounts a FAT32 image handled by [vfat-rs](https://crates.io/crates/vfat-rs)
into the Linux VFS, so you can browse and edit it with ordinary tools (`ls`,
`cat`, `cp`, `mkdir`, `rm`, ...).

It is a userspace integration example for vfat-rs built on top of the
[`fuser`](https://crates.io/crates/fuser) crate. This crate is **not** part of the
main `vfat-rs` workspace; it has its own `Cargo.toml`/`Cargo.lock` and depends on
`vfat-rs` via a local path.

## What it does

The driver maps FUSE inode numbers to absolute paths inside the FAT filesystem
and forwards the following operations to vfat-rs:

* `lookup` / `getattr` — stat files and directories (size, kind, and the
  creation/modification timestamps stored on disk),
* `readdir` — list directory contents,
* `read` — read file contents (with offset support),
* `write` — write file contents (gaps past EOF are zero-filled, since FAT32 has
  no sparse files),
* `create` / `mkdir` — create files and directories,
* `unlink` / `rmdir` — remove files and (empty) directories,
* `rename` — rename/move entries,
* `setattr` — truncate or grow files (`size`); other attribute changes
  (permissions, ownership, explicit timestamps) are accepted but ignored, since
  FAT32 cannot represent them,
* `statfs` — report total/free space (in cluster-sized blocks) so `df` works,
* `open` / `opendir` / `flush` / `fsync` / `release` — stateless, always succeed.

Large sequential copies stay fast: consecutive contiguous `read`/`write` calls
to the same file reuse a cached open handle (and its warm cluster-chain
reader/writer), so a copy is O(size) rather than O(size²).

The image is opened read/write, so the volume is mounted writable. Open the
image read-only at the OS level (or keep a backup) if you want to avoid
modifying it.

FAT32 stores no ownership information, so every entry is reported as owned by
the user that mounted the filesystem (matching the kernel `vfat` driver's
default), and permissions are fixed at `0755` for directories and `0644` for
files.

## Limitations

* **Out of space (`ENOSPC`) is not transactional.** When the volume fills up
  part-way through a write, the call fails with `No space left on device`, but
  the clusters already allocated to that write stay attached to the file (the
  bytes written before the failure are kept, as POSIX permits for a short
  write). The space is *not* leaked permanently: deleting the partially-written
  file (`rm`) reclaims those clusters. So after a failed large copy you may see
  the destination file present at a partial size; remove it to recover the
  space. There is no automatic rollback that frees the partial allocation on
  failure.
* **Explicit timestamps and ownership/permissions are not persisted** — see the
  `setattr` note above; FAT32 has no on-disk representation for them.

## Building

```bash
cargo build
```

`fuser` is configured with its pure-Rust mount backend, so no `libfuse`
development package is required to build.

## Running

You need a FAT32 image. The quickest way to create a raw (non-partitioned) one
is with `mkfs.fat` from `dosfstools`:

```bash
# 64 MiB FAT32 image
mkfs.fat -F 32 -C my.img $((64 * 1024))
```

Then mount it:

```bash
mkdir -p /tmp/mnt
cargo run -- my.img /tmp/mnt
ls -la /tmp/mnt
echo "hello" > /tmp/mnt/greeting.txt   # writes are persisted to the image
fusermount -u /tmp/mnt   # unmount when done
```

> **Note:** the mountpoint stays registered with the kernel until it is
> unmounted. If a run exits abnormally you may see
> `Transport endpoint is not connected`; run `fusermount -u /tmp/mnt`
> (or `sudo umount -l /tmp/mnt`) to clear the stale mount before retrying.

### Usage

```text
vfat-fuse <image> <mountpoint> [partition_start_sector]
```

* `image` — path to the FAT32 image file.
* `mountpoint` — existing empty directory to mount onto.
* `partition_start_sector` — LBA where the FAT volume begins. **Optional**: if
  omitted, the driver auto-detects it by reading the MBR and using the first
  FAT32 partition's start sector, falling back to sector `0` for a raw
  `mkfs.fat` image. Pass an explicit value to override detection (e.g. `0` for a
  raw image, or `2048` for a 1 MiB-aligned first partition such as the one
  produced by the repo's `tests/setup.sh`).

So both of these work without specifying a sector:

```bash
cargo run -- raw-mkfs.img      /tmp/mnt   # raw image -> sector 0
cargo run -- /tmp/irisos_fat32/fat32.fs /tmp/mnt   # partitioned -> sector 2048
```

Mounting requires permission to use FUSE (typically membership of the `fuse`
group or appropriate `user_allow_other` configuration).

## Testing

```bash
cargo test
```

The tests do **not** mount a real FUSE filesystem (that needs privileges and a
running kernel module). Instead they:

1. create a FAT32 image with `mkfs.fat`,
2. populate it through the vfat-rs API,
3. drive the adapter logic directly and assert on the attributes, directory
   listings and file contents that the FUSE layer would hand back to the kernel
   — including the write path (create/write/truncate/mkdir/unlink/rmdir/rename),
   reading data back to confirm it was persisted.

If `mkfs.fat` (package `dosfstools`) is not installed, the tests skip
themselves gracefully.

## CI

`.github/workflows/fuse-ci.yml` builds this crate, checks formatting, runs
Clippy with `-D warnings`, and runs the test suite (installing `dosfstools` so
the image-backed tests actually execute).
