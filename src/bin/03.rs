#![feature(map_try_insert)]
use advent_2019::*;
use std::collections;
use std::str::{self, FromStr};
use strum;

#[derive(Hash, Eq, PartialEq, Debug, strum::EnumString)]
enum Dir { U, D, L, R }

type WireData = collections::HashMap<(isize, isize), isize>;

fn wire_data(line: &str) -> WireData {
  let mut x = 0;
  let mut y = 0;
  let mut steps = 0;
  let mut coords = collections::HashMap::new();
  for s in line.split(",") {
    let dir = Dir::from_str(&s[0..1]).unwrap();
    let stride = s[1..].parse::<isize>().unwrap();
    use crate::Dir::*;
    match dir {
      U => y += stride,
      D => y -= stride,
      R => x += stride,
      L => x -= stride,
    }
    steps += stride;
    for n in 0..stride {
      let _ = coords.try_insert(match dir {
        U => (x, y - n),
        D => (x, y + n),
        R => (x - n, y),
        L => (x + n, y),
      }, steps - n);
    }
  }
  coords
}

fn merge_wire_data(w0: WireData, w1: WireData) -> WireData {
  w0.into_iter()
    .flat_map(|(coord, steps0)| w1.get(&coord)
      .map(|steps1| (coord, steps0 + steps1)))
    .collect()
}

fn main() {
  let coords = read_lines("input/03.txt").unwrap()
    .map(|line| wire_data(&line.unwrap()))
    .reduce(merge_wire_data)
    .unwrap();
  let part1 = coords.keys().map(|(x, y)| x.abs() + y.abs()).min().unwrap();
  println!("Part 1: {:?}", part1);
  let part2 = coords.values().min().unwrap();
  println!("Part 2: {:?}", part2);
}
