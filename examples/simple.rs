use std::fs::OpenOptions;
use std::io::SeekFrom;

use vfat_rs::mbr::MasterBootRecord;
use vfat_rs::{
    BlockDevice, DirectoryEntry, FilebackedBlockDevice, SectorId, VfatFS, VfatMetadataTrait,
};

fn print_contents(contents: vfat_rs::Result<Vec<DirectoryEntry>>) {
    println!(
        "Root directory content: {:?}",
        contents
            .unwrap()
            .into_iter()
            .map(|f| f.name().to_string())
            .collect::<Vec<_>>()
    );
}
fn main() {
    // to enable logging:
    // use env_logger::Env;
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let mut fbd = FilebackedBlockDevice {
        image: OpenOptions::new()
            .read(true)
            .write(true)
            .open("/tmp/irisos_fat32/fat32.fs")
            .unwrap(),
    };

    let mut buf = [0; 512];
    fbd.read_sector(SectorId(0), &mut buf).unwrap();

    // MBR is always located in sector 0 of the disk
    let master_boot_record = MasterBootRecord::from(buf);
    let mut vfat_fs = VfatFS::new(fbd, master_boot_record.partitions[0].start_sector).unwrap();
    let mut root = vfat_fs.get_root().unwrap();
    print_contents(root.contents());
    println!("Creating file 'my-file'");
    let mut file = root.create_file("my-file".to_string()).unwrap();

    print_contents(root.contents());

    println!("Writing some text to the file");
    file.write(b"Hello, world!").unwrap();

    println!("Done, no reading it back");
    file.seek(SeekFrom::Start(0)).unwrap();
    let mut buf = [0u8; 13];
    file.read(&mut buf).unwrap();
    println!("The file contains: '{}'", String::from_utf8_lossy(&buf));

    root.rename("my-file".to_string(), "my-file-renamed".to_string())
        .unwrap();

    println!(
        "The file was renamed. old file exists? {} new file exists? {}",
        root.contains("my-file").unwrap(),
        root.contains("my-file-renamed").unwrap(),
    );
    print_contents(root.contents());
    println!("Deleting now...");
    root.delete("my-file-renamed".to_string()).unwrap();
    println!("File was deleted!");

    print_contents(root.contents());
}
