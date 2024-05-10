use chrono::{DateTime, Datelike, Utc};
use std::fs::OpenOptions;
use vfat_rs::io::{SeekFrom, Write};

use log::info;
use rand::Rng;
use serial_test::serial;

use crate::common::VfatFsRandomPath;
use block_devs::FilebackedBlockDevice;
use vfat_rs::mbr::MasterBootRecord;
use vfat_rs::{mbr, BlockDevice, PathBuf, SectorId, VfatFS};

mod block_devs;
mod common;
/*
   Vfat's integration tests. Why the serial annotation? Because each test is creating a new instance
   of VFAT, so they are not synchronized underneath (something that should not happen in the kernel were
   one is supposed to have one instance per device). Because wrapping the VFAT instance into a mutex
   would end up to just have them running in serial, I preferred to just go ahead and use `serial_test` crate.
*/
fn init() -> (FilebackedBlockDevice, MasterBootRecord, VfatFsRandomPath) {
    // If this is set to debug, for stress tests this produces a lot of logs that can cause OOM kill.
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

/// Returns name and path
fn random_name(prefix: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let random_suffix: u32 = rng.gen_range(1..999999);
    let name = format!("{}-{}.txt", prefix, random_suffix);
    let path = format!("/{}", name);
    (name, path)
}

#[test]
fn test_read_bios_parameter_block() {
    let (mut dev, master_boot_record, vfatfs_randompath) = init();

    assert_eq!(
        master_boot_record.valid_bootsector_sign,
        mbr::VALID_BOOTSECTOR_SIGN
    );

    let partition = master_boot_record.get_vfat_partition(0).unwrap();
    let fullbpb = VfatFS::read_fullebpb(&mut dev, partition.start_sector).unwrap();
    assert_eq!(
        String::from_utf8_lossy(fullbpb.extended.volume_label_string.as_ref()).trim(),
        "IRISVOL".to_string()
    );
}

#[test]
fn test_read_file() -> vfat_rs::Result<()> {
    let (mut vfat, _f) = init_vfat()?;
    let expected_content = "Hello, Iris OS!".to_string();
    let mut file = vfat.get_path("/hello.txt".into())?.into_file().unwrap();
    let mut buf = [0; 512];
    file.read(&mut buf)?;
    assert_eq!(
        String::from_utf8_lossy(&buf[..expected_content.len()]),
        expected_content
    );

    const LONG_FILE: &[u8] = b"From fairest creatures we desire increase,
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
";

    let mut file = vfat
        .get_path("/a-big-file.txt".into())?
        .into_file()
        .unwrap();
    info!(
        "Big file found!, size: {}, file size: {}",
        LONG_FILE.len(),
        file.metadata().size()
    );

    let mut buf = [0; LONG_FILE.len()];
    file.read(&mut buf)?;
    assert_eq!(LONG_FILE, &buf);

    const FIRST_LINE: &[u8] = b"From fairest creatures we desire increase,";
    let mut buf = [0u8; FIRST_LINE.len()];
    file.seek(SeekFrom::Start(0))?;

    file.read(&mut buf)?;
    assert_eq!(FIRST_LINE, &buf);

    const LAST_LINE: &[u8] = b"To eat the world's due, by the grave and thee.\n";
    let mut buf = [0u8; LAST_LINE.len()];
    file.seek(SeekFrom::End(-(LAST_LINE.len() as i64)))?;
    info!("Position: {}", file.offset);
    file.read(&mut buf)?;
    assert_eq!(LAST_LINE, &buf);

    const SECOND_CHAR: &[u8] = b"r";
    const THIRD_CHAR: &[u8] = b"o";

    let mut buf = [0u8; 1];
    file.seek(SeekFrom::Start(1))?;
    file.read(&mut buf)?;
    assert_eq!(buf, SECOND_CHAR);
    file.seek(SeekFrom::Start(2))?;
    file.read(&mut buf)?;
    assert_eq!(buf, THIRD_CHAR);

    file.seek(SeekFrom::Start(0))?;
    // seek to a position < 0
    file.seek(SeekFrom::Current(-1)).unwrap_err();
    // Seek to 0:
    file.seek(SeekFrom::End(-(LONG_FILE.len() as i64)))?;
    // seek to -1:
    file.seek(SeekFrom::End(-(LONG_FILE.len() as i64 + 1 as i64)))
        .unwrap_err();

    Ok(())
}

#[test]
fn test_path() {
    init();
    let expected = "//folder/something";
    let path = PathBuf::from("/folder/something");

    #[cfg(feature = "std")]
    let path_str = path
        .iter()
        .map(|el| el.to_str().unwrap())
        .collect::<Vec<&str>>()
        .join("/");

    #[cfg(not(feature = "std"))]
    let path_str = path.iter().collect::<Vec<&str>>().join("/");
    assert_eq!(expected, path_str);
}

#[test]
fn test_get_path() -> vfat_rs::Result<()> {
    use vfat_rs::VfatMetadataTrait;

    let (mut vfat, _f) = init_vfat()?;
    vfat.get_path("/not-found.txt".into()).unwrap_err();
    let file = vfat.get_path("/hello.txt".into()).unwrap();
    let local: DateTime<Utc> = Utc::now();

    // these are add by the local os's vfat implementation
    // during vfat fs setup
    assert_eq!(file.creation().year(), local.year() as u32);
    assert_eq!(file.creation().month(), local.month());
    assert_eq!(file.creation().day(), local.day());
    assert!(file.creation().hour() <= 23);
    assert!(file.creation().minute() <= 60);
    assert!(file.creation().second() <= 60);
    info!("Hello txt found!");
    assert!(vfat
        .get_path("/folder/some/deep/nested/folder/file".into())
        .is_ok());
    Ok(())
}
#[test]
fn test_list_directory() -> vfat_rs::Result<()> {
    use vfat_rs::VfatMetadataTrait;

    let (mut vfat, _f) = init_vfat()?;
    assert_eq!(
        vfat.get_root()?
            .contents()?
            .into_iter()
            .map(|entry| entry.name().to_string())
            .collect::<Vec<String>>(),
        vec![
            "IRISVOL",
            "folder",
            "MyFoLdEr",
            "a-big-file.txt",
            "a-very-long-file-name-entry.txt",
            "hello.txt"
        ]
        .into_iter()
        .map(Into::into)
        .collect::<Vec<String>>()
    );

    Ok(())
}

#[test]
fn test_get_root() -> vfat_rs::Result<()> {
    let (mut vfat, _f) = init_vfat()?;
    let entry = vfat.get_root().unwrap();
    assert_eq!(
        entry.metadata.full_path().display().to_string(),
        entry.metadata.name()
    );
    assert_eq!(entry.metadata.full_path().display().to_string(), "/");
    info!("Entry:{:?}", entry);
    Ok(())
}

#[test]
fn test_write_side_short() -> vfat_rs::Result<()> {
    test_file_write("fl")
}

#[test]
fn test_file_write_long() -> vfat_rs::Result<()> {
    test_file_write("a-very-long-file-name")?;
    test_file_write("a-very-long-file-name-but-one-which-is-very-very-long")
}

#[test]
fn test_file_creation() -> vfat_rs::Result<()> {
    let file_name = "hello_world";
    let used_name_path = "/hello_world";

    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;

    // 2. assert file does not exists
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

    Ok(())
}

#[test]
fn test_multiple_file_creation() -> vfat_rs::Result<()> {
    // test entry creation that needs multiple clusters allocated to this directory

    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;

    let mut files = (0..200)
        .map(|_| random_name("test_multiple_file_creation"))
        .collect::<Vec<(String, String)>>();
    files.sort();
    files.dedup();

    for (file_name, file_path) in files.clone() {
        root.create_file(file_name).expect("Cannote create file");
        assert!(vfat.path_exists(file_path.into())?);
    }

    // let's also cleanup:
    for (file_name, file_path) in files {
        root.delete(file_name).expect("Cannote delete file");
        assert!(!vfat.path_exists(file_path.into())?);
    }

    Ok(())
}

fn test_file_write(name: &str) -> vfat_rs::Result<()> {
    let (file_name, file_path) = random_name(name);
    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;

    // 2. assert file does not exists
    vfat.path_exists(file_path.as_str().into())
        .expect("File already exists. Please delete it.");

    // 3. create file
    let mut as_file = root
        .create_file(file_name.clone())
        .expect("Cannote create file");

    // 4. Write CONTENT to file
    const CONTENT: &[u8] = b"Hello, world! This is Vfat\n";
    as_file.write_all(CONTENT).expect("write all");

    let mut as_file = vfat
        .get_path(file_path.as_str().into())
        .unwrap()
        .into_file()
        .unwrap();

    println!("File's metadata: {:?}", as_file.metadata());
    assert_eq!(
        as_file.metadata().size(),
        CONTENT.len(),
        "File's metadata size is wrong."
    );

    // 5. Read CONTENT back
    as_file.seek(SeekFrom::Start(0)).unwrap();
    let mut buf = [0; CONTENT.len()];
    as_file.read(&mut buf).expect("Read exact");
    info!("Read: {}", String::from_utf8_lossy(&buf));
    assert_eq!(buf, CONTENT, "simple write failed");

    as_file.write(CONTENT).expect("second write");
    // return to 0.
    as_file
        .seek(SeekFrom::End(-(CONTENT.len() as i64) * 2))
        .unwrap();
    let mut double_buf = [0u8; CONTENT.len() * 2];

    as_file.read(&mut double_buf).unwrap();
    info!("Read: {:?}", String::from_utf8_lossy(&double_buf));
    assert_eq!(CONTENT, &double_buf[..CONTENT.len()], "first half");
    assert_eq!(CONTENT, &double_buf[CONTENT.len()..], "second half");

    root.delete(file_name).expect("delete file");
    // 6. assert file does not exists
    let _file = vfat.get_path(file_path.as_str().into()).unwrap_err();
    Ok(())
}

pub fn convert(num: f64) -> String {
    use std::cmp;
    let negative = if num.is_sign_positive() { "" } else { "-" };
    let num = num.abs();
    let units = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    if num < 1_f64 {
        return format!("{}{} {}", negative, num, "B");
    }
    let delimiter = 1000_f64;
    let exponent = cmp::min(
        (num.ln() / delimiter.ln()).floor() as i32,
        (units.len() - 1) as i32,
    );
    let pretty_bytes = format!("{:.2}", num / delimiter.powi(exponent))
        .parse::<f64>()
        .unwrap()
        * 1_f64;
    let unit = units[exponent as usize];
    format!("{}{} {}", negative, pretty_bytes, unit)
}

#[ignore]
#[test]
fn test_big_write_and_read() -> vfat_rs::Result<()> {
    // Write and read back a big file
    // The file size will be ITERATIONS * CONTENT.len()
    const ITERATIONS: usize = 4000;
    println!(
        "Starting big write and read, filesize will be: {}",
        convert(ITERATIONS as f64 * CONTENT.len() as f64)
    );
    let (file_name, file_path) = random_name("big_write");
    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;

    // 2. assert file does not exists
    vfat.path_exists(file_path.as_str().into())
        .expect("File already exists. Please delete it.");

    // 3. create file
    let mut as_file = root
        .create_file(file_name.clone())
        .expect("Cannote create file");

    // 4. Write CONTENT to file
    const CONTENT: &[u8] = b"Hello, world! This is Vfat\n";
    for _ in 0..ITERATIONS {
        as_file.write_all(CONTENT).expect("write all");
    }

    let mut as_file = vfat
        .get_path(file_path.as_str().into())
        .unwrap()
        .into_file()
        .unwrap();

    println!("File's metadata: {:?}", as_file.metadata());
    assert_eq!(
        as_file.metadata().size(),
        CONTENT.len() * ITERATIONS,
        "File's metadata size is wrong."
    );

    // 5. Read CONTENT back
    as_file.seek(SeekFrom::Start(0)).unwrap();
    for i in 0..ITERATIONS {
        let mut buf = [0; CONTENT.len()];
        as_file.read(&mut buf).expect("Read exact");
        assert_eq!(buf, CONTENT, "long file write, read failed {}", i);
    }

    root.delete(file_name).expect("delete file");
    // 6. assert file does not exists
    let _file = vfat.get_path(file_path.as_str().into()).unwrap_err();
    Ok(())
}

#[test]
fn test_create_directory_long() -> vfat_rs::Result<()> {
    test_create_directory("some-uncommonly-long-folder-name")
}

#[test]
fn test_create_directory_short() -> vfat_rs::Result<()> {
    test_create_directory("fld")
}

fn test_create_directory(prefix: &str) -> vfat_rs::Result<()> {
    let (dir_name, dir_path) = random_name(prefix);
    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;

    let err = format!("Directory '{}' already exists. Please delete it.", dir_path);

    // 2. assert file does not exists
    let _file = vfat
        .get_path(dir_path.as_str().into())
        .expect_err(err.as_str());

    // 3. create directory
    let mut res = root.create_directory(dir_name.clone())?;

    let sub_dir = "prova";
    res.create_directory(sub_dir.to_string())?;
    let full_path = format!("/{}/{}", dir_name, sub_dir);
    vfat.get_path(PathBuf::from(full_path))?;

    // Cleanup:
    vfat.get_path(PathBuf::from(dir_path))?
        .into_directory_unchecked()
        .delete(sub_dir.to_string())?;
    vfat.get_root()?.delete(dir_name.to_string())?;
    Ok(())
}

#[test]
fn test_delete_folder_non_empty() -> vfat_rs::Result<()> {
    let (folder_name, _folder_path) = random_name("delfld");
    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;
    let mut folder = root.create_directory(folder_name.clone())?;
    let (subfolder_name, _subfolder_path) = random_name("subfld");
    folder.create_directory(subfolder_name.clone())?;
    // cannot delete folder with some content:
    root.delete(folder_name.to_string()).unwrap_err();

    // deleting subcontent first should allow delete to succeed.
    folder.delete(subfolder_name.clone())?;
    root.delete(folder_name.to_string())?;

    Ok(())
}

#[ignore]
#[test]
fn test_disk_full() -> vfat_rs::Result<()> {
    // before runnning this, update setup.sh to create a smaller vfat fs (say 5 mb).
    println!("Starting stress test");
    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;
    println!("Starting stress test");
    for i in 0..1000 {
        let (file_name, file_path) = random_name("stress_test");
        println!("Creating file: {}", file_name.as_str());
        // 2. assert file does not exists
        vfat.path_exists(file_path.as_str().into())
            .expect("File already exists. Please delete it.");

        // 3. create file
        let mut as_file = root
            .create_file(file_name.clone())
            .expect("Cannote create file");

        // 4. Write CONTENT to file
        const CONTENT: &[u8] = b"Hello, world! This is Vfat\n";
        for i in 0..10000 {
            as_file.write_all(CONTENT).expect("write all");
        }
        let mut as_file = vfat
            .get_path(file_path.as_str().into())
            .unwrap()
            .into_file()
            .unwrap();
        println!("File's metadata: {:?}", as_file.metadata());
    }
    Ok(())
}
