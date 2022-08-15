use std::collections::HashMap;
use std::{fs, path::PathBuf};

struct PathSizeMetadata {
    path: PathBuf,
    size: u64,
    depth: u16,
}

enum Depth {
    None,
    Depth(u16),
}

/// Convert path passed in as argument to have a consistent path separator.
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
/// If successful, return an `impl Iterator` to avoid collecting unless explicitly called upon.
///
/// If the provided path is not a path to a directory, but instead to a file or a dead path,
/// return an Error result.
fn read_dir_contents(dir_path: &PathBuf) -> Result<impl Iterator<Item = PathBuf>, &str> {
    if !dir_path.is_dir() {
        return Err("Provided path is not a directory");
    }

    Ok(fs::read_dir(dir_path).unwrap().map(|f| f.unwrap().path()))
}

/// Get directory size by recursively entering each directory within and summing the size of
/// its children until there are no directories left.
fn get_dir_data<'a>(
    dir_path: PathBuf,
    depth: Depth,
) -> Result<(Vec<PathSizeMetadata>, u64), &'a str> {
    if !dir_path.is_dir() {
        return Err("Provided path is not a directory");
    }

    let current_depth: u16 = match depth {
        Depth::None => 0,
        Depth::Depth(d) => d,
    };

    let paths: Vec<PathBuf> = read_dir_contents(&dir_path).unwrap().collect();

    let mut metadata: Vec<PathSizeMetadata> = Vec::new();
    let mut size: u64 = 0;

    for path in paths {
        if path.is_dir() {
            match get_dir_data(path, Depth::Depth(current_depth + 1)) {
                Ok(data) => {
                    metadata.extend(data.0);
                    size += data.1;
                }
                Err(_e) => {}
            }
        } else {
            match get_file_size(path, &Depth::Depth(current_depth + 1)) {
                Ok(data) => {
                    size += data.size;
                    metadata.push(data);
                }
                Err(_e) => {}
            }
        }
    }

    metadata.push(PathSizeMetadata {
        path: dir_path,
        size,
        depth: match depth {
            Depth::None => 0,
            Depth::Depth(d) => d,
        },
    });

    Ok((metadata, size))
}

/// Get the size of a file.
///
/// # Arguments
///
/// * `file_path` - A path to a file. If this path does not point to a file that is not a directory, an error is returned.
/// * `depth` - A depth value to represent the current depth of the path from the starting directory. Can be borrowed instead
/// of owned since the Depth enum contains only a u16, which is copyable by default.
fn get_file_size(file_path: PathBuf, depth: &Depth) -> Result<PathSizeMetadata, &str> {
    if !file_path.is_file() {
        return Err("Get file size provided with a non-file path");
    }

    match fs::metadata(&file_path) {
        Ok(metadata) => Ok(PathSizeMetadata {
            path: file_path,
            size: metadata.len(),
            depth: match depth {
                Depth::None => 0,
                Depth::Depth(d) => *d,
            },
        }),
        Err(e) => {
            // panic on permissions errors or file does not exist
            panic!("{}", e);
        }
    }
}

/// Get total number of digits in a number
fn get_num_digits(size: u64) -> usize {
    (size as f64).log(10.0).ceil() as usize
}

/// Print function in bytes format
fn print_bytes(data: Vec<PathSizeMetadata>) {
    // first pass to determine max value in dataset to adjust width of left column
    let max: u64 = data.iter().max_by_key(|md| md.size).unwrap().size;

    // get number of digits of character
    let max_digits: usize = get_num_digits(max);

    for item in data {
        println!("{:<max_digits$}  {}", item.size, item.path.display());
    }
}

/// Print function in human readable format
fn print_readable(data: Vec<PathSizeMetadata>) {
    // max digits will always be 3 + 1 character for the letter
    let max_digits: usize = 4;
    let units: HashMap<u8, char> = HashMap::from([(0, 'B'), (1, 'K'), (2, 'M'), (3, 'G')]);

    for item in data {
        // truncate off digits until below the 4 digit count
        let mut truncate_count: u8 = 0;
        let mut size = item.size;
        while get_num_digits(size) >= 4 {
            size /= 1024;
            truncate_count += 1;
        }

        println!(
            "{:<max_digits$}  {}",
            format!("{}{}", size, units.get(&truncate_count).unwrap_or(&'?')),
            item.path.display()
        );
    }
}

/// Log disk usage for a given depth and path. The de
pub fn log_disk_usage(path: PathBuf, depth: u16, human_readable: bool, sort: bool) {
    let mut res: Vec<PathSizeMetadata> = get_dir_data(path, Depth::None)
        .unwrap()
        .0
        .into_iter()
        .filter(|data| data.depth <= depth)
        .collect();

    if sort {
        res.sort_by_key(|d| d.size);
    }

    if human_readable {
        print_readable(res)
    } else {
        print_bytes(res)
    }
}
