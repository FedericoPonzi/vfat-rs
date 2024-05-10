use rand::{random, Rng};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};

const FS_PATH: &str = "/tmp/irisos_fat32/fat32.fs";

pub fn create_random_dir() -> PathBuf {
    let random_dir_name: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    use std::env::temp_dir;
    temp_dir().join(format!("irisos_fat32_{}", random_dir_name))
}
#[derive(Debug)]
pub struct VfatFsRandomPath {
    pub fs_path: PathBuf,
}
impl Drop for VfatFsRandomPath {
    fn drop(&mut self) {
        let dir = self.fs_path.parent().unwrap().to_path_buf();
        assert!(dir.is_dir());
        // Additional safety. It might not be true for all OSs, but it is for Linux so all good
        assert!(dir.starts_with("/tmp/"));
        fs::remove_file(self.fs_path.clone()).unwrap();
        fs::remove_dir(dir).unwrap();
    }
}

pub fn setup() -> VfatFsRandomPath {
    let mut random_dir_path = create_random_dir();
    if random_dir_path.exists() {
        println!(
            "Ops! Random dir '{:?}' already exists. Trying again.",
            random_dir_path.display()
        );
        return setup();
    }
    match fs::create_dir(&random_dir_path) {
        Ok(_) => println!("Random directory created: {:?}", random_dir_path),
        Err(e) => panic!(
            "Error creating random directory '{}': error: {}",
            random_dir_path.display(),
            e
        ),
    }
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/setup.sh");
    let _output = Command::new("bash")
        .arg(d.display().to_string().as_str())
        .arg(random_dir_path.display().to_string().as_str())
        .output()
        .expect("failed to execute setup script.");
    // println!("Setup script output: {:?}", output);

    random_dir_path.push("fat32.fs");

    VfatFsRandomPath {
        fs_path: random_dir_path,
    }
}
