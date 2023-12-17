fn main() {
  let platform = std::io::stdin()
    .lines()
    .map(Result::unwrap)
    .map(|line| line.chars().collect())
    .collect::<Vec<Vec<char>>>();

  let rows = platform.len() as u8;
  let cols = platform[0].len() as u8;

  let mut rocks = Vec::new();
  for r in 0..rows {
    for c in 0..cols {
      match platform[r as usize][c as usize] {
        'O' => rocks.push(Rock { r, c, moving: true }),
        '#' => rocks.push(Rock { r, c, moving: false }),
        '.' => (),
        _ => unreachable!(),
      }
    }
  }

  let mut positions = Vec::<(usize, Vec<Rock>)>::new();
  let mut period_length = 0;
  let mut period_start = 0;
  'outer: for cycle in 1.. {
    tilt_north(&mut rocks, rows, cols);
    tilt_west(&mut rocks, rows, cols);
    tilt_south(&mut rocks, rows, cols);
    tilt_east(&mut rocks, rows, cols);

    for pos in positions.iter().rev() {
      if rocks == pos.1 {
        period_length = cycle - pos.0;
        period_start = pos.0;
        break 'outer;
      }
    }

    positions.push((cycle, rocks.clone()));
  }

  let target_cycle =
    (1_000_000_000 - period_start) % period_length + period_start;

  let target_position = &positions[target_cycle - 1];

  println!("{}", calc_load(&target_position.1, rows as usize));
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Rock {
  r: u8,
  c: u8,
  moving: bool,
}

fn tilt_north(rocks: &mut Vec<Rock>, _rows: u8, _cols: u8) {
  rocks.sort_unstable_by(|a, b| (a.c, a.r).cmp(&(b.c, b.r)));
  for i in 0..rocks.len() {
    if rocks[i].moving {
      if i != 0 && rocks[i - 1].c == rocks[i].c {
        rocks[i].r = rocks[i - 1].r + 1;
      } else {
        rocks[i].r = 0;
      }
    }
  }
}

fn tilt_west(rocks: &mut Vec<Rock>, _rows: u8, _cols: u8) {
  rocks.sort_unstable_by(|a, b| (a.r, a.c).cmp(&(b.r, b.c)));
  for i in 0..rocks.len() {
    if rocks[i].moving {
      if i != 0 && rocks[i - 1].r == rocks[i].r {
        rocks[i].c = rocks[i - 1].c + 1;
      } else {
        rocks[i].c = 0;
      }
    }
  }
}

fn tilt_south(rocks: &mut Vec<Rock>, rows: u8, _cols: u8) {
  rocks.sort_unstable_by(|a, b| (a.c, b.r).cmp(&(b.c, a.r)));
  for i in 0..rocks.len() {
    if rocks[i].moving {
      if i != 0 && rocks[i - 1].c == rocks[i].c {
        rocks[i].r = rocks[i - 1].r - 1;
      } else {
        rocks[i].r = rows - 1;
      }
    }
  }
}

fn tilt_east(rocks: &mut Vec<Rock>, _rows: u8, cols: u8) {
  rocks.sort_unstable_by(|a, b| (a.r, b.c).cmp(&(b.r, a.c)));
  for i in 0..rocks.len() {
    if rocks[i].moving {
      if i != 0 && rocks[i - 1].r == rocks[i].r {
        rocks[i].c = rocks[i - 1].c - 1;
      } else {
        rocks[i].c = cols - 1;
      }
    }
  }
}

fn calc_load(rocks: &[Rock], rows: usize) -> usize {
  let mut load = 0;
  for rock in rocks {
    if rock.moving {
      load += rows - rock.r as usize;
    }
  }
  load
}
