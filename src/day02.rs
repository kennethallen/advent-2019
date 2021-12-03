use crate::intcode::Intcode;

pub fn part1(lines: &Vec<String>) -> i32 {
  Intcode::load_text(lines).run_basic(12, 2)
}
pub fn part2(lines: &Vec<String>) -> i32 {
  let i = Intcode::load_text(lines);
  for n in 0..100 {
    for v in 0..100 {
      if i.clone().run_basic(n, v) == 19690720 {
        return 100*n + v;
      }
    }
  }
  panic!("No solution found");
}

#[cfg(test)]
mod tests {
  use crate::*;
  fn in0() -> Vec<String> { read_lines("input/02.txt") }

  #[test]
  fn part1() {
    assert_eq!(6730673, super::part1(&in0()));
  }
  #[test]
  fn part2() {
    assert_eq!(3749, super::part2(&in0()));
  }
}
