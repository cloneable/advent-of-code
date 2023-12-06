fn main() {
  let mut lines = std::io::stdin().lines();

  let times: usize = lines
    .next()
    .unwrap()
    .unwrap()
    .split_ascii_whitespace()
    .skip(1)
    .map(|n| n.parse().unwrap())
    .zip(
      lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap()),
    )
    .map(calc)
    .product();

  println!("{times}");
}

fn calc((time, distance): (usize, usize)) -> usize {
  let mut count = 0;
  for t0 in 1..time {
    let t1 = time - t0;
    let d = t0 * t1;
    if d > distance {
      count += 1
    }
  }
  count
}
