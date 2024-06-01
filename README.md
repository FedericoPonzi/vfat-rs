# VFAT / FAT32

A simple VFAT implementation written in rust, and mostly tested against Linux's vfat driver.

It aims to be straightforward to understand and easy to use in a custom kernel.

It supports all the basic operations:

* File and directory creation,
* File reading and writing,
* Directory and file deletion.

It needs better support for reusing deleted space. Deletion updates the entry's metadata in the directory and marks it
as deleted,
but it's never garbage collected (this would be done by a defrag tool) nor reused.

## no_std

This component was first developed with `no_std` in mind. `std` is supported behind a feature flag, and it is used
for integration testing.

## Using it in your kernel

The exported apis are in the api module. The OS should provide:

* An implementation for the `TimestampTrait`. This is used for timestamping file creation and update.
* An implementation for the device trait. This is used to interact with the disk.
* Alloc support. It is mostly used to allocate some vec and strings, it might be put behind a feature flag later on.

## Run example

To run the example, first create a vfat fs using the script `tests/setup.sh`. This script:

* creates a vfat fs,
* mounts it,
* writes a bunch of files and directories (using your kernel's driver)
* unmounts it.

This file is also used for running integration tests. It needs sudo access for `mount` and `unmount` commands.

Then you're ready to run the example file using:

```bash
cargo run --example simple --features std
```

## Testing

To run the setup.sh script, I've added an exception for my user in the sudoers file:

```
fponzi ALL=(ALL) NOPASSWD: /usr/bin/mount,/usr/bin/umount
```

On Github actions (CI) it just works because the user has passwordless sudo.

Then all tests can be run with `cargo test`. Each test in vfat.rs will create and delete a vfat filesystem.

### Some vfat related utilities

You can check whether the file contains a valid MBR via `gdisk`:

```bash
$ gdisk -l fat32.fs
```

and you can get info about the filesystem with `fdisk`:

```
$ fdisk -l fat32.fs
```

Some stupid script to flush fs changes:

```
sudo umount /mnt/test
sudo mount -o loop,offset=$((2048*512)) fat32.fs /mnt/test/
ls -l /mnt/test
```

Check the changes:

```shell
sudo dosfsck -w -r -l -v -r /dev/loop13
```

---

## Useful docs:

* https://www.win.tue.nl/~aeb/linux/fs/fat/fat-1.html
* Exfat specification: https://docs.microsoft.com/en-us/windows/win32/fileio/exfat-specification#1-introduction

---

To mount with 777 permission:

```
sudo mount -o loop,offset=$((2048*512)),uid=1000,gid=1000,dmask=0000,fmask=0001 fat32.fs /mnt/test/
```