use std::{collections::HashSet, str::FromStr};

fn main() {
  let mut guide = RucksackReader::default();
  aoc::read_from_stdin(&mut guide);
  let sum: usize = guide.items.into_iter().map(|i| i.0 as usize).sum();
  println!("{}", sum);
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Item(u8);

impl FromStr for Item {
  type Err = anyhow::Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.len() != 1 {
      return Err(anyhow::Error::msg(format!("wrong len: {}", s)));
    }
    let c = s.bytes().into_iter().next().unwrap();
    if c >= b'a' && c <= b'z' {
      Ok(Item(c - b'a' + 1))
    } else if c >= b'A' && c <= b'Z' {
      Ok(Item(c - b'A' + 1 + 26))
    } else {
      Err(anyhow::Error::msg(format!("wrong char: {}", s)))
    }
  }
}

#[derive(Default, Debug)]
struct RucksackReader {
  member_index: usize,
  group_items: HashSet<Item>,
  items: Vec<Item>,
}

impl aoc::LineParser for RucksackReader {
  fn parse_line(&mut self, line: &str) {
    let mut group_items = HashSet::new();
    for i in 0..line.len() {
      let item: Item = line[i..i + 1].parse().unwrap();
      if self.group_items.len() == 0 || self.group_items.contains(&item) {
        group_items.insert(item);
      }
    }
    self.group_items = group_items;
    self.member_index = (self.member_index + 1) % 3;

    if self.member_index == 0 {
      debug_assert_eq!(1, self.group_items.len());
      self.items.push(*self.group_items.iter().next().unwrap());
      self.group_items.clear();
    }
  }
}
