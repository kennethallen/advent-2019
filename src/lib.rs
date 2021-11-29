use std::fs;
use std::io::{self, BufRead};
use std::path;

pub fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where P: AsRef<path::Path>, {
  Ok(io::BufReader::new(fs::File::open(path)?).lines())
}
