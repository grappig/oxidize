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

#[cfg(test)]
mod tests {
    use super::{initialize, DIRECTORIES};
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
}
