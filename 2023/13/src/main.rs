fn main() {
  let mut sum = 0;

  let mut rows = Vec::new();
  let mut row_len = 0;
  for line in std::io::stdin().lines().map(Result::unwrap) {
    if line.is_empty() {
      sum += search(&rows, row_len);
      rows.clear();
      row_len = 0;
    } else {
      if row_len == 0 {
        row_len = line.len();
      }
      rows.push(line2bits(&line));
    }
  }
  if !rows.is_empty() {
    sum += search(&rows, row_len);
  }

  println!("{sum}");
}

fn line2bits(line: &str) -> u32 {
  line.as_bytes().iter().map(|&b| b == b'#').fold(0, |a, b| (a << 1) | b as u32)
}

fn col2bits(grid: &[u32], col: usize) -> u32 {
  grid
    .iter()
    .map(|row| row & (1 << col) != 0)
    .fold(0, |a, b| (a << 1) | b as u32)
}

fn search(rows: &[u32], row_len: usize) -> usize {
  let mut columns = Vec::new();
  for c in (0..row_len).rev() {
    columns.push(col2bits(&rows, c));
  }

  let mut sum = 0;

  if let Some(col) = search_nums(&columns, rows.len()) {
    sum += col;
  }

  if let Some(row) = search_nums(&rows, row_len) {
    sum += row * 100;
  }

  search_nums(&rows, row_len);

  assert_ne!(0, sum);
  sum
}

fn search_nums(nums: &[u32], width: usize) -> Option<usize> {
  for i in 0..nums.len() {
    if i != 0 {
      if check(nums, width, i - 1, i, false) == (true, true) {
        return Some(i);
      }
    }
  }

  None
}

fn check(
  nums: &[u32],
  width: usize,
  a: usize,
  b: usize,
  smudge: bool,
) -> (bool, bool) {
  if b >= nums.len() {
    return (true, smudge);
  }
  if nums[a] == nums[b] {
    if a == 0 {
      return (true, smudge);
    }
    check(nums, width, a - 1, b + 1, smudge)
  } else if !smudge && (nums[a] ^ nums[b]).count_ones() == 1 {
    if a == 0 {
      return (true, true);
    }
    check(nums, width, a - 1, b + 1, true)
  } else {
    (false, smudge)
  }
}
