use itertools::Itertools;
use std::collections::HashSet;

fn main() {
  let galaxies = std::io::stdin()
    .lines()
    .map(Result::unwrap)
    .enumerate()
    .map(|(r, line)| {
      line
        .bytes()
        .enumerate()
        .filter(|(_, b)| *b == b'#')
        .map(|(c, _)| (r, c).into())
        .collect::<Vec<Pos>>()
    })
    .flatten()
    .collect::<Vec<_>>();

  let rows = galaxies.iter().map(|p| p.row).collect::<HashSet<_>>();
  let cols = galaxies.iter().map(|p| p.col).collect::<HashSet<_>>();

  let mut sum = 0;
  for (a, b) in galaxies.into_iter().tuple_combinations() {
    sum += a.distance(b, &rows, &cols);
  }

  println!("{sum}");
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
  row: usize,
  col: usize,
}

impl From<(usize, usize)> for Pos {
  fn from((row, col): (usize, usize)) -> Self {
    Pos { row, col }
  }
}

impl Pos {
  fn distance(
    self,
    other: Pos,
    rows: &HashSet<usize>,
    cols: &HashSet<usize>,
  ) -> usize {
    let r_start = self.row.min(other.row);
    let r_end = self.row.max(other.row);
    let c_start = self.col.min(other.col);
    let c_end = self.col.max(other.col);

    let extra_rows = (r_start..r_end).filter(|r| !rows.contains(r)).count();
    let extra_cols = (c_start..c_end).filter(|c| !cols.contains(c)).count();

    let rows = (r_end - r_start) + extra_rows;
    let cols = (c_end - c_start) + extra_cols;

    rows + cols
  }
}
