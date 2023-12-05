use std::{io, ops::Range};

fn main() {
  let mut prev: Option<String> = None;
  let mut lines = io::stdin().lines().peekable();
  let mut sum = 0;
  while let Some(current) = lines.next() {
    let current = current.unwrap();
    let next = lines.peek().map(|l| l.as_deref().unwrap());
    process_line(&mut sum, &current, prev.as_ref().map(|s| s.as_str()), next);
    prev = Some(current);
  }
  println!("{sum}");
}

fn process_line(
  sum: &mut usize,
  curr: &str,
  prev: Option<&str>,
  next: Option<&str>,
) {
  let prev_nums = prev.map(number_indices);
  let curr_nums = number_indices(curr);
  let next_nums = next.map(number_indices);

  curr.as_bytes().iter().enumerate().filter(|(_, &c)| c == b'*').for_each(
    |(i, _)| {
      let mut total_nums = Vec::new();
      total_nums.extend(adjacent_numbers(i, prev_nums.as_ref()));
      total_nums.extend(adjacent_numbers(i, Some(&curr_nums)));
      total_nums.extend(adjacent_numbers(i, next_nums.as_ref()));
      if total_nums.len() == 2 {
        *sum += total_nums.iter().product::<usize>();
      }
    },
  );
}

fn adjacent_numbers(
  index: usize,
  nums: Option<&Vec<(Range<usize>, usize)>>,
) -> Vec<usize> {
  let mut v = Vec::new();
  let Some(nums) = nums else {
    return v;
  };
  for (num_range, num) in nums {
    let start = num_range.start.saturating_sub(1);
    let end = num_range.end + 1;
    if (start..end).contains(&index) {
      v.push(*num);
    }
  }
  v
}

fn number_indices(line: &str) -> Vec<(Range<usize>, usize)> {
  let mut v = Vec::new();
  let mut num_start = Some(0usize);
  let mut num = 0usize;
  for (i, &c) in line.as_bytes().iter().enumerate() {
    if c.is_ascii_digit() {
      if num_start.is_none() {
        num_start = Some(i);
      }
      num = num * 10 + (c - b'0') as usize;
    } else if let Some(i0) = num_start {
      v.push((i0..i, num));
      num = 0;
      num_start = None;
    }
  }
  if let Some(i0) = num_start {
    v.push((i0..line.len(), num));
  }
  v
}
