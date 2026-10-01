use std::{fs, io, path::Path};

const DIRECTORIES: [&str; 9] = [
    "bin", "dev", "etc", "proc", "run", "sys", "tmp", "usr", "var",
];

pub fn initialize(path: &Path) -> io::Result<()> {
    for directory in DIRECTORIES {
        fs::create_dir_all(path.join(directory))?;
    }

    Ok(())
}

pub struct Inspection {
    missing_directories: Vec<&'static str>,
}

impl Inspection {
    pub fn is_valid(&self) -> bool {
        self.missing_directories.is_empty()
    }

    pub fn missing_directories(&self) -> &[&'static str] {
        &self.missing_directories
    }
}

pub fn inspect(path: &Path) -> io::Result<Inspection> {
    if !path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "rootfs directory does not exist",
        ));
    }

    let missing_directories = DIRECTORIES
        .into_iter()
        .filter(|directory| !path.join(directory).is_dir())
        .collect();

    Ok(Inspection {
        missing_directories,
    })
}

pub fn proc_mount_point(path: &Path) -> io::Result<std::path::PathBuf> {
    let mount_point = path.join("proc");

    if mount_point.is_dir() {
        return Ok(mount_point);
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!(
            "rootfs proc directory does not exist: {}",
            mount_point.display()
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::{DIRECTORIES, initialize, inspect};
    use std::{
        fs,
        path::{Path, PathBuf},
    };

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new(name: &str) -> Self {
            let path = Path::new("target").join(name);
            if path.exists() {
                fs::remove_dir_all(&path).unwrap();
            }
            Self(path)
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn creates_the_standard_directory_layout() {
        let directory = TestDirectory::new("test-rootfs");

        initialize(&directory.0).unwrap();

        for name in DIRECTORIES {
            assert!(directory.0.join(name).is_dir());
        }
    }

    #[test]
    fn reports_an_initialized_rootfs_as_valid() {
        let directory = TestDirectory::new("test-rootfs-valid");

        initialize(&directory.0).unwrap();

        assert!(inspect(&directory.0).unwrap().is_valid());
    }

    #[test]
    fn reports_missing_directories() {
        let directory = TestDirectory::new("test-rootfs-incomplete");
        fs::create_dir_all(directory.0.join("bin")).unwrap();

        let report = inspect(&directory.0).unwrap();
        assert!(!report.is_valid());
        assert!(report.missing_directories().contains(&"etc"));
        assert!(!report.missing_directories().contains(&"bin"));
    }

    #[test]
    fn fails_to_inspect_a_missing_rootfs() {
        let directory = TestDirectory::new("test-rootfs-missing");

        assert!(inspect(&directory.0).is_err());
    }

    #[test]
    fn returns_the_proc_directory_as_a_mount_point() {
        let directory = TestDirectory::new("test-rootfs-proc");

        initialize(&directory.0).unwrap();

        assert_eq!(
            super::proc_mount_point(&directory.0).unwrap(),
            directory.0.join("proc")
        );
    }
}
