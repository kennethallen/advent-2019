use advent_2019::*;
use std::collections::{HashSet, HashMap};

type Orbits = HashMap<String, HashSet<String>>;

fn count_orbits(orbits: &Orbits, b: &String, depth: usize) -> usize {
  depth + match orbits.get(b) {
    Some(bs) => bs.iter().map(|b1| count_orbits(orbits, b1, depth + 1)).sum(),
    None => 0,
  }
}

fn main() {
  let data: Vec<(String, String)> = read_lines("input/06.txt").unwrap()
    .map(|line| line.unwrap())
    .map(|line| {
      let mut iter = line.split(")");
      (iter.next().unwrap().to_owned(), iter.next().unwrap().to_owned())
    })
    .collect();
  let mut orbits: Orbits = HashMap::new();
  for (b0, b1) in data {
    if let Some(bs) = orbits.get_mut(&b0) {
      bs.insert(b1);
    } else {
      orbits.insert(b0, HashSet::from([b1]));
    }
  }
  let root = orbits.keys()
      .find(|b| orbits.values().all(|bs| !bs.contains(b.as_str())))
      .unwrap();
  let part1 = count_orbits(&orbits, root, 0);
  println!("Part 1: {}", part1);
}
