use std::{
    fs,
    path::{Path, PathBuf},
};

/// Convert path passed in as argument to have a consistent path separator
pub fn normalize_path_arg(path: &str) -> String {
    // change "/" -> "\" if on Windows, and vice versa if on Unix-based

    if cfg!(windows) {
        path.chars()
            .map(|c| if c == '/' { '\\' } else { c })
            .collect()
    } else {
        path.chars()
            .map(|c| if c == '\\' { '/' } else { c })
            .collect()
    }
}

/// Receives an absolute or relative path and returns all paths in the provided directory.
///
/// If the provided path is not a path to a directory, but instead to a file or a dead path,
/// return an Error result.
pub fn get_paths(dir_path: &str) -> Result<Vec<PathBuf>, &str> {
    if !Path::new(dir_path).is_dir() {
        return Err("Provided path is not a directory");
    }

    Ok(fs::read_dir(dir_path)
        .unwrap()
        .map(|f| f.unwrap().path())
        .collect())
}
