use advent_2019::*;
use divrem::DivRem;
use std::convert::TryInto;

fn decode_op(opcode: i32) -> (i32, i32, i32, i32) {
  let (div0, de) = opcode.div_rem(100);
  let (div1, c) = div0.div_rem(10);
  let (a, b) = div1.div_rem(10);
  (de, c, b, a)
}

fn intcode_access(tape: &[i32], index: usize, mode: i32) -> i32 {
    let immed = tape[index];
  match mode {
    0 => {
      let target: usize = immed.try_into().unwrap();
      tape[target]
    },
    1 => immed,
    n => panic!("Unrecognized access mode {}", n),
  }
}

fn intcode(tape: &mut [i32], input: impl IntoIterator<Item = i32>) -> Vec<i32> {
  let mut input_iter = input.into_iter();
  let mut i = 0;
  let mut out = vec![];
  loop {
    if i >= tape.len() {
      panic!("Program counter exceeded tape");
    }
    i = match decode_op(tape[i]) {
      (01, am0, am1, 0) => {
        let target: usize = tape[i+3].try_into().unwrap();
        tape[target] = intcode_access(&tape, i+1, am0) + intcode_access(&tape, i+2, am1);
        i + 4
      },
      (02, am0, am1, 0) => {
        let target: usize = tape[i+3].try_into().unwrap();
        tape[target] = intcode_access(&tape, i+1, am0) * intcode_access(&tape, i+2, am1);
        i + 4
      },
      (03, 0, 0, 0) => {
        let target: usize = tape[i+1].try_into().unwrap();
        tape[target] = input_iter.next().unwrap();
        i + 2
      },
      (04, am0, 0, 0) => {
        out.push(intcode_access(&tape, i+1, am0));
        i + 2
      },
      (05, am0, am1, 0) => {
        if intcode_access(&tape, i+1, am0) != 0 {
          intcode_access(&tape, i+2, am1).try_into().unwrap()
        } else {
          i + 3
        }
      },
      (06, am0, am1, 0) => {
        if intcode_access(&tape, i+1, am0) == 0 {
          intcode_access(&tape, i+2, am1).try_into().unwrap()
        } else {
          i + 3
        }
      },
      (07, am0, am1, 0) => {
        let target: usize = tape[i+3].try_into().unwrap();
        tape[target] = if intcode_access(&tape, i+1, am0) < intcode_access(&tape, i+2, am1) { 1 } else { 0 };
        i + 4
      },
      (08, am0, am1, 0) => {
        let target: usize = tape[i+3].try_into().unwrap();
        tape[target] = if intcode_access(&tape, i+1, am0) == intcode_access(&tape, i+2, am1) { 1 } else { 0 };
        i + 4
      },
      (99, 0, 0, 0) => break out,
      _ => panic!("Unknown opcode {} at PC {}", tape[i], i),
    }
  }
}

fn main() {
  let tape: Vec<i32> = read_lines("input/05.txt").unwrap()
    .flat_map(|line| line.unwrap().split(",")
      .map(|word| word.parse().unwrap())
      .collect::<Vec<i32>>())
    .collect();
  let part1 = intcode(&mut tape.clone(), vec![1]);
  println!("Part 1: {}", part1.last().unwrap());
  let part2 = intcode(&mut tape.clone(), vec![5]);
  println!("Part 2: {}", part2.last().unwrap());
}
