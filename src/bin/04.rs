use advent_2019::*;

fn nondecreasing(pass: &[u8]) -> bool {
  (1..pass.len()).into_iter()
    .all(|i| pass[i - 1] <= pass[i])
}
fn has_double_lax(pass: &[u8]) -> bool {
  (1..pass.len()).into_iter()
    .any(|i| pass[i - 1] == pass[i])
}
fn has_double_strict(pass: &[u8]) -> bool {
  (1..pass.len()).into_iter()
    .any(|i| pass[i - 1] == pass[i]
      && (i < 2 || pass[i - 2] != pass[i])
      && (i + 1 >= pass.len() || pass[i + 1] != pass[i]))
}

struct Scanner<'a> {
  lo: &'a [u8],
  hi: &'a [u8],
  stack: Vec<u8>,
}
impl Scanner<'_> {
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

fn main() {
  let bounds: Vec<Vec<u8>> = read_lines("input/04.txt").unwrap()
    .map(|line| line.unwrap().chars()
      .map(|digit| digit.to_string().parse::<u8>().unwrap())
      .collect())
    .collect();
  let part1 = Scanner { lo: &bounds[0], hi: &bounds[1], stack: vec![] }
    .scan(&|p| nondecreasing(p) && has_double_lax(p));
  println!("Part 1: {}", part1);
  let part2 = Scanner { lo: &bounds[0], hi: &bounds[1], stack: vec![] }
    .scan(&|p| nondecreasing(p) && has_double_strict(p));
  println!("Part 2: {}", part2);
}
