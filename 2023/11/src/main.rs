const EXPANSION: usize = 1_000_000;

fn main() {
  let mut galaxies = Vec::new();
  let mut nonempty_cols = Vec::new();

  let mut r = 0;
  for line in std::io::stdin().lines().map(Result::unwrap) {
    if r == 0 {
      nonempty_cols.resize(line.len(), false);
    }
    let mut r_inc = EXPANSION;
    for (c, b) in line.bytes().enumerate() {
      if b == b'#' {
        galaxies.push((r, c));
        nonempty_cols[c] = true;
        r_inc = 1;
      }
    }
    r += r_inc;
  }

  let mut sum = 0;
  for a in 0..galaxies.len() - 1 {
    for b in a + 1..galaxies.len() {
      sum += distance(galaxies[a], galaxies[b], &nonempty_cols);
    }
  }

  println!("{sum}");
}

fn distance(
  a: (usize, usize),
  b: (usize, usize),
  nonempty_cols: &[bool],
) -> usize {
  let r_range = a.0.min(b.0)..a.0.max(b.0);
  let c_range = a.1.min(b.1)..a.1.max(b.1);

  let extra_cols = c_range.clone().filter(|&c| !nonempty_cols[c]).count();

  let rows = r_range.len();
  let cols = c_range.len() + extra_cols * (EXPANSION - 1);

  rows + cols
}
