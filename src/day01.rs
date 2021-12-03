use std::iter;

fn calc_fuel(mass: isize) -> Option<isize> {
  Some(mass / 3 - 2).filter(|&n| n > 0)
}

pub fn part1(lines: &Vec<String>) -> isize {
  lines.iter()
    .map(|s| s.parse::<isize>().unwrap())
    .flat_map(|m| calc_fuel(m))
    .sum()
}
pub fn part2(lines: &Vec<String>) -> isize {
  lines.iter()
    .map(|s| s.parse::<isize>().unwrap())
    .flat_map(|m| iter::successors(calc_fuel(m), |&n| calc_fuel(n)))
    .sum()
}

#[cfg(test)]
mod tests {
  use crate::*;
  fn in0() -> Vec<String> { to_vec_string(["12", "14", "1969", "100756"]) }
  fn in1() -> Vec<String> { read_lines("input/01.txt") }
  fn in2() -> Vec<String> { to_vec_string(["14", "1969", "100756"]) }

  #[test]
  fn part1() {
    assert_eq!(2 + 2 + 654 + 33583, super::part1(&in0()));
    assert_eq!(3474920, super::part1(&in1()));
  }
  #[test]
  fn part2() {
    assert_eq!(2 + 966 + 50346, super::part2(&in2()));
    assert_eq!(5209504, super::part2(&in1()));
  }
}
