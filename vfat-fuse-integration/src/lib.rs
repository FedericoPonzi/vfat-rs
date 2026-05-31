//! A small read/write userspace FUSE driver that exposes a [vfat-rs] FAT32
//! filesystem as a mountable directory tree.
//!
//! The driver translates FUSE operations (`lookup`, `getattr`, `read`,
//! `readdir`, `write`, `create`, `mkdir`, `unlink`, ...) into calls against
//! [`vfat_rs::VfatFS`]. It keeps an in-memory table mapping FUSE inode numbers
//! to absolute paths inside the FAT filesystem, since vfat-rs is addressed by
//! path rather than by inode.
//!
//! [vfat-rs]: https://crates.io/crates/vfat-rs
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use fuser::{
    Errno, FileAttr, FileHandle, FileType, Filesystem, FopenFlags, Generation, INodeNo,
    KernelConfig, ReplyAttr, ReplyCreate, ReplyData, ReplyDirectory, ReplyEmpty, ReplyEntry,
    ReplyOpen, ReplyStatfs, ReplyWrite, Request,
};
use vfat_rs::io::SeekFrom;
use vfat_rs::{Directory, VfatFS, VfatMetadataTrait, VfatRsError};

/// The inode number of the filesystem root. FUSE reserves `1` for the root.
const ROOT_INODE: u64 = 1;
/// How long the kernel may cache attributes/entries returned by this driver.
const TTL: Duration = Duration::from_secs(1);

/// Bidirectional mapping between FUSE inode numbers and absolute paths.
///
/// vfat-rs identifies entries by absolute path, while FUSE identifies them by
/// inode number. This table assigns a stable inode number to every path the
/// kernel asks about and lets us resolve back and forth.
struct InodeTable {
    by_ino: HashMap<u64, PathBuf>,
    by_path: HashMap<PathBuf, u64>,
    next_ino: u64,
}

impl InodeTable {
    fn new() -> Self {
        let mut by_ino = HashMap::new();
        let mut by_path = HashMap::new();
        by_ino.insert(ROOT_INODE, PathBuf::from("/"));
        by_path.insert(PathBuf::from("/"), ROOT_INODE);
        Self {
            by_ino,
            by_path,
            next_ino: ROOT_INODE + 1,
        }
    }

    /// Return the path for an inode, if known.
    fn path(&self, ino: u64) -> Option<PathBuf> {
        self.by_ino.get(&ino).cloned()
    }

    /// Return the inode for `path`, allocating a new one on first sight.
    fn get_or_insert(&mut self, path: PathBuf) -> u64 {
        if let Some(ino) = self.by_path.get(&path) {
            return *ino;
        }
        let ino = self.next_ino;
        self.next_ino += 1;
        self.by_ino.insert(ino, path.clone());
        self.by_path.insert(path, ino);
        ino
    }

    /// Forget `path` and everything beneath it.
    ///
    /// Called after an entry is deleted or renamed so its inode does not keep
    /// pointing at a path that no longer exists.
    fn remove_subtree(&mut self, path: &Path) {
        let stale: Vec<PathBuf> = self
            .by_path
            .keys()
            .filter(|candidate| candidate.as_path() == path || candidate.starts_with(path))
            .cloned()
            .collect();
        for candidate in stale {
            if let Some(ino) = self.by_path.remove(&candidate) {
                self.by_ino.remove(&ino);
            }
        }
    }
}

/// A regular file kept open across consecutive sequential writes.
///
/// The kernel issues one FUSE `write` per chunk of a large copy. Re-resolving
/// the path and re-opening the file on every chunk would rebuild vfat-rs'
/// `ClusterChainWriter` from the start of the file each time, making a copy
/// quadratic in its size. By holding the open [`vfat_rs::File`] between
/// contiguous writes we keep that writer warm, so each chunk is O(1).
///
/// `next_offset` is the byte offset immediately after the last write, i.e. where
/// the next contiguous write must start to reuse this handle.
struct OpenFile {
    ino: u64,
    file: vfat_rs::File,
    next_offset: u64,
}

/// Mutable state shared across FUSE operations.
///
/// FUSE 0.17 calls operation methods through a shared `&self`, so the mutable
/// filesystem handle and inode table live behind a [`Mutex`] (see
/// [`VfatFuse`]). The methods on this struct contain the actual logic and are
/// exercised directly by the test suite, independently of the FUSE layer.
struct Inner {
    fs: VfatFS,
    inodes: InodeTable,
    /// A single file kept open to serve a run of contiguous sequential writes.
    /// See [`OpenFile`]. Evicted before any operation that could change the
    /// file's cluster chain so a stale warm writer can never be reused.
    open_write: Option<OpenFile>,
    /// A single file kept open to serve a run of contiguous sequential reads.
    /// Mirrors [`Inner::open_write`] for the read path so reading a large file
    /// back stays O(size) instead of O(size^2). Evicted whenever the file could
    /// change underneath it.
    open_read: Option<OpenFile>,
    /// uid/gid reported for every entry. FAT stores no ownership information, so
    /// the driver synthesises it from the user that mounted the filesystem,
    /// matching the behaviour of the kernel `vfat` driver's default.
    uid: u32,
    gid: u32,
}

impl Inner {
    fn new(fs: VfatFS) -> Self {
        // Safe: getuid/getgid never fail and have no preconditions.
        let uid = unsafe { libc::getuid() };
        let gid = unsafe { libc::getgid() };
        Self {
            fs,
            inodes: InodeTable::new(),
            open_write: None,
            open_read: None,
            uid,
            gid,
        }
    }

    /// Drop any cached open write handle, releasing its warm `ClusterChainWriter`.
    ///
    /// Writes are flushed through to the device as they happen (and the file
    /// size is persisted on every write), so simply dropping the handle is safe.
    /// This must be called before any operation that could free or move the
    /// cached file's clusters (truncate, delete, rename) so the next reuse can
    /// never write through a stale chain position.
    fn evict_open_write(&mut self) {
        self.open_write = None;
    }

    /// Drop any cached open read handle (and its warm `ClusterChainReader`).
    fn evict_open_read(&mut self) {
        self.open_read = None;
    }

    /// Drop both cached handles. Called before any operation that can change a
    /// file's identity, size or cluster chain (create, delete, rename, truncate)
    /// and on flush/release.
    fn evict_open_handles(&mut self) {
        self.open_write = None;
        self.open_read = None;
    }

    /// Build a [`FileAttr`] for an entry, registering its inode if needed.
    fn build_attr(
        &mut self,
        path: PathBuf,
        size: u64,
        is_dir: bool,
        mtime: SystemTime,
        crtime: SystemTime,
    ) -> FileAttr {
        let ino = self.inodes.get_or_insert(path);
        let kind = if is_dir {
            FileType::Directory
        } else {
            FileType::RegularFile
        };
        FileAttr {
            ino: INodeNo(ino),
            size,
            blocks: size.div_ceil(512),
            atime: mtime,
            mtime,
            ctime: mtime,
            crtime,
            kind,
            perm: if is_dir { 0o755 } else { 0o644 },
            nlink: if is_dir { 2 } else { 1 },
            uid: self.uid,
            gid: self.gid,
            rdev: 0,
            blksize: vfat_rs::SECTOR_SIZE as u32,
            flags: 0,
        }
    }

    /// Stat the entry at `path`, deriving size, kind and timestamps from the
    /// on-disk metadata.
    fn attr_for_path(&mut self, path: PathBuf) -> Result<FileAttr, VfatRsError> {
        let entry = self.fs.get_from_absolute_path(path.clone())?;
        let meta = entry.metadata();
        let size = meta.size() as u64;
        let mtime = system_time(meta.modified());
        let crtime = system_time(meta.created());
        let is_dir = entry.into_directory().is_some();
        Ok(self.build_attr(path, size, is_dir, mtime, crtime))
    }

    /// Resolve the absolute path of an inode or fail with `ENOENT`.
    fn path_of(&self, ino: u64) -> Result<PathBuf, VfatRsError> {
        self.inodes.path(ino).ok_or(VfatRsError::FileNotFound {
            target: format!("inode {ino}"),
        })
    }

    /// `getattr` implementation: stat the entry behind `ino`.
    fn getattr(&mut self, ino: u64) -> Result<FileAttr, VfatRsError> {
        let path = self.path_of(ino)?;
        self.attr_for_path(path)
    }

    /// `lookup` implementation: stat `name` inside the directory `parent`.
    fn lookup(&mut self, parent: u64, name: &str) -> Result<FileAttr, VfatRsError> {
        let parent_path = self.path_of(parent)?;
        let child_path = parent_path.join(name);
        self.attr_for_path(child_path)
    }

    /// `readdir` implementation: list the children of directory `ino`.
    ///
    /// The returned vector is the full listing including the conventional `.`
    /// and `..` entries; the FUSE layer is responsible for honouring `offset`.
    fn readdir(&mut self, ino: u64) -> Result<Vec<(u64, FileType, String)>, VfatRsError> {
        let path = self.path_of(ino)?;
        let directory = self
            .fs
            .get_from_absolute_path(path.clone())?
            .into_directory_or_not_found()?;

        // `..` of the root is the root itself; otherwise it is the parent path.
        let parent_ino = match path.parent() {
            Some(parent) => self.inodes.get_or_insert(parent.to_path_buf()),
            None => ROOT_INODE,
        };
        let mut out = vec![
            (ino, FileType::Directory, ".".to_string()),
            (parent_ino, FileType::Directory, "..".to_string()),
        ];
        for entry in directory.contents()? {
            let name = entry.name().to_string();
            let is_dir = entry.into_directory().is_some();
            let child_ino = self.inodes.get_or_insert(path.join(&name));
            let kind = if is_dir {
                FileType::Directory
            } else {
                FileType::RegularFile
            };
            out.push((child_ino, kind, name));
        }
        Ok(out)
    }

    /// `read` implementation: read up to `size` bytes from `ino` at `offset`.
    fn read(&mut self, ino: u64, offset: u64, size: u32) -> Result<Vec<u8>, VfatRsError> {
        // A read means we are not in the middle of a sequential write stream.
        self.evict_open_write();

        // Fast path: continue an already-open sequential read.
        if let Some(open) = self.open_read.as_mut() {
            if open.ino == ino && open.next_offset == offset {
                let data = read_up_to(&mut open.file, size as usize)?;
                open.next_offset += data.len() as u64;
                return Ok(data);
            }
        }

        // Slow path: (re)open the file from its path and seek to `offset`.
        self.evict_open_read();
        let path = self.path_of(ino)?;
        let mut file =
            self.fs
                .get_from_absolute_path(path)?
                .into_file()
                .ok_or(VfatRsError::FileNotFound {
                    target: format!("inode {ino}"),
                })?;
        file.seek(SeekFrom::Start(offset))?;
        let data = read_up_to(&mut file, size as usize)?;

        // Keep the handle open so following contiguous reads hit the fast path.
        self.open_read = Some(OpenFile {
            ino,
            file,
            next_offset: offset + data.len() as u64,
        });
        Ok(data)
    }

    /// Resolve the directory behind `ino` or fail.
    fn directory_of(&mut self, ino: u64) -> Result<Directory, VfatRsError> {
        let path = self.path_of(ino)?;
        self.fs
            .get_from_absolute_path(path)?
            .into_directory_or_not_found()
    }

    /// Report whether the child `name` inside directory `parent` is itself a
    /// directory. Fails with `FileNotFound` if the child does not exist.
    fn child_is_dir(&mut self, parent: u64, name: &str) -> Result<bool, VfatRsError> {
        let child = self.path_of(parent)?.join(name);
        Ok(self
            .fs
            .get_from_absolute_path(child)?
            .into_directory()
            .is_some())
    }

    /// `write` implementation: write `data` at `offset` into file `ino`.
    ///
    /// FAT32 has no sparse files, so any gap between the current end of file
    /// and `offset` is zero-filled before the data is written.
    ///
    /// Consecutive contiguous writes to the same inode reuse a cached open file
    /// (and its warm cluster-chain writer) so a large sequential copy stays
    /// O(size) overall instead of O(size^2).
    fn write(&mut self, ino: u64, offset: u64, data: &[u8]) -> Result<u32, VfatRsError> {
        // A write means we are not in the middle of a sequential read stream, and
        // it changes the file, so any cached read handle is now stale.
        self.evict_open_read();

        // Fast path: continue an already-open sequential write.
        if let Some(open) = self.open_write.as_mut() {
            if open.ino == ino && open.next_offset == offset {
                write_all(&mut open.file, data)?;
                open.next_offset += data.len() as u64;
                return Ok(data.len() as u32);
            }
        }

        // Slow path: the write does not continue the cached handle, so drop it
        // and (re)open the target file from its path.
        self.evict_open_write();

        let path = self.path_of(ino)?;
        let mut file = self
            .fs
            .get_from_absolute_path(path)?
            .into_file()
            .ok_or_else(|| not_found(ino))?;

        let current = file.metadata().size() as u64;
        let gap = offset.saturating_sub(current);
        file.seek(SeekFrom::Start(current.min(offset)))?;

        let mut zeros_remaining = gap as usize;
        let zeros = [0u8; 4096];
        while zeros_remaining > 0 {
            let chunk = zeros_remaining.min(zeros.len());
            write_all(&mut file, &zeros[..chunk])?;
            zeros_remaining -= chunk;
        }
        write_all(&mut file, data)?;

        // Keep the handle open so following contiguous writes hit the fast path.
        // After the writes above, the file is positioned at `offset + data.len()`.
        self.open_write = Some(OpenFile {
            ino,
            file,
            next_offset: offset + data.len() as u64,
        });
        Ok(data.len() as u32)
    }

    /// `create` implementation: make an empty regular file `name` in `parent`.
    fn create(&mut self, parent: u64, name: &str) -> Result<FileAttr, VfatRsError> {
        self.evict_open_handles();
        let child = self.path_of(parent)?.join(name);
        self.directory_of(parent)?.create_file(name.to_string())?;
        self.attr_for_path(child)
    }

    /// `mkdir` implementation: make an empty directory `name` in `parent`.
    fn mkdir(&mut self, parent: u64, name: &str) -> Result<FileAttr, VfatRsError> {
        self.evict_open_handles();
        let child = self.path_of(parent)?.join(name);
        self.directory_of(parent)?
            .create_directory(name.to_string())?;
        self.attr_for_path(child)
    }

    /// `unlink`/`rmdir` implementation: delete `name` from directory `parent`.
    fn delete(&mut self, parent: u64, name: &str) -> Result<(), VfatRsError> {
        self.evict_open_handles();
        let child = self.path_of(parent)?.join(name);
        self.directory_of(parent)?.delete(name.to_string())?;
        self.inodes.remove_subtree(&child);
        Ok(())
    }

    /// `rename` implementation: move `name` from `parent` to `newname` in
    /// `newparent`.
    fn rename(
        &mut self,
        parent: u64,
        name: &str,
        newparent: u64,
        newname: &str,
    ) -> Result<(), VfatRsError> {
        self.evict_open_handles();
        let source = self.path_of(parent)?.join(name);
        let destination = self.path_of(newparent)?.join(newname);
        self.directory_of(parent)?
            .rename(name.to_string(), destination)?;
        // The moved entry (and any children) now lives at a new path; forget the
        // old mapping so it is re-resolved lazily.
        self.inodes.remove_subtree(&source);
        Ok(())
    }

    /// `setattr(size)` implementation: grow or shrink file `ino` to `size`.
    ///
    /// vfat-rs' `truncate` only shrinks, so growth is emulated by zero-filling
    /// from the old end of file up to the requested size.
    fn set_size(&mut self, ino: u64, size: u64) -> Result<FileAttr, VfatRsError> {
        self.evict_open_handles();
        let path = self.path_of(ino)?;
        let mut file = self
            .fs
            .get_from_absolute_path(path.clone())?
            .into_file()
            .ok_or_else(|| not_found(ino))?;

        let current = file.metadata().size() as u64;
        if size < current {
            file.truncate(size as u32)?;
        } else if size > current {
            file.seek(SeekFrom::Start(current))?;
            let mut remaining = (size - current) as usize;
            let zeros = [0u8; 4096];
            while remaining > 0 {
                let chunk = remaining.min(zeros.len());
                write_all(&mut file, &zeros[..chunk])?;
                remaining -= chunk;
            }
        }
        drop(file);
        self.attr_for_path(path)
    }

    /// `statfs` implementation: report total/free space in cluster-sized blocks
    /// so `df` and the kernel's free-space accounting work. Returns
    /// `(total_blocks, free_blocks, block_size)`.
    fn statfs(&self) -> Result<(u64, u64, u32), VfatRsError> {
        let bsize = self.fs.bytes_per_cluster();
        let blocks = self.fs.cluster_count() as u64;
        let bfree = self.fs.count_free_clusters()? as u64;
        Ok((blocks, bfree, bsize))
    }
}

/// Read up to `size` bytes from `file` at its current position, looping over
/// short reads until the buffer is full or end of file is reached.
fn read_up_to(file: &mut vfat_rs::File, size: usize) -> Result<Vec<u8>, VfatRsError> {
    let mut buf = vec![0u8; size];
    let mut total = 0;
    while total < buf.len() {
        let read = file.read(&mut buf[total..])?;
        if read == 0 {
            break;
        }
        total += read;
    }
    buf.truncate(total);
    Ok(buf)
}

/// Write the whole of `buf` to `file`, looping over short writes.
fn write_all(file: &mut vfat_rs::File, mut buf: &[u8]) -> Result<(), VfatRsError> {
    while !buf.is_empty() {
        let written = file.write(buf)?;
        if written == 0 {
            return Err(VfatRsError::FilesystemCorrupted {
                reason: "write made no progress",
            });
        }
        buf = &buf[written..];
    }
    Ok(())
}

/// Build a `FileNotFound` error for an inode.
fn not_found(ino: u64) -> VfatRsError {
    VfatRsError::FileNotFound {
        target: format!("inode {ino}"),
    }
}

/// Convert a VFAT on-disk timestamp into a [`SystemTime`].
fn system_time(ts: vfat_rs::VfatTimestamp) -> SystemTime {
    SystemTime::UNIX_EPOCH + Duration::from_secs(ts.to_unix_timestamp())
}

/// Map a vfat-rs error to a FUSE errno for the reply layer.
fn errno_of(err: &VfatRsError) -> Errno {
    match err {
        VfatRsError::FileNotFound { .. } | VfatRsError::EntryNotFound { .. } => Errno::ENOENT,
        VfatRsError::NameAlreadyInUse { .. } => Errno::EEXIST,
        VfatRsError::NonEmptyDirectory { .. } => Errno::ENOTEMPTY,
        VfatRsError::FreeClusterNotFound => Errno::ENOSPC,
        VfatRsError::NameTooLong { .. } => Errno::ENAMETOOLONG,
        _ => Errno::EIO,
    }
}

/// A mountable, read/write FUSE adapter over a [`VfatFS`].
pub struct VfatFuse {
    inner: Mutex<Inner>,
}

impl VfatFuse {
    /// Wrap a [`VfatFS`] handle into a FUSE filesystem.
    pub fn new(fs: VfatFS) -> Self {
        Self {
            inner: Mutex::new(Inner::new(fs)),
        }
    }
}

impl Filesystem for VfatFuse {
    fn init(&mut self, _req: &Request, _config: &mut KernelConfig) -> std::io::Result<()> {
        Ok(())
    }

    fn lookup(&self, _req: &Request, parent: INodeNo, name: &OsStr, reply: ReplyEntry) {
        let name = match name.to_str() {
            Some(name) => name,
            None => return reply.error(Errno::EINVAL),
        };
        let mut inner = self.inner.lock().unwrap();
        match inner.lookup(parent.0, name) {
            Ok(attr) => reply.entry(&TTL, &attr, fuser::Generation(0)),
            Err(err) => reply.error(errno_of(&err)),
        }
    }

    fn getattr(&self, _req: &Request, ino: INodeNo, _fh: Option<FileHandle>, reply: ReplyAttr) {
        let mut inner = self.inner.lock().unwrap();
        match inner.getattr(ino.0) {
            Ok(attr) => reply.attr(&TTL, &attr),
            Err(err) => reply.error(errno_of(&err)),
        }
    }

    fn open(&self, _req: &Request, ino: INodeNo, flags: fuser::OpenFlags, reply: ReplyOpen) {
        // The kernel normally truncates via setattr(size=0), but honour an
        // explicit O_TRUNC here too so a write-open always starts from empty.
        if flags.0 & libc::O_TRUNC != 0 {
            let mut inner = self.inner.lock().unwrap();
            if let Err(err) = inner.set_size(ino.0, 0) {
                return reply.error(errno_of(&err));
            }
        }
        reply.opened(FileHandle(0), FopenFlags::empty());
    }

    fn opendir(&self, _req: &Request, _ino: INodeNo, _flags: fuser::OpenFlags, reply: ReplyOpen) {
        reply.opened(FileHandle(0), FopenFlags::empty());
    }

    fn read(
        &self,
        _req: &Request,
        ino: INodeNo,
        _fh: FileHandle,
        offset: u64,
        size: u32,
        _flags: fuser::OpenFlags,
        _lock_owner: Option<fuser::LockOwner>,
        reply: ReplyData,
    ) {
        let mut inner = self.inner.lock().unwrap();
        match inner.read(ino.0, offset, size) {
            Ok(data) => reply.data(&data),
            Err(err) => reply.error(errno_of(&err)),
        }
    }

    fn readdir(
        &self,
        _req: &Request,
        ino: INodeNo,
        _fh: FileHandle,
        offset: u64,
        mut reply: ReplyDirectory,
    ) {
        let mut inner = self.inner.lock().unwrap();
        let entries = match inner.readdir(ino.0) {
            Ok(entries) => entries,
            Err(err) => return reply.error(errno_of(&err)),
        };
        for (index, (child_ino, kind, name)) in
            entries.into_iter().enumerate().skip(offset as usize)
        {
            // The offset handed back to the kernel is the index of the *next*
            // entry, so a follow-up readdir resumes right after this one.
            if reply.add(INodeNo(child_ino), index as u64 + 1, kind, name) {
                break;
            }
        }
        reply.ok();
    }

    #[allow(clippy::too_many_arguments)]
    fn write(
        &self,
        _req: &Request,
        ino: INodeNo,
        _fh: FileHandle,
        offset: u64,
        data: &[u8],
        _write_flags: fuser::WriteFlags,
        _flags: fuser::OpenFlags,
        _lock_owner: Option<fuser::LockOwner>,
        reply: ReplyWrite,
    ) {
        let mut inner = self.inner.lock().unwrap();
        match inner.write(ino.0, offset, data) {
            Ok(written) => reply.written(written),
            Err(err) => reply.error(errno_of(&err)),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn setattr(
        &self,
        _req: &Request,
        ino: INodeNo,
        _mode: Option<u32>,
        _uid: Option<u32>,
        _gid: Option<u32>,
        size: Option<u64>,
        _atime: Option<fuser::TimeOrNow>,
        _mtime: Option<fuser::TimeOrNow>,
        _ctime: Option<SystemTime>,
        _fh: Option<FileHandle>,
        _crtime: Option<SystemTime>,
        _chgtime: Option<SystemTime>,
        _bkuptime: Option<SystemTime>,
        _flags: Option<fuser::BsdFileFlags>,
        reply: ReplyAttr,
    ) {
        let mut inner = self.inner.lock().unwrap();
        // Only a size change is meaningful on FAT32; permission/owner/timestamp
        // changes are accepted but ignored, so chmod/chown/touch do not fail.
        let result = match size {
            Some(size) if size > u32::MAX as u64 => return reply.error(Errno::EFBIG),
            Some(size) => inner.set_size(ino.0, size),
            None => inner.getattr(ino.0),
        };
        match result {
            Ok(attr) => reply.attr(&TTL, &attr),
            Err(err) => reply.error(errno_of(&err)),
        }
    }

    fn create(
        &self,
        _req: &Request,
        parent: INodeNo,
        name: &OsStr,
        _mode: u32,
        _umask: u32,
        _flags: i32,
        reply: ReplyCreate,
    ) {
        let name = match name.to_str() {
            Some(name) => name,
            None => return reply.error(Errno::EINVAL),
        };
        let mut inner = self.inner.lock().unwrap();
        match inner.create(parent.0, name) {
            Ok(attr) => reply.created(
                &TTL,
                &attr,
                Generation(0),
                FileHandle(0),
                FopenFlags::empty(),
            ),
            Err(err) => reply.error(errno_of(&err)),
        }
    }

    fn mkdir(
        &self,
        _req: &Request,
        parent: INodeNo,
        name: &OsStr,
        _mode: u32,
        _umask: u32,
        reply: ReplyEntry,
    ) {
        let name = match name.to_str() {
            Some(name) => name,
            None => return reply.error(Errno::EINVAL),
        };
        let mut inner = self.inner.lock().unwrap();
        match inner.mkdir(parent.0, name) {
            Ok(attr) => reply.entry(&TTL, &attr, Generation(0)),
            Err(err) => reply.error(errno_of(&err)),
        }
    }

    fn unlink(&self, _req: &Request, parent: INodeNo, name: &OsStr, reply: ReplyEmpty) {
        let name = match name.to_str() {
            Some(name) => name,
            None => return reply.error(Errno::EINVAL),
        };
        let mut inner = self.inner.lock().unwrap();
        match inner.child_is_dir(parent.0, name) {
            Ok(true) => return reply.error(Errno::EISDIR),
            Ok(false) => {}
            Err(err) => return reply.error(errno_of(&err)),
        }
        match inner.delete(parent.0, name) {
            Ok(()) => reply.ok(),
            Err(err) => reply.error(errno_of(&err)),
        }
    }

    fn rmdir(&self, _req: &Request, parent: INodeNo, name: &OsStr, reply: ReplyEmpty) {
        let name = match name.to_str() {
            Some(name) => name,
            None => return reply.error(Errno::EINVAL),
        };
        let mut inner = self.inner.lock().unwrap();
        match inner.child_is_dir(parent.0, name) {
            Ok(true) => {}
            Ok(false) => return reply.error(Errno::ENOTDIR),
            Err(err) => return reply.error(errno_of(&err)),
        }
        match inner.delete(parent.0, name) {
            Ok(()) => reply.ok(),
            Err(err) => reply.error(errno_of(&err)),
        }
    }

    fn rename(
        &self,
        _req: &Request,
        parent: INodeNo,
        name: &OsStr,
        newparent: INodeNo,
        newname: &OsStr,
        _flags: fuser::RenameFlags,
        reply: ReplyEmpty,
    ) {
        let (name, newname) = match (name.to_str(), newname.to_str()) {
            (Some(name), Some(newname)) => (name, newname),
            _ => return reply.error(Errno::EINVAL),
        };
        let mut inner = self.inner.lock().unwrap();
        match inner.rename(parent.0, name, newparent.0, newname) {
            Ok(()) => reply.ok(),
            Err(err) => reply.error(errno_of(&err)),
        }
    }

    fn flush(
        &self,
        _req: &Request,
        _ino: INodeNo,
        _fh: FileHandle,
        _lock_owner: fuser::LockOwner,
        reply: ReplyEmpty,
    ) {
        // Writes are flushed through to the device as they happen; close the
        // cached sequential-write handle so it cannot outlive the open file.
        self.inner.lock().unwrap().evict_open_handles();
        reply.ok();
    }

    fn fsync(
        &self,
        _req: &Request,
        _ino: INodeNo,
        _fh: FileHandle,
        _datasync: bool,
        reply: ReplyEmpty,
    ) {
        reply.ok();
    }

    fn fsyncdir(
        &self,
        _req: &Request,
        _ino: INodeNo,
        _fh: FileHandle,
        _datasync: bool,
        reply: ReplyEmpty,
    ) {
        reply.ok();
    }

    fn statfs(&self, _req: &Request, _ino: INodeNo, reply: ReplyStatfs) {
        let inner = self.inner.lock().unwrap();
        match inner.statfs() {
            Ok((blocks, bfree, bsize)) => {
                // bavail == bfree (no reserved blocks); files/ffree unknown on FAT.
                reply.statfs(blocks, bfree, bfree, 0, 0, bsize, 255, bsize);
            }
            Err(err) => reply.error(errno_of(&err)),
        }
    }

    fn release(
        &self,
        _req: &Request,
        _ino: INodeNo,
        _fh: FileHandle,
        _flags: fuser::OpenFlags,
        _lock_owner: Option<fuser::LockOwner>,
        _flush: bool,
        reply: ReplyEmpty,
    ) {
        // The file is being closed: drop any cached read/write handle so it can
        // never outlive the open file description.
        self.inner.lock().unwrap().evict_open_handles();
        reply.ok();
    }
}
///
/// `partition_start_sector` selects where the FAT volume begins:
///
/// * `Some(sector)` — open the volume at that LBA (use `Some(0)` for a raw,
///   non-partitioned `mkfs.fat` image).
/// * `None` — auto-detect: if `image` has an MBR with a FAT32 partition, use
///   that partition's start sector; otherwise fall back to sector `0`.
///
/// This call blocks until the filesystem is unmounted.
pub fn mount(
    image: impl AsRef<Path>,
    mountpoint: impl AsRef<Path>,
    partition_start_sector: Option<u32>,
) -> std::io::Result<()> {
    use fuser::{Config, MountOption};

    let image = image.as_ref();
    let start_sector = match partition_start_sector {
        Some(sector) => sector,
        None => detect_start_sector(image)?,
    };
    log::info!("Opening vfat volume at start sector {start_sector}");

    let device = open_device(image)?;
    let fs = VfatFS::new(device, start_sector)
        .map_err(|err| std::io::Error::other(format!("failed to open vfat fs: {err}")))?;

    let mut config = Config::default();
    config.mount_options = vec![MountOption::FSName("vfat-rs".to_string())];
    fuser::mount2(VfatFuse::new(fs), mountpoint, &config)
}

/// Open `image` as a read/write block device.
fn open_device(image: &Path) -> std::io::Result<vfat_rs::FilebackedBlockDevice> {
    use std::fs::OpenOptions;
    Ok(vfat_rs::FilebackedBlockDevice {
        image: OpenOptions::new().read(true).write(true).open(image)?,
    })
}

/// Determine where the FAT volume starts inside `image`.
///
/// Reads the MBR at sector 0 and returns the start sector of the first FAT32
/// partition it finds. If the image has no MBR / no FAT32 partition (e.g. a raw
/// `mkfs.fat` image whose boot sector sits at sector 0), returns `0`.
fn detect_start_sector(image: &Path) -> std::io::Result<u32> {
    use vfat_rs::mbr::MasterBootRecord;

    let device = open_device(image)?;
    let mbr = match MasterBootRecord::load(device) {
        Ok(mbr) => mbr,
        // Unreadable MBR: treat the image as a raw volume at sector 0.
        Err(_) => return Ok(0),
    };
    for index in 0..mbr.partitions.len() {
        if let Ok(partition) = mbr.get_vfat_partition(index) {
            log::info!(
                "Detected FAT32 partition {index} starting at sector {}",
                partition.start_sector
            );
            return Ok(partition.start_sector);
        }
    }
    Ok(0)
}

#[cfg(test)]
mod tests;
