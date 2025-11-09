use std::path::{Path, PathBuf};

pub fn get_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

pub fn parse_with_root(string: String) -> PathBuf {
    let mut path: PathBuf = get_root();
    path.push(string);
    path
}
