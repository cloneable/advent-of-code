use std::{cmp::Ordering, ops::Range};

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

fn binary_search<F>(time: Range<usize>, mut f: F) -> usize
where
  F: FnMut(usize) -> Ordering,
{
  let mut size = time.end;
  let mut left = time.start;
  let mut right = size;
  while left < right {
    let mid = left + size / 2;
    match f(mid) {
      Ordering::Less => left = mid + 1,
      Ordering::Greater => right = mid,
      Ordering::Equal => return mid,
    }
    size = right - left;
  }
  left
}

fn calc(press: usize, time: usize, distance: usize) -> bool {
  let t1 = time - press;
  let d = press * t1;
  d > distance
}
