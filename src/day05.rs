use crate::intcode::Intcode;

pub fn part1(lines: &Vec<String>) -> i32 {
  *Intcode::load_text(lines).run([1]).last().unwrap()
}
pub fn part2(lines: &Vec<String>) -> i32 {
  *Intcode::load_text(lines).run([5]).last().unwrap()
}

#[cfg(test)]
mod tests {
  use crate::*;
  fn in0() -> Vec<String> { read_lines("input/05.txt") }

  #[test]
  fn part1() {
    assert_eq!(15386262, super::part1(&in0()));
  }
  #[test]
  fn part2() {
    assert_eq!(10376124, super::part2(&in0()));
  }
}
