use itertools::Itertools;

fn main() {
  let grid = Grid(
    std::io::stdin()
      .lines()
      .map(Result::unwrap)
      .map(|l| l.bytes().map(Tile::from).collect::<Vec<_>>())
      .collect::<Vec<_>>(),
  );

  let start_pos = grid.find_start();
  eprintln!("{start_pos:?}");

  let start_pipes = start_pos
    .adjacent()
    .filter_map(|pos| grid.pipe_at(pos).and_then(|pipe| Some((pos, pipe))))
    .filter_map(|(pos, pipe)| pos.connects(pipe, start_pos))
    .combinations(2)
    .map(|v| Pipe::new(v[0].min(v[1]), v[0].max(v[1])))
    .sorted_unstable()
    .unique()
    .collect::<Vec<Pipe>>();
  eprintln!("{start_pipes:?}");

  let mut steps = 0;
  'outer: for mut cur_pipe in start_pipes {
    steps = 0;

    let mut cur_pos = start_pos;
    loop {
      steps += 1;

      let Some(next_pos) = cur_pos.move_in(cur_pipe.to) else {
        break;
      };
      let Some(next_pipe) = grid.pipe_at(next_pos) else {
        break;
      };
      let Some(next_pipe) = next_pipe.align(cur_pipe.to.reverse()) else {
        break;
      };

      cur_pos = next_pos;
      cur_pipe = next_pipe;

      if cur_pos == start_pos {
        break 'outer;
      }
    }
  }
  eprintln!("{steps:?}");

  let steps = (steps + 1) / 2;

  println!("{steps}");
}

struct Grid(Vec<Vec<Tile>>);

impl Grid {
  fn pipe_at(&self, pos: Pos) -> Option<Pipe> {
    match self.0[pos.row][pos.col] {
      Tile::Pipe(pipe) => Some(pipe),
      _ => None,
    }
  }

  fn bounds(&self) -> Bounds {
    Bounds { rows: self.0.len(), cols: self.0[0].len() }
  }

  fn find_start(&self) -> Pos {
    let bounds = self.bounds();
    let mut starts = Vec::<Pos>::new();
    for r in 0..bounds.rows {
      for c in 0..bounds.cols {
        if self.0[r][c] == Tile::Start {
          starts.push((r, c, bounds).into());
        }
      }
    }
    assert_eq!(1, starts.len());
    starts[0]
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Bounds {
  rows: usize,
  cols: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
  row: usize,
  col: usize,
  bounds: Bounds,
}

impl From<(usize, usize, Bounds)> for Pos {
  fn from((row, col, bounds): (usize, usize, Bounds)) -> Self {
    Pos { row, col, bounds }
  }
}

impl Pos {
  fn adjacent(self) -> impl Iterator<Item = Pos> {
    AdjacencyIter { pos: self, cur: 0 }
  }

  fn move_in(self, dir: Dir) -> Option<Self> {
    match dir {
      Dir::N if self.row > 0 => {
        return Some(Pos {
          row: self.row - 1,
          col: self.col,
          bounds: self.bounds,
        });
      }
      Dir::S if self.row + 1 < self.bounds.rows => {
        return Some(Pos {
          row: self.row + 1,
          col: self.col,
          bounds: self.bounds,
        });
      }
      Dir::W if self.col > 0 => {
        return Some(Pos {
          row: self.row,
          col: self.col - 1,
          bounds: self.bounds,
        });
      }
      Dir::E if self.col + 1 < self.bounds.cols => {
        return Some(Pos {
          row: self.row,
          col: self.col + 1,
          bounds: self.bounds,
        });
      }
      _ => None,
    }
  }

  fn connects(self, pipe: Pipe, target: Pos) -> Option<Dir> {
    if self.move_in(pipe.from) == Some(target) {
      Some(pipe.from.reverse())
    } else if self.move_in(pipe.to) == Some(target) {
      Some(pipe.to.reverse())
    } else {
      None
    }
  }
}

struct AdjacencyIter {
  pos: Pos,
  cur: usize,
}

impl Iterator for AdjacencyIter {
  type Item = Pos;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      self.cur += 1;
      match self.cur {
        1 => {
          if self.pos.row > 0 {
            return Some(
              (self.pos.row - 1, self.pos.col, self.pos.bounds).into(),
            );
          }
        }
        2 => {
          if self.pos.col > 0 {
            return Some(
              (self.pos.row - 1, self.pos.col, self.pos.bounds).into(),
            );
          }
        }
        3 => {
          if self.pos.col + 1 < self.pos.bounds.cols {
            return Some(
              (self.pos.row, self.pos.col + 1, self.pos.bounds).into(),
            );
          }
        }
        4 => {
          if self.pos.row + 1 < self.pos.bounds.rows {
            return Some(
              (self.pos.row + 1, self.pos.col, self.pos.bounds).into(),
            );
          }
        }
        _ => return None,
      }
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
  N,
  S,
  E,
  W,
}

impl Dir {
  fn reverse(self) -> Self {
    match self {
      Dir::N => Dir::S,
      Dir::S => Dir::N,
      Dir::E => Dir::W,
      Dir::W => Dir::E,
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
  Ground,
  Start,
  Pipe(Pipe),
}

impl From<u8> for Tile {
  fn from(value: u8) -> Self {
    match value {
      b'.' => Tile::Ground,
      b'S' => Tile::Start,
      b'|' => Tile::Pipe(Pipe::new(Dir::N, Dir::S)),
      b'-' => Tile::Pipe(Pipe::new(Dir::E, Dir::W)),
      b'L' => Tile::Pipe(Pipe::new(Dir::N, Dir::E)),
      b'J' => Tile::Pipe(Pipe::new(Dir::N, Dir::W)),
      b'F' => Tile::Pipe(Pipe::new(Dir::S, Dir::E)),
      b'7' => Tile::Pipe(Pipe::new(Dir::S, Dir::W)),
      _ => unreachable!("{value}"),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pipe {
  from: Dir,
  to: Dir,
}

impl Pipe {
  fn new(from: Dir, to: Dir) -> Self {
    Pipe { from, to }
  }

  fn align(self, from: Dir) -> Option<Self> {
    if self.from == from {
      Some(self)
    } else if self.to == from {
      Some(self.reverse())
    } else {
      None
    }
  }

  fn reverse(self) -> Self {
    Pipe { to: self.from, from: self.to }
  }
}
