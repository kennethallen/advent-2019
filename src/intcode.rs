use divrem::DivRem;
use std::convert::TryInto;
use std::str;

#[derive(Clone)]
pub struct Intcode {
  pub tape: Vec<i32>,
}

impl Intcode {
  pub fn load_text(lines: &Vec<String>) -> Intcode {
    Intcode {
      tape: lines.iter()
        .flat_map(|l| l.split(",").map(str::parse).map(Result::unwrap).collect::<Vec<i32>>())
        .collect(),
    }
  }

  fn decode_op(&self, i: usize) -> (i32, i32, i32, i32) {
    let (div0, de) = self.tape[i].div_rem(100);
    let (div1, c) = div0.div_rem(10);
    let (a, b) = div1.div_rem(10);
    (de, c, b, a)
  }

  fn access_arg(&self, index: usize, mode: i32) -> i32 {
    let immed = self.tape[index];
    match mode {
      0 => {
        let target: usize = immed.try_into().unwrap();
        self.tape[target]
      },
      1 => immed,
      _ => panic!("Unrecognized access mode {}", mode),
    }
  }

  pub fn run_basic(&mut self, noun: i32, verb: i32) -> i32 {
    self.tape[1] = noun;
    self.tape[2] = verb;
    self.run([]);
    self.tape[0]
  }

  pub fn run(&mut self, input: impl IntoIterator<Item = i32>) -> Vec<i32> {
    let mut input_iter = input.into_iter();
    let mut i = 0;
    let mut out = vec![];
    loop {
      if i >= self.tape.len() {
        panic!("Program counter exceeded tape");
      }
      i = match self.decode_op(i) {
        (01, am0, am1, 0) => {
          let target: usize = self.tape[i+3].try_into().unwrap();
          self.tape[target] = self.access_arg(i+1, am0) + self.access_arg(i+2, am1);
          i + 4
        },
        (02, am0, am1, 0) => {
          let target: usize = self.tape[i+3].try_into().unwrap();
          self.tape[target] = self.access_arg(i+1, am0) * self.access_arg(i+2, am1);
          i + 4
        },
        (03, 0, 0, 0) => {
          let target: usize = self.tape[i+1].try_into().unwrap();
          self.tape[target] = input_iter.next().unwrap();
          i + 2
        },
        (04, am0, 0, 0) => {
          out.push(self.access_arg(i+1, am0));
          i + 2
        },
        (05, am0, am1, 0) => {
          if self.access_arg(i+1, am0) != 0 {
            self.access_arg(i+2, am1).try_into().unwrap()
          } else {
            i + 3
          }
        },
        (06, am0, am1, 0) => {
          if self.access_arg(i+1, am0) == 0 {
            self.access_arg(i+2, am1).try_into().unwrap()
          } else {
            i + 3
          }
        },
        (07, am0, am1, 0) => {
          let target: usize = self.tape[i+3].try_into().unwrap();
          self.tape[target] = if self.access_arg(i+1, am0) < self.access_arg(i+2, am1) { 1 } else { 0 };
          i + 4
        },
        (08, am0, am1, 0) => {
          let target: usize = self.tape[i+3].try_into().unwrap();
          self.tape[target] = if self.access_arg(i+1, am0) == self.access_arg(i+2, am1) { 1 } else { 0 };
          i + 4
        },
        (99, 0, 0, 0) => break out,
        _ => panic!("Unknown opcode {} at PC {}", self.tape[i], i),
      }
    }
  }
}
