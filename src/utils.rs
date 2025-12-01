use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

const INPUT_ROOT: &str = "input";
pub fn load_input(filename: &str) -> io::Result<Vec<String>> {
    let input = build_input_path(filename);
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    Ok(lines)
}

fn build_input_path(target: &str) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = [manifest_dir, INPUT_ROOT, target].iter().collect();
    path
}