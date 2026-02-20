use rand::RngExt;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn create_random_dir() -> PathBuf {
    let random_dir_name: String = rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
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
        assert!(dir.starts_with("/tmp/"));
        fs::remove_file(self.fs_path.clone()).unwrap();
        fs::remove_dir(dir).unwrap();
    }
}

pub fn setup() -> VfatFsRandomPath {
    setup_with_size(None)
}

pub fn setup_with_size(size_mb: Option<u32>) -> VfatFsRandomPath {
    let mut random_dir_path = create_random_dir();
    if random_dir_path.exists() {
        println!(
            "Ops! Random dir '{:?}' already exists. Trying again.",
            random_dir_path.display()
        );
        return setup_with_size(size_mb);
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
    let mut cmd = Command::new("bash");
    cmd.arg(d.display().to_string().as_str())
        .arg(random_dir_path.display().to_string().as_str());
    if let Some(mb) = size_mb {
        cmd.arg(mb.to_string());
    }
    let _output = cmd.output().expect("failed to execute setup script.");

    random_dir_path.push("fat32.fs");

    VfatFsRandomPath {
        fs_path: random_dir_path,
    }
}
