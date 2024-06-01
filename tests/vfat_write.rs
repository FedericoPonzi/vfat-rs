use crate::block_devs::FilebackedBlockDevice;
use crate::common::VfatFsRandomPath;
use rand::Rng;
use std::fs::OpenOptions;
use vfat_rs::mbr::MasterBootRecord;
use vfat_rs::{BlockDevice, SectorId, VfatFS};

mod block_devs;
mod common;
fn init() -> (FilebackedBlockDevice, MasterBootRecord, VfatFsRandomPath) {
    // If this is set to debug for stress tests, this produces a lot of logs that can cause OOM kill.
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    let _ = env_logger::builder().is_test(true).try_init();
    let vfatfs_randompath = common::setup();
    let mut fs = FilebackedBlockDevice {
        image: OpenOptions::new()
            .read(true)
            .write(true)
            .open(&vfatfs_randompath.fs_path)
            .unwrap(),
    };
    let mut buf = [0; 512];
    // MBR is always located in sector 0 of the disk
    fs.read_sector(SectorId(0), &mut buf).unwrap();
    let master_boot_record = MasterBootRecord::from(buf);
    (fs, master_boot_record, vfatfs_randompath)
}

/// VfatFsRandomPath implements the Drop trait - so at the end of the test, it's automatically cleaned up.
fn init_vfat() -> vfat_rs::Result<(VfatFS, VfatFsRandomPath)> {
    let (dev, master_boot_record, vfatfs_randompath) = init();
    //info!("start: {:#?}", master_boot_record);
    VfatFS::new(dev, master_boot_record.partitions[0].start_sector)
        .map(|fs| (fs, vfatfs_randompath))
}

/// Returns "name" and as a path in root folder.
fn random_name(prefix: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let random_suffix: u32 = rng.gen_range(1..999999);
    let name = format!("{}-{}.txt", prefix, random_suffix);
    let path = format!("/{}", name);
    (name, path)
}

#[test]
fn test_file_rename() -> vfat_rs::Result<()> {
    let file_name = "hello_world";
    let used_name_path = "/hello_world";

    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;

    // 2. assert file does not exist
    assert!(
        !vfat.path_exists(used_name_path.into())?,
        "File already exists"
    );

    // 3. create file
    root.create_file(file_name.into())
        .expect("Cannote create file");

    assert!(vfat.path_exists(used_name_path.into())?);

    // 4. try to create another file with the same name should fail.
    root.create_file(file_name.into()).unwrap_err();

    let new_name = random_name("file_rename");
    root.rename(file_name.into(), new_name.0.clone())?;
    assert!(vfat.path_exists(new_name.1.into())?);
    assert!(!vfat.path_exists(used_name_path.into())?);

    Ok(())
}
