fn main() {
  let mut digplan = Vec::new();
  std::io::stdin().lines().map(Result::unwrap).for_each(|line| {
    let mut s = line.split_ascii_whitespace();
    let cmd = s.next().unwrap().chars().next().unwrap();
    let meters = s.next().unwrap().parse::<isize>().unwrap();
    let color = s
      .next()
      .unwrap()
      .trim_matches(|c| c == '(' || c == ')' || c == '#')
      .to_owned();
    digplan.push((cmd, meters, color));
  });

  let mut digger = (0isize, 0isize);
  let mut min = (0isize, 0isize);
  let mut max = (0isize, 0isize);
  let mut trench = Vec::new();
  let mut prev = None;
  let mut lr = (0, 0);
  trench.push(digger);
  for (cmd, meters, _) in digplan {
    match cmd {
      'U' => {
        match prev {
          Some('L') => lr.0 += 1,
          Some('R') => lr.1 += 1,
          None => (),
          _ => unreachable!(),
        }
        digger.0 -= meters;
      }
      'D' => {
        match prev {
          Some('R') => lr.0 += 1,
          Some('L') => lr.1 += 1,
          None => (),
          _ => unreachable!(),
        }
        digger.0 += meters;
      }
      'L' => {
        match prev {
          Some('D') => lr.0 += 1,
          Some('U') => lr.1 += 1,
          None => (),
          _ => unreachable!(),
        }
        digger.1 -= meters;
      }
      'R' => {
        match prev {
          Some('U') => lr.0 += 1,
          Some('D') => lr.1 += 1,
          None => (),
          _ => unreachable!(),
        }
        digger.1 += meters;
      }
      _ => unreachable!(),
    }
    if min.0 > digger.0 {
      min.0 = digger.0;
    }
    if max.0 < digger.0 {
      max.0 = digger.0;
    }
    if min.1 > digger.1 {
      min.1 = digger.1;
    }
    if max.1 < digger.1 {
      max.1 = digger.1;
    }
    prev = Some(cmd);
    trench.push(digger);
  }
  assert_eq!((0, 0), *trench.last().unwrap());

  let rows = (1 + max.0 - min.0) as usize;
  let cols = (1 + max.1 - min.1) as usize;
  let trench = trench
    .into_iter()
    .map(|(row, col)| {
      let row = (row - min.0) as usize;
      let col = (col - min.1) as usize;
      (row, col)
    })
    .collect::<Vec<_>>();

  // TODO: try segment tree with U,D lines?
  let mut ground = vec![vec!['.'; cols]; rows];
  trench.windows(2).for_each(|w| {
    let [from, to] = w else {
      unreachable!();
    };
    line(&mut ground, *from, *to);
  });

  let left_is_inside = lr.0 < lr.1;

  let mut area = 0;
  for r in 0..rows {
    let mut inside = false;
    let mut prev = '?';
    for c in 0..cols {
      let next = ground[r][c];
      match next {
        '.' if inside => {
          area += 1;
          ground[r][c] = 'x';
        }
        'U' => {
          area += 1;
          inside = !left_is_inside
        }
        'D' => {
          area += 1;
          inside = left_is_inside
        }
        'R' | 'L' => area += 1,
        _ => (),
      }
      if prev != next {
        prev = next;
      }
    }
  }

  eprintln!("{area}");
}

fn line(ground: &mut Vec<Vec<char>>, from: (usize, usize), to: (usize, usize)) {
  // TODO: L, R not needed
  if from.0 == to.0 {
    if from.1 < to.1 {
      for c in from.1 + 1..=to.1 {
        ground[from.0][c] = 'R';
      }
    } else if from.1 > to.1 {
      for c in to.1..from.1 {
        ground[from.0][c] = 'L';
      }
    } else {
      unreachable!()
    }
  } else if from.1 == to.1 {
    if from.0 < to.0 {
      for r in from.0..=to.0 {
        ground[r][from.1] = 'D';
      }
    } else if from.0 > to.0 {
      for r in to.0..=from.0 {
        ground[r][from.1] = 'U';
      }
    } else {
      unreachable!()
    }
  } else {
    unreachable!()
  }
}
