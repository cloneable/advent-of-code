use std::collections::VecDeque;

fn main() {
  let tiles = std::io::stdin()
    .lines()
    .map(Result::unwrap)
    .map(|line| line.chars().collect())
    .collect::<Vec<Vec<char>>>();

  let rows = tiles.len();
  let cols = tiles[0].len();

  let mut max = 0;

  for r in 0..rows {
    let e = fire_beam(
      &tiles,
      rows as isize,
      cols as isize,
      Beam { pos: (r as isize, -1), dir: Dir::E },
    );
    if e >= max {
      max = e;
    }

    let e = fire_beam(
      &tiles,
      rows as isize,
      cols as isize,
      Beam { pos: (r as isize, cols as isize), dir: Dir::W },
    );
    if e >= max {
      max = e;
    }
  }

  for c in 0..cols {
    let e = fire_beam(
      &tiles,
      rows as isize,
      cols as isize,
      Beam { pos: (-1, c as isize), dir: Dir::S },
    );
    if e >= max {
      max = e;
    }

    let e = fire_beam(
      &tiles,
      rows as isize,
      cols as isize,
      Beam { pos: (rows as isize, c as isize), dir: Dir::N },
    );
    if e >= max {
      max = e;
    }
  }

  println!("{max}");
}

fn fire_beam(
  tiles: &Vec<Vec<char>>,
  rows: isize,
  cols: isize,
  beam: Beam,
) -> usize {
  let mut energized = vec![vec![0u8; cols as usize]; rows as usize];

  let mut beams = VecDeque::new();
  beams.push_back(beam);
  while !beams.is_empty() {
    let mut beam = beams.pop_front().unwrap();
    let (active, sibling) = beam.step(&tiles, rows, cols, &mut energized);
    if active {
      beams.push_back(beam);
    }
    if let Some(sibling) = sibling {
      beams.push_back(sibling);
    }
  }

  energized
    .into_iter()
    .fold(0, |ar, r| ar + r.into_iter().fold(0, |ac, e| ac + (e != 0) as usize))
}

#[derive(Debug)]
struct Beam {
  pos: (isize, isize),
  dir: Dir,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum Dir {
  N = 8,
  E = 4,
  S = 2,
  W = 1,
}

impl Beam {
  fn step(
    &mut self,
    tiles: &Vec<Vec<char>>,
    rows: isize,
    cols: isize,
    energized: &mut Vec<Vec<u8>>,
  ) -> (bool, Option<Beam>) {
    let (Some(new_row), Some(new_col)) = (match self.dir {
      Dir::N => (checked_dec(self.pos.0), Some(self.pos.1)),
      Dir::W => (Some(self.pos.0), checked_dec(self.pos.1)),
      Dir::S => (checked_inc(self.pos.0, rows), Some(self.pos.1)),
      Dir::E => (Some(self.pos.0), checked_inc(self.pos.1, cols)),
    }) else {
      return (false, None);
    };

    self.pos = (new_row, new_col);

    let e = energized[self.pos.0 as usize][self.pos.1 as usize];
    if (e & self.dir as u8) != 0 {
      return (false, None);
    } else {
      energized[self.pos.0 as usize][self.pos.1 as usize] |= self.dir as u8;
    }

    match tiles[self.pos.0 as usize][self.pos.1 as usize] {
      '.' => (),

      '/' => {
        self.dir = match self.dir {
          Dir::N => Dir::E,
          Dir::W => Dir::S,
          Dir::S => Dir::W,
          Dir::E => Dir::N,
        };
      }
      '\\' => {
        self.dir = match self.dir {
          Dir::N => Dir::W,
          Dir::W => Dir::N,
          Dir::S => Dir::E,
          Dir::E => Dir::S,
        };
      }

      '|' => {
        match self.dir {
          Dir::W | Dir::E => {
            let sibling = Beam { pos: self.pos, dir: Dir::N };
            self.dir = Dir::S;
            return (true, Some(sibling));
          }
          Dir::N | Dir::S => (),
        };
      }
      '-' => {
        match self.dir {
          Dir::N | Dir::S => {
            let sibling = Beam { pos: self.pos, dir: Dir::W };
            self.dir = Dir::E;
            return (true, Some(sibling));
          }
          Dir::W | Dir::E => (),
        };
      }

      _ => unreachable!(),
    }

    (true, None)
  }
}

fn checked_dec(v: isize) -> Option<isize> {
  v.checked_sub(1).filter(|&v| v >= 0)
}

fn checked_inc(v: isize, bound: isize) -> Option<isize> {
  v.checked_add(1).filter(|&v| v < bound)
}
