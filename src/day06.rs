use std::collections::{HashSet, HashMap, VecDeque};

struct System {
  bodies: HashMap<String, Body>,
}
impl System {
  fn new() -> System {
    System {
      bodies: HashMap::new(),
    }
  }
  fn parse<'a>(lines: impl IntoIterator<Item = &'a str>) -> System {
    lines.into_iter().fold(System::new(), |mut system, line| {
      let mut iter = line.split(")");
      system.add_orbit(iter.next().unwrap(), iter.next().unwrap());
      system
    })
  }

  fn add_orbit(&mut self, parent_name: &str, child_name: &str) {
    if let Some(parent) = self.bodies.get_mut(parent_name) {
      parent.children.push(child_name.to_owned());
    } else {
      self.bodies.insert(parent_name.to_owned(), Body {
        parents: vec![],
        children: vec![child_name.to_owned()],
      });
    }
    if let Some(child) = self.bodies.get_mut(child_name) {
      child.parents.push(parent_name.to_owned());
    } else {
      self.bodies.insert(child_name.to_owned(), Body {
        parents: vec![parent_name.to_owned()],
        children: vec![],
      });
    }
  }

  fn count_orbits(&self, name: &str) -> usize {
    self.count_orbits_recurse(name, 0)
  }
  fn count_orbits_recurse(&self, name: &str, depth: usize) -> usize {
    depth + self.bodies[name].children.iter()
      .map(|c| self.count_orbits_recurse(c, depth + 1))
      .sum::<usize>()
  }

  fn orbit_distance(&self, start: &str, end: &str) -> usize {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut queue = VecDeque::from([(start, 0)]);
    while let Some((name, depth)) = queue.pop_front() {
      if name == end {
        return depth;
      }
      let node = self.bodies.get(name).unwrap();
      for next in node.parents.iter().chain(node.children.iter()) {
        if !visited.contains(next.as_str()) {
          queue.push_back((next, depth + 1));
        }
      }
      visited.insert(name); 
    }
    panic!["Destination not found"];
  }
}

struct Body {
  parents: Vec<String>,
  children: Vec<String>,
}

pub fn part1(lines: &Vec<String>) -> usize {
  System::parse(lines.iter().map(String::as_str)).count_orbits("COM")
}
pub fn part2(lines: &Vec<String>) -> usize {
  System::parse(lines.iter().map(String::as_str)).orbit_distance("YOU", "SAN") -2
}

#[cfg(test)]
mod tests {
  use crate::*;
  fn in0() -> Vec<String> { to_vec_string(["COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L"]) }
  fn in1() -> Vec<String> { to_vec_string(["COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU", "I)SAN"]) }
  fn in2() -> Vec<String> { read_lines("input/06.txt") }

  #[test]
  fn part1() {
    assert_eq!(42, super::part1(&in0()));
    assert_eq!(312697, super::part1(&in2()));
  }
  #[test]
  fn part2() {
    assert_eq!(4, super::part2(&in1()));
    assert_eq!(466, super::part2(&in2()));
  }
}
