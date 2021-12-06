#![feature(map_try_insert)]

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod intcode;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_lines<P>(path: P) -> Vec<String>
where P: AsRef<Path> {
  BufReader::new(File::open(path).unwrap()).lines().map(Result::unwrap).collect()
}
pub fn to_vec_string<'a>(arr: impl IntoIterator<Item = &'a str>) -> Vec<String> {
  arr.into_iter().map(str::to_owned).collect()
}
