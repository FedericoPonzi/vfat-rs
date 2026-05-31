//! Command-line entry point for the vfat-rs FUSE driver.
//!
//! Usage:
//!
//! ```text
//! vfat-fuse <image> <mountpoint> [partition_start_sector]
//! ```
//!
//! If `partition_start_sector` is omitted, the driver auto-detects it: it reads
//! the MBR and uses the first FAT32 partition's start sector, falling back to
//! sector `0` for a raw `mkfs.fat` image. Pass an explicit value to override
//! the detection (e.g. `0` for a raw image, `2048` for a 1MiB-aligned
//! partition).
use std::process::ExitCode;

fn main() -> ExitCode {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut args = std::env::args().skip(1);
    let (image, mountpoint) = match (args.next(), args.next()) {
        (Some(image), Some(mountpoint)) => (image, mountpoint),
        _ => {
            eprintln!("usage: vfat-fuse <image> <mountpoint> [partition_start_sector]");
            return ExitCode::FAILURE;
        }
    };
    let partition_start_sector = match args.next() {
        Some(value) => match value.parse::<u32>() {
            Ok(sector) => Some(sector),
            Err(_) => {
                eprintln!("error: partition_start_sector must be a non-negative integer");
                return ExitCode::FAILURE;
            }
        },
        None => None,
    };

    match partition_start_sector {
        Some(sector) => println!("Mounting '{image}' at '{mountpoint}' (start sector {sector})..."),
        None => println!("Mounting '{image}' at '{mountpoint}' (auto-detecting start sector)..."),
    }
    match vfat_fuse::mount(&image, &mountpoint, partition_start_sector) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}
