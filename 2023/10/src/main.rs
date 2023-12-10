fn main() {
  let grid = Grid(
    std::io::stdin()
      .lines()
      .map(Result::unwrap)
      .map(|l| l.bytes().map(Tile::from).collect::<Vec<_>>())
      .collect::<Vec<_>>(),
  );

  let mut markers = MarkerGrid::new(&grid);

  let (start_pos, start_pipe) = grid.find_start();

  let mut cur_pos = start_pos;
  let mut cur_pipe = start_pipe;
  let mut turns = (0isize, 0isize);
  loop {
    markers.mark(cur_pos, cur_pipe);
    match cur_pipe.turn() {
      Some(Turn::Left) => turns.0 += 1,
      Some(Turn::Right) => turns.1 += 1,
      _ => (),
    }

    let next_pos = cur_pos.move_in(cur_pipe.to).unwrap();
    if next_pos == start_pos {
      break;
    }
    let next_pipe =
      grid.pipe_at(next_pos).unwrap().align(cur_pipe.to.reverse()).unwrap();

    cur_pos = next_pos;
    cur_pipe = next_pipe;
  }

  let inner_side = if turns.0 > turns.1 { Side::Left } else { Side::Right };
  let mut inside = false;
  let mut count = 0;

  for cols in markers.0 {
    for &m in &cols {
      match m {
        Marker::Unknown if inside => count += 1,
        Marker::Unknown => (),
        Marker::Path(_) => inside = false,
        Marker::Side(side) => {
          if side == inner_side {
            inside = true;
            count += 1;
          } else {
            inside = false;
          }
        }
      }
    }
  }

  println!("{count}");
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
enum Side {
  Left,
  Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Marker {
  Unknown,
  Path(Pipe),
  Side(Side),
}

impl Marker {
  fn set(&mut self, target: Side) {
    if *self == Marker::Unknown {
      *self = Marker::Side(target);
    }
  }
}

struct MarkerGrid(Vec<Vec<Marker>>);

impl MarkerGrid {
  fn new(grid: &Grid) -> Self {
    let bounds = grid.bounds();
    MarkerGrid(vec![vec![Marker::Unknown; bounds.cols]; bounds.rows])
  }

  fn mark(&mut self, pos: Pos, pipe: Pipe) {
    self.0[pos.row][pos.col] = Marker::Path(pipe);
    use Dir as D;
    match (pipe.from, pipe.to) {
      (D::N, D::S) => self.mark_row(pos, Side::Right, Side::Left),
      (D::N, D::E) => self.mark_bent(pos, Bent::SW, Side::Right),
      (D::N, D::W) => self.mark_bent(pos, Bent::SE, Side::Left),

      (D::S, D::N) => self.mark_row(pos, Side::Left, Side::Right),
      (D::S, D::E) => self.mark_bent(pos, Bent::NW, Side::Left),
      (D::S, D::W) => self.mark_bent(pos, Bent::NE, Side::Right),

      (D::E, D::N) => self.mark_bent(pos, Bent::SW, Side::Left),
      (D::E, D::S) => self.mark_bent(pos, Bent::NW, Side::Right),
      (D::E, D::W) => self.mark_col(pos, Side::Right, Side::Left),

      (D::W, D::N) => self.mark_bent(pos, Bent::SE, Side::Right),
      (D::W, D::S) => self.mark_bent(pos, Bent::NE, Side::Left),
      (D::W, D::E) => self.mark_col(pos, Side::Left, Side::Right),

      _ => {
        unreachable!()
      }
    }
  }

  fn mark_bent(&mut self, pos: Pos, q: Bent, s: Side) {
    match q {
      Bent::NW => {
        let r_start = pos.row.saturating_sub(1);
        let r_end = pos.row;
        let rows = (r_start..=r_end).rev();

        let c_start = pos.col.saturating_sub(1);
        let c_end = pos.col;
        let cols = (c_start..=c_end).rev();

        for r in rows {
          for c in cols.clone() {
            if !(r == pos.row && c == pos.col) {
              self.0[r][c].set(s);
            }
          }
        }
      }
      Bent::NE => {
        let r_start = pos.row.saturating_sub(1);
        let r_end = pos.row;
        let rows = (r_start..=r_end).rev();

        let c_start = pos.col;
        let c_end = (pos.col + 2).min(pos.bounds.cols);
        let cols = c_start..c_end;

        for r in rows {
          for c in cols.clone() {
            if !(r == pos.row && c == pos.col) {
              self.0[r][c].set(s);
            }
          }
        }
      }
      Bent::SW => {
        let r_start = pos.row;
        let r_end = (pos.row + 2).min(pos.bounds.rows);
        let rows = r_start..r_end;

        let c_start = pos.col.saturating_sub(1);
        let c_end = pos.col;
        let cols = (c_start..=c_end).rev();

        for r in rows {
          for c in cols.clone() {
            if !(r == pos.row && c == pos.col) {
              self.0[r][c].set(s);
            }
          }
        }
      }
      Bent::SE => {
        let r_start = pos.row;
        let r_end = (pos.row + 2).min(pos.bounds.rows);
        let rows = r_start..r_end;

        let c_start = pos.col;
        let c_end = (pos.col + 2).min(pos.bounds.cols);
        let cols = c_start..c_end;

        for r in rows {
          for c in cols.clone() {
            if !(r == pos.row && c == pos.col) {
              self.0[r][c].set(s);
            }
          }
        }
      }
    }
  }

  fn mark_row(&mut self, pos: Pos, west: Side, east: Side) {
    let start = pos.col.saturating_sub(1);
    let end = pos.col;
    for c in (start..end).rev() {
      self.0[pos.row][c].set(west);
    }
    let start = pos.col + 1;
    let end = (pos.col + 2).min(pos.bounds.cols);
    for c in start..end {
      self.0[pos.row][c].set(east);
    }
  }

  fn mark_col(&mut self, pos: Pos, north: Side, south: Side) {
    let start = pos.row.saturating_sub(1);
    let end = pos.row;
    for r in (start..end).rev() {
      self.0[r][pos.col].set(north);
    }
    let start = pos.row + 1;
    let end = (pos.row + 2).min(pos.bounds.rows);
    for r in start..end {
      self.0[r][pos.col].set(south);
    }
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
enum Turn {
  Left,
  Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Bent {
  NE,
  NW,
  SE,
  SW,
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

  fn turn(self) -> Option<Turn> {
    use Dir as D;
    match (self.from, self.to) {
      (D::N, D::S) => None,
      (D::N, D::E) => Some(Turn::Left),
      (D::N, D::W) => Some(Turn::Right),

      (D::S, D::N) => None,
      (D::S, D::W) => Some(Turn::Left),
      (D::S, D::E) => Some(Turn::Right),

      (D::E, D::N) => Some(Turn::Right),
      (D::E, D::S) => Some(Turn::Left),
      (D::E, D::W) => None,

      (D::W, D::N) => Some(Turn::Left),
      (D::W, D::S) => Some(Turn::Right),
      (D::W, D::E) => None,

      (D::N, D::N) => unreachable!(),
      (D::S, D::S) => unreachable!(),
      (D::W, D::W) => unreachable!(),
      (D::E, D::E) => unreachable!(),
    }
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
