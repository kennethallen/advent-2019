use advent_2019::*;
use std::str;

fn intcode(mut tape: Vec<usize>, noun: usize, verb: usize) -> usize {
  tape[1] = noun;
  tape[2] = verb;
  let mut i = 0;
  loop {
    if i >= tape.len() {
      panic!("Program counter exceeded tape");
    }
    match tape[i] {
      1 => {
        let target = tape[i + 3]; 
        tape[target] = tape[tape[i + 1]] + tape[tape[i + 2]];
        i += 4;
       },
      2 => {
        let target = tape[i + 3];
        tape[target] = tape[tape[i + 1]] * tape[tape[i + 2]];
        i += 4;
       },
      99 => break tape[0],
      n => panic!("Unknown opcode {} at PC {}", n, i),
    }
  }
}

fn main() {
  let tape: Vec<usize> = read_lines("input/02.txt").unwrap()
    .map(|line| line.unwrap())
    .flat_map(|line| line.split(",").map(str::to_owned).collect::<Vec<String>>())
    .map(|word| word.parse::<usize>().unwrap())
    .collect();

  let part1 = intcode(tape.clone(), 12, 2);
  println!("Part 1: {}", part1);
  for n in 0..100 {
    for v in 0..100 {
      if intcode(tape.clone(), n, v) == 19690720 {
        let part2 = 100 * n + v;
        println!("Part 2: {}", part2);
        break
      }
    }
  }
}
