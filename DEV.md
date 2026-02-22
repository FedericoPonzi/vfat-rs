
## Testing

To run the setup.sh script, I've added an exception for my user in the sudoers file:

```text
fponzi ALL=(ALL) NOPASSWD: /usr/bin/mount,/usr/bin/umount
```

On Github actions (CI) it just works because the user has passwordless sudo.

Then all tests can be run with `cargo test`. Each test in vfat.rs will create and delete a vfat filesystem.

## Benchmarks

Benchmarks use [Criterion.rs](https://github.com/bheisler/criterion.rs) and require the `std` feature:

```bash
cargo bench --features std
```

This benchmarks file I/O (read/write/seek), directory operations (create/delete/list/rename), path traversal, and FAT table operations — each comparing cached (64-sector cache) vs. uncached modes.

HTML reports are generated in `target/criterion/`. CI runs benchmarks automatically on pushes to `master` (not on PRs).


### Some vfat related utilities

You can check whether the file contains a valid MBR via `gdisk`:

```bash
$ gdisk -l fat32.fs
```

and you can get info about the filesystem with `fdisk`:

```text
$ fdisk -l fat32.fs
```

Some simple script to flush fs changes:

```bash
sudo umount /mnt/test
sudo mount -o loop,offset=$((2048*512)) /tmp/irisos_fat32/fat32.fs /mnt/test/
ls -l /mnt/test
```

Check the changes:

```shell
# find loop device:
sudo losetup -a | grep fat
/dev/loop51: [66306]:8300622 (/tmp/irisos_fat32/fat32.fs), offset 1048576
# sanity check:
sudo dosfsck -r -l -v -r /dev/loop13
```

To mount with `777` permission:

```bash
sudo mount -o loop,offset=$((2048*512)),uid=1000,gid=1000,dmask=0000,fmask=0001 fat32.fs /mnt/test/
```

## Useful docs:

* https://www.win.tue.nl/~aeb/linux/fs/fat/fat-1.html
* Exfat specification: https://docs.microsoft.com/en-us/windows/win32/fileio/exfat-specification#1-introduction
