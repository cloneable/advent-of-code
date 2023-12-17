use std::convert::identity;

fn main() {
  let platform = std::io::stdin()
    .lines()
    .map(Result::unwrap)
    .map(|line| line.chars().collect())
    .collect::<Vec<Vec<char>>>();

  let mut sum = 0;

  let mut stacks: Vec<Option<Stack>> = vec![None; platform[0].len()];
  for (row, r) in platform.iter().rev().zip(1usize..) {
    for (c, &v) in row.iter().enumerate() {
      match v {
        '.' => {
          if let Some(stack) = &mut stacks[c] {
            stack.row = r;
          }
        }
        'O' => {
          let stack = stacks[c].get_or_insert(Default::default());
          stack.height += 1;
          stack.row = r;
        }
        '#' => {
          if let Some(stack) = &mut stacks[c] {
            sum += stack.load();
          }
          stacks[c] = None;
        }
        _ => unreachable!(),
      }
    }
  }

  sum += stacks
    .into_iter()
    .filter_map(identity)
    .map(|stack| stack.load())
    .sum::<usize>();

  println!("{sum}");
}

#[derive(Clone, Default, Debug)]
struct Stack {
  row: usize,
  height: usize,
}

impl Stack {
  fn load(&self) -> usize {
    (self.row + self.row + 1 - self.height) * self.height / 2
  }
}
