use chrono::{DateTime, Datelike, Utc};
use std::fs::OpenOptions;
use vfat_rs::io::{SeekFrom, Write};

use log::info;
use rand::RngExt;

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

fn init_vfat_cached(cache_capacity: usize) -> vfat_rs::Result<(VfatFS, VfatFsRandomPath)> {
    let (dev, master_boot_record, vfatfs_randompath) = init();
    VfatFS::new_with_cache(
        dev,
        master_boot_record.partitions[0].start_sector,
        vfat_rs::TimeManagerNoop::new(),
        cache_capacity,
    )
    .map(|fs| (fs, vfatfs_randompath))
}

/// Returns name and path
fn random_name(prefix: &str) -> (String, String) {
    let mut rng = rand::rng();
    let random_suffix: u32 = rng.random_range(1..999999);
    let name = format!("{}-{}.txt", prefix, random_suffix);
    let path = format!("/{}", name);
    (name, path)
}

#[test]
fn test_read_bios_parameter_block() {
    let (mut dev, master_boot_record, _vfatfs_randompath) = init();

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
    // this file is created by setup script.
    assert!(
        vfat.path_exists("/hello.txt".into())?,
        "File doesn't exists. Please run setup script."
    );

    let mut file = vfat
        .get_from_absolute_path("/hello.txt".into())?
        .into_file()
        .unwrap();
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
    // this file is created by setup script.
    assert!(
        vfat.path_exists("/a-big-file.txt".into())?,
        "File doesn't exists. Please run setup script."
    );
    let mut file = vfat
        .get_from_absolute_path("/a-big-file.txt".into())?
        .into_file()
        .unwrap();

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
    file.seek(SeekFrom::End(-(LONG_FILE.len() as i64 + 1_i64)))
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
    vfat.get_from_absolute_path("/not-found.txt".into())
        .unwrap_err();
    let file = vfat.get_from_absolute_path("/hello.txt".into())?;
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
        .get_from_absolute_path("/folder/some/deep/nested/folder/file".into())
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
    let entry = vfat.get_root()?;
    assert_eq!(
        entry.metadata.full_path().display().to_string(),
        entry.metadata.name()
    );
    assert_eq!(entry.metadata.full_path().display().to_string(), "/");
    info!("Entry:{:?}", entry);
    Ok(())
}

#[test]
fn test_file_write_name_short() -> vfat_rs::Result<()> {
    test_file_write("fl")
}

#[test]
fn test_file_write_name_long() -> vfat_rs::Result<()> {
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
    assert!(
        !vfat.path_exists(file_path.clone().into())?,
        "File already exists. Please delete it."
    );

    // 3. create file
    let mut as_file = root
        .create_file(file_name.clone())
        .expect("Cannote create file");
    assert!(
        vfat.path_exists(file_path.clone().into())?,
        "File already exists. Please delete it."
    );

    // 4. Write CONTENT to file
    const CONTENT: &[u8] = b"Hello, world! This is Vfat\n";
    as_file.write_all(CONTENT).expect("write all");

    let mut as_file = vfat
        .get_from_absolute_path(file_path.as_str().into())?
        .into_file()
        .unwrap();

    println!("File's metadata: {:?}", as_file.metadata());
    assert_eq!(
        as_file.metadata().size(),
        CONTENT.len(),
        "File's metadata size is wrong."
    );

    // 5. Read CONTENT back
    as_file.seek(SeekFrom::Start(0))?;
    let mut buf = [0; CONTENT.len()];
    as_file.read(&mut buf).expect("Read exact");
    info!("Read: {}", String::from_utf8_lossy(&buf));
    assert_eq!(buf, CONTENT, "simple write failed");

    as_file.write(CONTENT).expect("second write");
    // return to 0.
    as_file.seek(SeekFrom::End(-(CONTENT.len() as i64) * 2))?;
    let mut double_buf = [0u8; CONTENT.len() * 2];

    as_file.read(&mut double_buf)?;
    info!("Read: {:?}", String::from_utf8_lossy(&double_buf));
    assert_eq!(CONTENT, &double_buf[..CONTENT.len()], "first half");
    assert_eq!(CONTENT, &double_buf[CONTENT.len()..], "second half");

    root.delete(file_name).expect("delete file");
    // 6. assert file does not exist
    assert!(!vfat.path_exists(file_path.into())?);
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

#[test]
#[ignore]
fn test_big_write_and_read() -> vfat_rs::Result<()> {
    // Write and read back a big file
    // The file size will be ITERATIONS * CONTENT.len()
    const ITERATIONS: usize = 1000;
    println!(
        "Starting big write and read, filesize will be: {}",
        convert(ITERATIONS as f64 * CONTENT.len() as f64)
    );
    let (file_name, file_path) = random_name("big_write");
    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;

    // 2. assert file does not exist
    vfat.path_exists(file_path.as_str().into())
        .expect("File already exists. Please delete it.");

    // 3. create file
    let mut as_file = root
        .create_file(file_name.clone())
        .expect("Cannote create file");

    // 4. Write CONTENT to file
    const CONTENT: &[u8] = b"Hello, world! This is Vfat file system. I'm doing some amount of writes, but they better be with longer buffers.\n";
    for _ in 0..ITERATIONS {
        as_file.write_all(CONTENT).expect("write all");
    }

    let mut as_file = vfat
        .get_from_absolute_path(file_path.as_str().into())?
        .into_file()
        .unwrap();

    println!("File's metadata: {:?}", as_file.metadata());
    assert_eq!(
        as_file.metadata().size(),
        CONTENT.len() * ITERATIONS,
        "File's metadata size is wrong."
    );

    // 5. Read CONTENT back
    as_file.seek(SeekFrom::Start(0))?;
    for i in 0..ITERATIONS {
        let mut buf = [0; CONTENT.len()];
        as_file.read(&mut buf).expect("Read exact");
        assert_eq!(buf, CONTENT, "long file write, read failed {}", i);
    }

    root.delete(file_name).expect("delete file");
    // 6. assert file does not exist
    assert!(!vfat.path_exists(file_path.into())?);
    Ok(())
}

#[test]
fn test_create_directory_long() -> vfat_rs::Result<()> {
    test_create_directory("some-uncommonly-long-folder-name-1234-1234-1234-1234-1234-1234-1234")
}

#[test]
fn test_create_directory_short() -> vfat_rs::Result<()> {
    test_create_directory("fld")
}

fn test_create_directory(prefix: &str) -> vfat_rs::Result<()> {
    let (dir_name, dir_path) = random_name(prefix);
    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;

    // 2. assert file does not exists
    assert!(!vfat.path_exists(dir_path.clone().into())?);

    // 3. create directory
    let mut res = root.create_directory(dir_name.clone())?;

    assert!(vfat.path_exists(dir_path.clone().into())?);

    // create subdirectory
    let sub_dir = "test-subdir";
    res.create_directory(sub_dir.to_string())?;

    let full_path = format!("/{}/{}", dir_name, sub_dir);
    assert!(vfat.path_exists(full_path.clone().into())?);

    // Cleanup:
    res.delete(sub_dir.to_string())?;
    assert!(!vfat.path_exists(full_path.into())?);

    vfat.get_root()?.delete(dir_name.to_string())?;
    assert!(!vfat.path_exists(dir_path.into())?);

    Ok(())
}

#[test]
fn test_file_rename() -> vfat_rs::Result<()> {
    test_rename(false)
}
#[test]
fn test_rename_dir() -> vfat_rs::Result<()> {
    test_rename(true)
}

fn test_rename(is_dir: bool) -> vfat_rs::Result<()> {
    let file_name = "hello_world";
    let used_name_path = "/hello_world";

    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;

    // 2. assert file does not exist
    assert!(
        !vfat.path_exists(used_name_path.into())?,
        "File already exists"
    );

    if is_dir {
        root.create_directory(file_name.into())?;
    } else {
        root.create_file(file_name.into())?;
    }
    assert!(vfat.path_exists(used_name_path.into())?);

    let new_name = random_name("file_rename");

    root.rename(file_name.into(), new_name.1.clone().into())?;

    assert!(vfat.path_exists(new_name.1.into())?);
    assert!(!vfat.path_exists(used_name_path.into())?);

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

#[test]
fn test_move_file_to_subdirectory() -> vfat_rs::Result<()> {
    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;

    let (file_name, file_path) = random_name("mvfile");
    let (dir_name, _dir_path) = random_name("mvdir");

    let mut file = root.create_file(file_name.clone())?;
    file.write_all(b"move test content")?;
    root.create_directory(dir_name.clone())?;

    let dest_path = format!("/{}/{}", dir_name, file_name);
    root.rename(file_name.clone(), dest_path.clone().into())?;

    assert!(!vfat.path_exists(file_path.into())?);
    assert!(vfat.path_exists(dest_path.into())?);
    Ok(())
}

#[test]
fn test_move_file_from_subdirectory_to_root() -> vfat_rs::Result<()> {
    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;

    let (dir_name, _dir_path) = random_name("srcdir");
    let (file_name, _) = random_name("mvback");

    let mut dir = root.create_directory(dir_name.clone())?;
    let mut file = dir.create_file(file_name.clone())?;
    file.write_all(b"content to move back")?;

    let src_path = format!("/{}/{}", dir_name, file_name);
    let dest_path = format!("/{}", file_name);

    assert!(vfat.path_exists(src_path.clone().into())?);
    dir.rename(file_name.clone(), dest_path.clone().into())?;

    assert!(!vfat.path_exists(src_path.into())?);
    assert!(vfat.path_exists(dest_path.into())?);
    Ok(())
}

#[test]
fn test_move_directory_across_directories() -> vfat_rs::Result<()> {
    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;

    let (src_dir_name, _) = random_name("srcd");
    let (dest_dir_name, _) = random_name("dstd");
    let (child_dir_name, _) = random_name("child");

    let mut src_dir = root.create_directory(src_dir_name.clone())?;
    src_dir.create_directory(child_dir_name.clone())?;
    root.create_directory(dest_dir_name.clone())?;

    let src_path = format!("/{}/{}", src_dir_name, child_dir_name);
    let dest_path = format!("/{}/{}", dest_dir_name, child_dir_name);

    src_dir.rename(child_dir_name.clone(), dest_path.clone().into())?;

    assert!(!vfat.path_exists(src_path.into())?);
    assert!(vfat.path_exists(dest_path.into())?);
    Ok(())
}

#[test]
fn test_move_and_rename() -> vfat_rs::Result<()> {
    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;

    let (file_name, file_path) = random_name("orig");
    let (dir_name, _) = random_name("tgtdir");
    let (new_name, _) = random_name("renamed");

    root.create_file(file_name.clone())?;
    root.create_directory(dir_name.clone())?;

    let dest_path = format!("/{}/{}", dir_name, new_name);
    root.rename(file_name.clone(), dest_path.clone().into())?;

    assert!(!vfat.path_exists(file_path.into())?);
    assert!(vfat.path_exists(dest_path.into())?);
    Ok(())
}

#[test]
fn test_circular_move_prevented() -> vfat_rs::Result<()> {
    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;

    let (parent_name, _) = random_name("parent");
    let (child_name, _) = random_name("child");

    let mut parent = root.create_directory(parent_name.clone())?;
    parent.create_directory(child_name.clone())?;

    // Try to move parent into its own child — should fail
    let dest_path = format!("/{}/{}/{}", parent_name, child_name, parent_name);
    let result = root.rename(parent_name.clone(), dest_path.into());
    assert!(result.is_err());

    // Parent should still exist
    assert!(vfat.path_exists(format!("/{}", parent_name).into())?);
    Ok(())
}

#[test]
fn test_move_overwrite_existing() -> vfat_rs::Result<()> {
    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;

    let (dir_name, _) = random_name("ovwdir");
    let (file1_name, _) = random_name("file1");

    let mut dir = root.create_directory(dir_name.clone())?;
    // Create file2 in subdirectory (the one that will be overwritten)
    dir.create_file(file1_name.clone())?;
    // Create file1 in root (the one that will be moved)
    root.create_file(file1_name.clone())?;

    let dest_path = format!("/{}/{}", dir_name, file1_name);
    // Move root/file1 to dir/file1, overwriting the existing one
    root.rename(file1_name.clone(), dest_path.clone().into())?;

    assert!(!vfat.path_exists(format!("/{}", file1_name).into())?);
    assert!(vfat.path_exists(dest_path.into())?);
    Ok(())
}

#[ignore]
#[test]
fn test_disk_full() -> vfat_rs::Result<()> {
    // before running this, update setup.sh to create a smaller vfat fs (say 5 mb).
    println!("Starting stress test");
    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;
    println!("Starting stress test");
    for _ in 0..1000 {
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
        for _ in 0..10000 {
            as_file.write_all(CONTENT).expect("write all");
        }
        let as_file = vfat
            .get_from_absolute_path(file_path.as_str().into())?
            .into_file()
            .unwrap();
        println!("File's metadata: {:?}", as_file.metadata());
    }
    Ok(())
}

#[test]
fn test_cached_write_read_roundtrip() -> vfat_rs::Result<()> {
    let (mut vfat, _f) = init_vfat_cached(64)?;
    let mut root = vfat.get_root()?;

    let (file_name, file_path) = random_name("cache_test");
    let mut file = root.create_file(file_name)?;

    let content = b"cached write test data 1234567890";
    file.write_all(content)?;
    file.flush()?;

    // Re-read from filesystem
    let mut file = vfat
        .get_from_absolute_path(file_path.as_str().into())?
        .into_file()
        .unwrap();
    let mut buf = vec![0u8; content.len()];
    file.read(&mut buf)?;
    assert_eq!(&buf, content);
    Ok(())
}

/// Create a tiny (1MB) FAT32 disk image for tests that need to fill a disk.
fn setup_small_disk() -> (VfatFS, common::VfatFsRandomPath) {
    use std::process::Command;

    let random_dir = common::create_random_dir();
    std::fs::create_dir(&random_dir).expect("create temp dir");

    let dir_str = random_dir.display().to_string();

    // Create a tiny FAT32 image: 1MB FAT partition + MBR wrapper
    let script = format!(
        r#"
        set -e
        cd "{dir}"
        mkfs.fat -C -F32 -n "TESTVOL" fat32.fs.fat 1024
        dd if=fat32.fs.fat of=fat32.fs conv=sparse obs=512 seek=2048
        truncate -s "+1048576" fat32.fs
        parted -s --align optimal fat32.fs mklabel msdos mkpart primary fat32 1MiB 100% set 1 boot on
        rm -f fat32.fs.fat
        "#,
        dir = dir_str
    );
    let output = Command::new("bash")
        .arg("-c")
        .arg(&script)
        .output()
        .expect("failed to create small disk image");
    assert!(
        output.status.success(),
        "Small disk setup failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let fs_path = random_dir.join("fat32.fs");
    let vfatfs_rp = common::VfatFsRandomPath {
        fs_path: fs_path.clone(),
    };

    let dev = FilebackedBlockDevice {
        image: std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&fs_path)
            .unwrap(),
    };
    let mut buf = [0; 512];
    let mut dev2 = FilebackedBlockDevice {
        image: std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&fs_path)
            .unwrap(),
    };
    dev2.read_sector(SectorId(0), &mut buf).unwrap();
    let mbr = MasterBootRecord::from(buf);

    let vfat = VfatFS::new(dev, mbr.partitions[0].start_sector).unwrap();
    (vfat, vfatfs_rp)
}

/// Test that the filesystem handles disk-full conditions gracefully.
/// Uses a tiny 1MB disk so it fills up quickly.
#[test]
fn test_disk_full_graceful() -> vfat_rs::Result<()> {
    let (mut vfat, _f) = setup_small_disk();
    let mut root = vfat.get_root()?;

    let mut disk_full = false;
    let mut files_created = 0;

    // Fill the tiny disk by creating small files until allocation fails
    for i in 0..5000 {
        let name = format!("fill{}.txt", i);
        match root.create_file(name) {
            Ok(mut f) => {
                if let Err(_) = f.write(b"x") {
                    disk_full = true;
                    break;
                }
                files_created += 1;
            }
            Err(_) => {
                disk_full = true;
                break;
            }
        }
    }

    assert!(disk_full, "Expected disk to fill up");
    assert!(files_created > 0, "Should have created at least one file");

    // Verify the filesystem is still functional after disk-full error:

    // 1. Listing root directory works
    let entries = root.contents()?;
    assert!(!entries.is_empty(), "Root should still have entries");

    // 2. A previously created file is readable
    let mut first_file = vfat
        .get_from_absolute_path("/fill0.txt".into())?
        .into_file()
        .unwrap();
    let mut buf = [0u8; 1];
    let n = first_file.read(&mut buf)?;
    assert_eq!(n, 1);
    assert_eq!(buf[0], b'x');

    // 3. Deleting a file works (frees space)
    root.delete("fill0.txt".to_string())?;
    assert!(!vfat.path_exists("/fill0.txt".into())?);

    // 4. After freeing space, creating a new file works again
    let mut recovered = root.create_file("recovered.txt".to_string())?;
    recovered.write(b"ok")?;

    let mut recovered = vfat
        .get_from_absolute_path("/recovered.txt".into())?
        .into_file()
        .unwrap();
    let mut rbuf = [0u8; 2];
    recovered.read(&mut rbuf)?;
    assert_eq!(&rbuf, b"ok");

    Ok(())
}
/// Test that deleted directory entry slots are reused when creating new files.
/// Uses `raw_entry_count()` to verify that the total number of raw slots
/// (including deleted) does NOT grow after a delete+recreate cycle — proving
/// that new entries occupy the deleted slots rather than appending.
#[test]
fn test_deleted_entry_reuse() -> vfat_rs::Result<()> {
    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;

    // Work in a fresh subdirectory so we control all entries
    let dir_name = "reuse_dir".to_string();
    let mut subdir = root.create_directory(dir_name)?;

    // Each short-named file uses 2 raw entry slots (1 LFN + 1 regular).
    // "." and ".." use 2 raw slots total.
    let file_count = 6;
    for i in 0..file_count {
        let name = format!("f{}.txt", i);
        let mut f = subdir.create_file(name)?;
        f.write_all(format!("data{}", i).as_bytes())?;
        f.flush()?;
    }

    // Record raw entry count: pseudo dirs + file entries (LFN + regular each)
    let raw_count_before = subdir.raw_entry_count()?;

    // Delete all files — raw entries become 0xE5 (deleted), count stays same
    for i in 0..file_count {
        subdir.delete(format!("f{}.txt", i))?;
    }
    let raw_after_delete = subdir.raw_entry_count()?;
    assert_eq!(
        raw_after_delete, raw_count_before,
        "Raw count should be unchanged after delete (entries marked 0xE5, not removed)"
    );

    // Recreate the same number of files — should reuse deleted slots
    for i in 0..file_count {
        let name = format!("g{}.txt", i);
        let mut f = subdir.create_file(name)?;
        f.write_all(format!("reused{}", i).as_bytes())?;
        f.flush()?;
    }

    let raw_after_reuse = subdir.raw_entry_count()?;
    assert_eq!(
        raw_after_reuse, raw_count_before,
        "Raw count should NOT grow — deleted slots were reused"
    );

    // Verify all recreated files are readable
    for i in 0..file_count {
        let path: PathBuf = format!("/reuse_dir/g{}.txt", i).into();
        let mut file = vfat.get_from_absolute_path(path)?.into_file().unwrap();
        let expected = format!("reused{}", i);
        let mut buf = vec![0u8; expected.len()];
        file.read(&mut buf)?;
        assert_eq!(buf, expected.as_bytes());
    }

    Ok(())
}
/// Test that concurrent threads can safely operate on the same VfatFS instance.
/// Writers create files while readers list the root directory contents.
#[test]
fn test_concurrent_read_write() -> vfat_rs::Result<()> {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let (mut vfat, _f) = init_vfat()?;

    // Pre-create a file so readers always have something to list
    let mut root = vfat.get_root()?;
    let (seed_name, _) = random_name("seed");
    let mut seed = root.create_file(seed_name)?;
    seed.write_all(b"seed data")?;
    seed.flush()?;

    let num_writers = 3;
    let num_readers = 3;
    let total = num_writers + num_readers;
    let barrier = Arc::new(Barrier::new(total));

    let mut handles = Vec::new();

    // Spawn writer threads — each creates a file
    for i in 0..num_writers {
        let mut vfat_clone = vfat.clone();
        let barrier = barrier.clone();
        handles.push(thread::spawn(move || {
            barrier.wait(); // synchronize start
            let mut root = vfat_clone.get_root().unwrap();
            let name = format!("concurrent_w{}", i);
            let mut file = root.create_file(name).unwrap();
            let data = format!("data from writer {}", i);
            file.write_all(data.as_bytes()).unwrap();
            file.flush().unwrap();
        }));
    }

    // Spawn reader threads — each lists root directory contents
    for _ in 0..num_readers {
        let mut vfat_clone = vfat.clone();
        let barrier = barrier.clone();
        handles.push(thread::spawn(move || {
            barrier.wait(); // synchronize start
            let root = vfat_clone.get_root().unwrap();
            let entries = root.contents().unwrap();
            // Should always find at least the seed file + pseudo dirs
            assert!(entries.len() >= 2, "Expected at least 2 entries");
        }));
    }

    for h in handles {
        h.join().expect("Thread panicked");
    }

    // Verify all writer files exist
    for i in 0..num_writers {
        let name = format!("concurrent_w{}", i);
        let path: PathBuf = format!("/{}", name).into();
        assert!(
            vfat.path_exists(path)?,
            "File {} should exist after concurrent writes",
            name
        );
    }

    Ok(())
}

#[test]
fn test_truncate_file() -> vfat_rs::Result<()> {
    let (mut vfat, _f) = init_vfat()?;
    let mut root = vfat.get_root()?;

    let (name, path) = random_name("trunc");
    let mut file = root.create_file(name)?;

    // Write 1000 bytes
    let data = vec![0xABu8; 1000];
    file.write(&data)?;
    file.flush()?;
    assert_eq!(file.metadata().size(), 1000);

    // Truncate to 500
    file.truncate(500)?;
    assert_eq!(file.metadata().size(), 500);

    // Read back and verify only 500 bytes
    file.seek(SeekFrom::Start(0))?;
    let mut buf = vec![0u8; 1000];
    let read = file.read(&mut buf)?;
    assert_eq!(read, 500);
    assert!(buf[..500].iter().all(|&b| b == 0xAB));

    // Truncate to 0
    file.truncate(0)?;
    assert_eq!(file.metadata().size(), 0);

    // Read should return 0 bytes
    file.seek(SeekFrom::Start(0))?;
    let read = file.read(&mut buf)?;
    assert_eq!(read, 0);

    // Verify file still exists
    assert!(vfat.path_exists(PathBuf::from(path.as_str()))?);

    // Can write again after truncating to 0
    file.write(b"hello")?;
    file.flush()?;
    assert_eq!(file.metadata().size(), 5);

    file.seek(SeekFrom::Start(0))?;
    let mut small_buf = vec![0u8; 10];
    let read = file.read(&mut small_buf)?;
    assert_eq!(read, 5);
    assert_eq!(&small_buf[..5], b"hello");

    Ok(())
}
