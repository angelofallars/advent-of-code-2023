use std::{fs, path::Path};

pub fn read<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<char>> {
    fs::read(path).map(|bytes| bytes.into_iter().map(|i| i as char).collect())
}
