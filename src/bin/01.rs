use advent_2019::*;
use std::iter;

fn calc_fuel(mass: isize) -> Option<isize> {
  let fuel = mass / 3 - 2;
  if fuel > 0 { Some(fuel) } else { None }
}

fn main() {
  let modules: Vec<isize> = read_lines("input/01.txt").unwrap()
    .map(|line| line.unwrap().parse::<isize>().unwrap())
    .collect();
  let part1: isize = modules.iter().copied()
    .flat_map(calc_fuel)
    .sum();
  println!("Part 1: {}", part1);
  let part2: isize = modules.iter().copied()
    .map(|mass| iter::successors(Some(mass), |&n| calc_fuel(n))
       .skip(1)
       .sum::<isize>())
    .sum();
  println!("Part 2: {}", part2);
}
