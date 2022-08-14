use clap::Parser;
use rdu::{log_disk_usage, normalize_path_arg};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    /// The path to check disk usage on. If not provided, use the current working directory.
    path: Option<String>,

    #[clap(short = 'd', long, value_parser)]
    /// The maximum recursive depth to show file sizes of.
    max_depth: Option<u16>,

    #[clap(short, value_parser)]
    /// Make output human readable
    human_readable: bool,
}

fn main() {
    let cli = Args::parse();
    let depth = cli.max_depth.unwrap_or(0);
    let human_readable = cli.human_readable;
    let root_path = match cli.path {
        Some(s) => PathBuf::from(&normalize_path_arg(&s)),
        None => PathBuf::from(&normalize_path_arg("./")),
    };

    log_disk_usage(root_path, depth, human_readable);
}
