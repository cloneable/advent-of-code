fn main() {
  let grid = Grid(
    std::io::stdin()
      .lines()
      .map(Result::unwrap)
      .map(|l| l.bytes().map(Tile::from).collect::<Vec<_>>())
      .collect::<Vec<_>>(),
  );

  let (start_pos, start_pipe) = grid.find_start();

  let mut steps = 0;
  let mut cur_pos = start_pos;
  let mut cur_pipe = start_pipe;
  loop {
    steps += 1;

    let next_pos = cur_pos.move_in(cur_pipe.to).unwrap();
    if next_pos == start_pos {
      break;
    }
    let next_pipe =
      grid.pipe_at(next_pos).unwrap().align(cur_pipe.to.reverse()).unwrap();

    cur_pos = next_pos;
    cur_pipe = next_pipe;
  }

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

  fn find_start(&self) -> (Pos, Pipe) {
    let bounds = self.bounds();
    for r in 0..bounds.rows {
      for c in 0..bounds.cols {
        if self.0[r][c] == Tile::Start {
          let pos = (r, c, bounds).into();
          let pipe = self.infer_pipe(pos);
          return (pos, pipe);
        }
      }
    }
    unreachable!()
  }

  fn infer_pipe(&self, pos: Pos) -> Pipe {
    let dirs = [Dir::N, Dir::S, Dir::E, Dir::W]
      .into_iter()
      .filter_map(|dir| pos.move_in(dir).and_then(|apos| Some((apos, dir))))
      .filter_map(|(apos, dir)| {
        self.pipe_at(apos).and_then(|pipe| Some((pipe, dir)))
      })
      .filter_map(|(pipe, dir)| pipe.align(dir.reverse()).and(Some(dir)))
      .collect::<Vec<_>>();

    assert_eq!(2, dirs.len());

    Pipe::new(dirs[0], dirs[1])
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
    assert_ne!(from, to);
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
