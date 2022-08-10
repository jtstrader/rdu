use std::path::PathBuf;

use clap::Parser;

use rdu::get_paths;
use rdu::normalize_path_arg;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    /// The path to check disk usage on. If not provided, use the current working directory.
    path: Option<String>,

    #[clap(short = 'd', long, value_parser)]
    /// The maximum recursive depth to show file sizes of.
    max_depth: Option<u8>,
}

fn main() {
    let cli = Args::parse();
    let root = match cli.path {
        Some(s) => s,
        None => String::from("./"),
    };

    let paths: Vec<PathBuf> = match get_paths(&normalize_path_arg(&root)) {
        Ok(vec) => vec,
        Err(_e) => vec![],
    };

    println!("{:#?}", paths);
}
