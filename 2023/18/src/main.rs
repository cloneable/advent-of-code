use std::cmp::Ordering;

fn main() {
  let mut digplan = Vec::new();
  digplan.push(Default::default());
  std::io::stdin().lines().map(Result::unwrap).for_each(|line| {
    let mut s = line.split_ascii_whitespace();
    let _ = s.next().unwrap();
    let _ = s.next().unwrap();

    let hex = s
      .next()
      .unwrap()
      .trim_matches(|c| c == '(' || c == ')' || c == '#')
      .to_owned();
    let meters = isize::from_str_radix(&hex[0..5], 16).unwrap();
    let cmd = match hex.as_bytes()[5] {
      b'0' => 'R',
      b'1' => 'D',
      b'2' => 'L',
      b'3' => 'U',
      _ => unreachable!(),
    };

    digplan.push((cmd, meters));
  });
  digplan[0] = *digplan.last().unwrap();
  digplan.push(digplan[1]);

  let mut digger = (0isize, 0isize);
  let mut hsegments = Vec::new();
  let mut lr = (0usize, 0usize);

  let mut digs = digplan.windows(3);
  while let Some([(prev_cmd, _), (cmd, meters), (next_cmd, _)]) = digs.next() {
    let old_pos = digger;
    match cmd {
      'U' => digger.0 -= meters,
      'D' => digger.0 += meters,

      'L' => {
        digger.1 -= meters;
        let adjust = match (prev_cmd, next_cmd) {
          ('U', 'D') => -1,
          ('U', 'U') => 0,
          ('D', 'D') => 0,
          ('D', 'U') => 1,
          _ => unreachable!(),
        };
        hsegments.push(Segment::new(digger, old_pos, false, adjust));
      }

      'R' => {
        digger.1 += meters;
        let adjust = match (prev_cmd, next_cmd) {
          ('D', 'U') => -1,
          ('U', 'U') => 0,
          ('D', 'D') => 0,
          ('U', 'D') => 1,
          _ => unreachable!(),
        };
        hsegments.push(Segment::new(old_pos, digger, true, adjust));
      }

      _ => (),
    }

    match (prev_cmd, cmd) {
      ('L', 'U') => lr.0 += 1,
      ('R', 'U') => lr.1 += 1,

      ('R', 'D') => lr.0 += 1,
      ('L', 'D') => lr.1 += 1,

      ('D', 'L') => lr.0 += 1,
      ('U', 'L') => lr.1 += 1,

      ('U', 'R') => lr.0 += 1,
      ('D', 'R') => lr.1 += 1,

      _ => unreachable!(),
    }
  }
  assert_eq!((0, 0), digger);

  let left_is_inside = lr.0 < lr.1;
  if !left_is_inside {
    for seg in hsegments.iter_mut() {
      seg.subtract = !seg.subtract;
    }
  }

  hsegments.sort_unstable();

  let mut row = hsegments.first().unwrap().row;
  let mut area = 0usize;
  let mut inside = 0usize;
  for seg in hsegments {
    assert!(row <= seg.row);

    if row != seg.row {
      area += (seg.row - row) as usize * inside;
      row = seg.row;
    }

    if seg.subtract {
      inside -= seg.len;
      area += seg.len; // re-add for current row
    } else {
      inside += seg.len;
    }
  }

  eprintln!("{area}",);
}

#[derive(PartialEq, Eq)]
struct Segment {
  row: isize,
  subtract: bool,
  len: usize,
}

impl Segment {
  fn new(
    from: (isize, isize),
    to: (isize, isize),
    subtract: bool,
    adjust: isize,
  ) -> Self {
    assert_eq!(from.0, to.0);
    let row = from.0;
    let len = (from.1.max(to.1) - from.1.min(to.1) + adjust) as usize;
    Segment { row, len, subtract }
  }
}

impl Ord for Segment {
  fn cmp(&self, other: &Self) -> Ordering {
    (self.row, self.subtract, self.len).cmp(&(
      other.row,
      other.subtract,
      other.len,
    ))
  }
}

impl PartialOrd for Segment {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
