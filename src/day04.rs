use divrem::DivRem;
use std::convert::TryInto;
use std::iter;

fn nondecreasing(pass: &[u8]) -> bool {
  pass.windows(2).all(|w| w[0] <= w[1])
}
fn has_double_lax(pass: &[u8]) -> bool {
  pass.windows(2).any(|w| w[0] == w[1])
}
fn has_double_strict(pass: &[u8]) -> bool {
  (1..pass.len()).into_iter()
    .any(|i| pass[i - 1] == pass[i]
      && (i < 2 || pass[i - 2] != pass[i])
      && (i + 1 >= pass.len() || pass[i + 1] != pass[i]))
}

struct Scanner {
  lo: Vec<u8>,
  hi: Vec<u8>,
  stack: Vec<u8>,
}
impl Scanner {
  fn to_digits(n: usize) -> Vec<u8> {
    fn try_digit(x: usize) -> Option<(usize, usize)> {
      if x == 0 { None } else { Some(x.div_rem(10)) }
    }
    let mut v: Vec<u8> = iter::successors(try_digit(n), |(q, _)| try_digit(*q))
      .map(|(_, r)| r.try_into().unwrap())
      .collect();
    v.reverse();
    v
  }

  fn new(lo: usize, hi: usize) -> Scanner {
    Scanner {
      lo: Scanner::to_digits(lo),
      hi: Scanner::to_digits(hi),
      stack: vec![],
    }
  }

  fn scan(&mut self, pred: &impl Fn(&[u8]) -> bool) -> usize {
    if self.stack.len() < self.lo.len() {
      let lo_digit = if &self.stack[..] > &self.lo[..self.stack.len()] { 0 } else { self.lo[self.stack.len()] };
      let hi_digit = if &self.stack[..] < &self.hi[..self.stack.len()] { 9 } else { self.hi[self.stack.len()] };
      let mut sum = 0;
      for digit in lo_digit..=hi_digit {
        self.stack.push(digit);
        sum += self.scan(pred);
        self.stack.pop();
      }
      sum
    } else if pred(&self.stack) {
      1
    } else {
      0
    }
  }
}

pub fn part1(lines: &Vec<String>) -> usize {
  Scanner::new(lines[0].parse().unwrap(), lines[1].parse().unwrap())
    .scan(&|p| nondecreasing(p) && has_double_lax(p))
}
pub fn part2(lines: &Vec<String>) -> usize {
  Scanner::new(lines[0].parse().unwrap(), lines[1].parse().unwrap())
    .scan(&|p| nondecreasing(p) && has_double_strict(p))
}

#[cfg(test)]
mod tests {
  use crate::*;
  fn in0() -> Vec<String> { read_lines("input/04.txt") }

  #[test]
  fn part1() {
    assert_eq!(594, super::part1(&in0()));
  }
  #[test]
  fn part2() {
    assert_eq!(364, super::part2(&in0()));
  }
}
