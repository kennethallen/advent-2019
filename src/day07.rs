use crate::intcode::Intcode;

fn optimize(comp: &Intcode, value: i32, remaining: &[i32]) -> i32 {
  remaining.into_iter()
    .map(|&digit| {
      let result = comp.clone().run([digit, value])[0];
      let rest: Vec<i32> = remaining.into_iter().copied().filter(|&e| e != digit).collect();
      optimize(comp, result, &rest)
    })
    .max()
    .unwrap_or(value)
}

pub fn part1(lines: &Vec<String>) -> i32 {
  optimize(&Intcode::load_text(lines), 0, &(0..5).collect::<Vec<i32>>())
}
pub fn part2(lines: &Vec<String>) -> i32 {
  let base = Intcode::load_test(lines);
  let mut comps: Vec<Intcode> = (0..5).map(|_| base.clone()).collect();

}

#[cfg(test)]
mod tests {
  use crate::*;
  fn in0() -> Vec<String> { to_vec_string(["3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"]) }
  fn in1() -> Vec<String> { to_vec_string(["3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"]) }
  fn in2() -> Vec<String> { to_vec_string(["3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"]) }
  fn in3() -> Vec<String> { read_lines("input/07.txt") }

  #[test]
  fn part1() {
    assert_eq!(43210, super::part1(&in0()));
    assert_eq!(54321, super::part1(&in1()));
    assert_eq!(65210, super::part1(&in2()));
    assert_eq!(19650, super::part1(&in3()));
  }
  #[test]
  fn part2() {
    //assert_eq!(10376124, super::part2(&in0()));
  }
}
