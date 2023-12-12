use std::fmt::{self, Write};

fn main() {
  let mut sum = 0;

  for line in std::io::stdin().lines().map(Result::unwrap) {
    if line.is_empty() {
      break;
    }
    let (springs, gs) = line.split_once(' ').unwrap();

    let springs = springs.bytes().map(Condition::from).collect::<Vec<_>>();
    let groups = gs
      .split(',')
      .map(|n| str::parse::<usize>(n).unwrap())
      .collect::<Vec<_>>();

    match consume(&springs, &groups) {
      Some(arrangements) => sum += arrangements,
      None => unreachable!(),
    }
  }

  println!("{sum}");
}

fn consume(springs: &[Condition], groups: &[usize]) -> Option<usize> {
  if groups.is_empty() && springs.iter().all(|&c| c != Condition::Damaged) {
    return Some(1);
  }
  let Some(&group) = groups.first() else {
    return None;
  };

  let mut sum = 0;

  if can_consume(springs, group) {
    if let Some(springs) = advance(springs, group) {
      match consume(springs, &groups[1..]) {
        Some(arrangements) => sum += arrangements,
        None => (),
      }
    }
  }

  if springs.get(0).is_some_and(|&c| c != Condition::Damaged) {
    match consume(&springs[1..], &groups) {
      Some(arrangements) => sum += arrangements,
      None => (),
    }
  }

  Some(sum)
}

fn advance(springs: &[Condition], group: usize) -> Option<&[Condition]> {
  if group + 1 <= springs.len() {
    Some(&springs[group + 1..])
  } else if group == springs.len() {
    Some(&springs[0..0])
  } else {
    None
  }
}

fn can_consume(springs: &[Condition], group: usize) -> bool {
  if springs.len() < group {
    return false;
  }
  for i in 0..group {
    match springs[i] {
      Condition::Damaged | Condition::Unknown => (),
      Condition::Operational => return false,
    }
  }
  springs.len() == group || springs[group] != Condition::Damaged
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
enum Condition {
  Operational = b'.',
  Damaged = b'#',
  Unknown = b'?',
}

impl From<u8> for Condition {
  fn from(value: u8) -> Self {
    match value {
      b'.' => Condition::Operational,
      b'#' => Condition::Damaged,
      b'?' => Condition::Unknown,
      _ => unreachable!(),
    }
  }
}

impl fmt::Debug for Condition {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_char(*self as u8 as char)
  }
}
