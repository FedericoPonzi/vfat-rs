# VFAT / FAT32

A simple VFAT implementation written in rust, and mostly tested against Linux's vfat driver.

It aims to be straightforward to understand and easy to use in a custom kernel. Currently supports reads and writes
but it doesn't support renaming of files/directory and flush().

## no_std

This component was first developed with no_std in mind. `std` is mostly supported behind a feature flag.
Check example/simple.rs for a usage example.

## Run example

To run the example, first create a vfat fs using tests/setup.sh then run the example file using:

```bash
cargo run --example simple --feature std
```

## Testing

To run the setup.sh script, I've added an exception for my user in the sudoers file:

```
fponzi ALL=(ALL) NOPASSWD: /usr/bin/mount,/usr/bin/umount
```

On github actions (CI) it just works, because the user has passwordless sudo.
Then all tests can be run with `cargo test`. Each test in vfat.rs will create and delete a vfat filesystem.

### Utils:

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

### Future improvements.

* Currently, the device mutex is shared behind an ARC reference. Maybe, also having the whole FS behind arc would save
  quite some space when
  returning files and directories. Because they get a copy of the Vfat struct.

### FAQ

* What happens if I have a "File" handle and meanwhile someone deletes this file and
  I try to read from a deleted file?
  This case should be taken care of by the application using this library.

--

## Useful docs:

* https://www.win.tue.nl/~aeb/linux/fs/fat/fat-1.html
* Exfat specification: https://docs.microsoft.com/en-us/windows/win32/fileio/exfat-specification#1-introduction

---

To mount with 777 permission:

```
sudo mount -o loop,offset=$((2048*512)),uid=1000,gid=1000,dmask=0000,fmask=0001 fat32.fs /mnt/test/
```