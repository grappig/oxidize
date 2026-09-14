use std::{
    fs, io,
    path::Path,
};

const DIRECTORIES: [&str; 9] = ["bin", "dev", "etc", "proc", "run", "sys", "tmp", "usr", "var"];

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

#[cfg(test)]
mod tests {
    use super::{initialize, inspect, DIRECTORIES};
    use std::{fs, path::PathBuf};

    #[test]
    fn creates_the_standard_directory_layout() {
        let path = PathBuf::from("target/test-rootfs");
        if path.exists() {
            fs::remove_dir_all(&path).unwrap();
        }

        initialize(&path).unwrap();

        for directory in DIRECTORIES {
            assert!(path.join(directory).is_dir());
        }

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn reports_an_initialized_rootfs_as_valid() {
        let path = PathBuf::from("target/test-rootfs-valid");
        if path.exists() {
            fs::remove_dir_all(&path).unwrap();
        }

        initialize(&path).unwrap();

        let report = inspect(&path).unwrap();
        assert!(report.is_valid());

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn reports_missing_directories() {
        let path = PathBuf::from("target/test-rootfs-incomplete");
        if path.exists() {
            fs::remove_dir_all(&path).unwrap();
        }
        fs::create_dir_all(path.join("bin")).unwrap();

        let report = inspect(&path).unwrap();
        assert!(!report.is_valid());
        assert!(report.missing_directories().contains(&"etc"));

        fs::remove_dir_all(path).unwrap();
    }
}
