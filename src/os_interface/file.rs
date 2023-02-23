use binrw::io::{SeekFrom, Write};
use core::fmt::Formatter;
use core::{cmp, fmt};

use log::{debug, info};

use crate::os_interface::Metadata;
use crate::{error, ClusterId, Result, VfatFS};

/// A File representation in a VfatFilesystem.
//#[derive(Clone)]
pub struct File {
    pub(crate) vfat_filesystem: VfatFS,
    pub(crate) metadata: Metadata,
    // Current Seek position
    pub offset: usize,
}
impl fmt::Debug for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VfatFile: metadata: {:?}, offset: {:?}.",
            self.metadata, self.offset
        )
    }
}

impl File {
    pub fn new(vfat_filesystem: VfatFS, metadata: Metadata) -> Self {
        File {
            vfat_filesystem,
            metadata,
            offset: 0,
        }
    }
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn update_file_size(&mut self, amount_written: usize) -> Result<()> {
        if self.offset + amount_written <= self.metadata.size as usize {
            return Ok(());
        }
        info!(
            "Offset: {}, written: {}, old size: {}",
            self.offset, amount_written, self.metadata.size
        );
        self.metadata.size = (self.offset + amount_written) as u32;
        info!("New file size: {}", self.metadata.size);
        info!(
            "I'm going to update file size on the fs... Parent path: {:?}",
            self.metadata.parent()
        );
        self.update_metadata()
    }

    pub fn update_metadata(&mut self) -> Result<()> {
        debug!("Going to update metadata on disk...");
        self.vfat_filesystem
            .get_path(self.metadata.parent().clone())?
            .into_directory()
            .unwrap()
            .update_entry(self.metadata.clone())
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        debug!("Requested write on file.");
        if self.metadata.cluster == ClusterId::new(0) {
            debug!("File's cluster is none.");
            self.metadata.cluster = self.vfat_filesystem.allocate_cluster_new_entry()?;
            debug!(
                "Allocated cluster to file: {}, updating metadata...",
                self.metadata.cluster
            );
            self.update_metadata()?;
            debug!("Updated metadata");
        }
        let mut ccw = self
            .vfat_filesystem
            .cluster_chain_writer(self.metadata.cluster);

        ccw.seek(self.offset)?;

        info!(
            "File: Write: Clusterid: {} amount to write: {}, offset: {}",
            self.metadata.cluster,
            buf.len(),
            self.offset,
        );
        let amount_written = ccw.write(buf)?;
        info!("File: Write: Amount written: {}", amount_written);
        self.update_file_size(amount_written)?;
        self.offset += amount_written;

        Ok(amount_written)
    }

    pub fn flush(&mut self) -> Result<()> {
        // TODO, should flush only data wrt this file..
        self.vfat_filesystem.device.flush()
    }

    pub fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        match pos {
            SeekFrom::Start(val) => {
                self.offset = val as usize;
            }
            SeekFrom::End(val) => {
                if self.metadata.size as i64 + val < 0 {
                    return Err(binrw::io::Error::new(
                        binrw::io::ErrorKind::InvalidInput,
                        "Invalid argument - offset cannot be less then zero.",
                    )
                    .into());
                }
                debug!(
                    "Seek from end, size: {}, movement: {}",
                    self.metadata.size, val
                );
                self.offset = (self.metadata.size as i64 + val) as usize;
            }
            SeekFrom::Current(val) => {
                if self.offset as i64 + val < 0 {
                    return Err(binrw::io::Error::new(
                        binrw::io::ErrorKind::InvalidInput,
                        "Invalid argument - offset cannot be less then zero.",
                    )
                    .into());
                }
                self.offset = (self.offset as i64 + val) as usize
            }
        }
        Ok(self.offset as u64)
    }
    pub fn read(&mut self, mut buf: &mut [u8]) -> Result<usize> {
        // it should read at most the buf size or the missing file data.
        let amount_to_read = cmp::min(buf.len(), self.metadata.size().saturating_sub(self.offset));
        if amount_to_read == 0
            || self.metadata.cluster == ClusterId::new(0)
            || self.offset > self.metadata.size as usize
        {
            info!(
                "Amount to read: {}, cluster: {}, offset: {}, size: {}",
                amount_to_read, self.metadata.cluster, self.offset, self.metadata.size
            );
            return Ok(0);
        }
        let mut ccr = self
            .vfat_filesystem
            .cluster_chain_reader(self.metadata.cluster);
        info!("Going to seek to:{}", self.offset);
        ccr.seek(self.offset)?;

        info!(
            "File: Clusterid: {} amount to read: {}, file size: {}",
            self.metadata.cluster, amount_to_read, self.metadata.size
        );
        buf = &mut buf[..amount_to_read];
        let amount_read = ccr.read(buf)?;
        self.offset += amount_read;
        Ok(amount_read)
    }

    fn _sync(&mut self) -> error::Result<()> {
        unimplemented!()
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> binrw::io::Result<usize> {
        Ok(self.write(buf)?)
    }

    fn flush(&mut self) -> binrw::io::Result<()> {
        Ok(self.flush()?)
    }
}
