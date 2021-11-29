use advent_2019::*;
use std::iter;

fn calc_fuel(mass: isize) -> Option<isize> {
  Some(mass / 3 - 2).filter(|&n| n > 0)
}

fn main() {
  let mut part1 = 0;
  let mut part2 = 0;
  for line in read_lines("input/01.txt").unwrap() {
    let mass: isize = line.unwrap().parse().unwrap();
    if let Some(fuel) = calc_fuel(mass) {
      part1 += fuel;
      part2 += iter::successors(Some(fuel), |&n| calc_fuel(n)).sum::<isize>();
    }
  }
  println!("Part 1: {}", part1);
  println!("Part 2: {}", part2);
}
