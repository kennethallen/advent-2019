use std::collections::HashMap;
use std::str::{FromStr};
use strum::EnumString;

#[derive(Hash, Eq, PartialEq, Debug, EnumString)]
enum Dir { U, D, L, R }

type WireData = HashMap<(isize, isize), isize>;

fn wire_data(line: &str) -> WireData {
  let mut x = 0;
  let mut y = 0;
  let mut steps = 0;
  let mut coords = HashMap::new();
  for s in line.split(",") {
    let dir = Dir::from_str(&s[0..1]).unwrap();
    let stride: isize = s[1..].parse().unwrap();
    use Dir::*;
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

fn parse(lines: &Vec<String>) -> WireData {
  lines.iter()
    .map(String::as_str)
    .map(wire_data)
    .reduce(merge_wire_data)
    .unwrap()
}

pub fn part1(lines: &Vec<String>) -> isize {
  parse(lines).keys().map(|(x, y)| x.abs() + y.abs()).min().unwrap()
}
pub fn part2(lines: &Vec<String>) -> isize {
  *parse(lines).values().min().unwrap()
}

#[cfg(test)]
mod tests {
  use crate::*;
  fn in0() -> Vec<String> { to_vec_string(["R8,U5,L5,D3", "U7,R6,D4,L4"]) }
  fn in1() -> Vec<String> { to_vec_string(["R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"]) }
  fn in2() -> Vec<String> { to_vec_string(["R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"]) }
  fn in3() -> Vec<String> { read_lines("input/03.txt") }

  #[test]
  fn part1() {
    assert_eq!(6, super::part1(&in0()));
    assert_eq!(159, super::part1(&in1()));
    assert_eq!(135, super::part1(&in2()));
    assert_eq!(806, super::part1(&in3()));
  }
  #[test]
  fn part2() {
    assert_eq!(30, super::part2(&in0()));
    assert_eq!(610, super::part2(&in1()));
    assert_eq!(410, super::part2(&in2()));
    assert_eq!(66076, super::part2(&in3()));
  }
}
