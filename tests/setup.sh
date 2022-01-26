#!/usr/bin/env bash
set -e
# FS size in megabytes:
fs_size=260
# Desired size in bytes
size=$((${fs_size}*(1<<20)))
# align to next MB (https://www.thomas-krenn.com/en/wiki/Partition_Alignment)
alignment=$((1<<20))
# ceil(size, 1MB):
size=$(( (size + alignment - 1)/alignment * alignment ))

temp_dir=/tmp/irisos_fat32/
diskimg=fat32.fs

echo "setup.sh: going to create an fs in ${temp_dir}${diskimg}";

# From: https://unix.stackexchange.com/a/527217/61495
# Filename of resulting disk image
mkdir -p $temp_dir
cd $temp_dir

# mkfs.fat requires size as an (undefined) block-count; seem to be units of 1k
mkfs.fat -C -F32 -n "IRISVOL" "${diskimg}".fat $((size >> 10))

# insert the filesystem to a new file at offset 1MB
dd if=${diskimg}.fat of=${diskimg} conv=sparse obs=512 seek=$((${alignment}/512))

# extend the file by 1MB
truncate -s "+${alignment}" "${diskimg}"

# apply partitioning
parted -s --align optimal "${diskimg}"\
  mklabel msdos\
  mkpart primary fat32 1MiB 100%\
  set 1 boot on

# Cleanup unneeded fat section
rm -fv ${temp_dir}fat32.fs.fat

#####################
######### Write test files
# 1. Mount the FS:
dest="/tmp/testmount/"
mkdir -p $dest
sudo mount -o loop,offset=$((2048*512)),uid=1000,gid=1000,dmask=0000,fmask=0001 fat32.fs $dest

# Create test files:
cd ${dest}

mkdir -p ${dest}folder/some/deep/nested/folder/
touch ${dest}folder/some/deep/nested/folder/file
mkdir ${dest}MyFoLdEr

cat > ${dest}a-big-file.txt <<EOF
From fairest creatures we desire increase,
That thereby beauty's rose might never die,
But as the riper should by time decrease,
His tender heir mught bear his memeory:
But thou, contracted to thine own bright eyes,
Feed'st thy light'st flame with self-substantial fuel,
Making a famine where abundance lies,
Thyself thy foe, to thy sweet self too cruel.
Thou that art now the world's fresh ornament
And only herald to the gaudy spring,
Within thine own bud buriest thy content
And, tender churl, makest waste in niggarding.
Pity the world, or else this glutton be,
To eat the world's due, by the grave and thee.
EOF

touch ${dest}a-very-long-file-name-entry.txt
echo 'Hello, Iris OS!' > ${dest}hello.txt

# exit from the mounted fs:
cd /tmp

## Then unmount the fs, to flush disk writes.
exec sudo umount $dest
