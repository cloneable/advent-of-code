use aoc::binary_search;
use std::cmp::Ordering;

fn main() {
  let mut lines = std::io::stdin().lines();

  let time = lines
    .next()
    .unwrap()
    .unwrap()
    .split_ascii_whitespace()
    .skip(1)
    .collect::<String>()
    .parse::<usize>()
    .unwrap();
  let distance = lines
    .next()
    .unwrap()
    .unwrap()
    .split_ascii_whitespace()
    .skip(1)
    .collect::<String>()
    .parse::<usize>()
    .unwrap();

  let start = binary_search(1..time, |mid| {
    let left = calc(mid - 1, time, distance);
    let mid = calc(mid, time, distance);

    match (left, mid) {
      (false, true) => Ordering::Equal,
      (false, false) => Ordering::Less,
      (true, _) => Ordering::Greater,
    }
  });

  let end = binary_search(1..time, |mid| {
    let right = calc(mid + 1, time, distance);
    let mid = calc(mid, time, distance);

    match (mid, right) {
      (true, false) => Ordering::Equal,
      (false, false) => Ordering::Greater,
      (_, true) => Ordering::Less,
    }
  });

  let ways = end - start + 1;

  println!("{ways}");
}

fn calc(press: usize, time: usize, distance: usize) -> bool {
  let t1 = time - press;
  let d = press * t1;
  d > distance
}
