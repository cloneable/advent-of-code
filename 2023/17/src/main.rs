use priority_queue::PriorityQueue;
use std::{cmp::Ordering, collections::HashMap, hash::Hash, ops::Add};

fn main() {
  let map = std::io::stdin()
    .lines()
    .map(Result::unwrap)
    .map(|line| line.bytes().map(|b| HeatLoss((b - b'0') as usize)).collect())
    .collect::<Vec<Vec<HeatLoss>>>();
  let rows = map.len();
  let cols = map[0].len();

  let city = City { map, rows, cols };
  let lavafall = Block { row: 0, col: 0 };
  let factory = Block { row: rows - 1, col: cols - 1 };

  let path = city.route(lavafall, factory).unwrap();
  let total_loss = path.iter().fold(0, |total, (_, loss)| total + loss.0);

  println!("{total_loss}");
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct HeatLoss(usize);

impl HeatLoss {
  const ZERO: Self = HeatLoss(0);
  const MAX: Self = HeatLoss(usize::MAX);
}

impl Ord for HeatLoss {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.cmp(&other.0).reverse()
  }
}

impl PartialOrd for HeatLoss {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Add for HeatLoss {
  type Output = HeatLoss;

  fn add(self, rhs: Self) -> Self::Output {
    HeatLoss(self.0 + rhs.0)
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Block {
  row: usize,
  col: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Edge {
  N,
  W,
  S,
  E,
}

impl Edge {
  fn reverse(self) -> Self {
    match self {
      Edge::N => Edge::S,
      Edge::S => Edge::N,
      Edge::W => Edge::E,
      Edge::E => Edge::W,
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Node {
  block: Block,
  edge: Option<(Edge, usize)>,
}

#[derive(Debug)]
struct City {
  map: Vec<Vec<HeatLoss>>,
  rows: usize,
  cols: usize,
}

impl City {
  fn route(&self, start: Block, dest: Block) -> Option<Vec<(Node, HeatLoss)>> {
    let mut backtraces = HashMap::<Node, Node>::new();

    let start = Node { block: start, edge: None };
    let mut queue = PriorityQueue::<Node, HeatLoss>::new();
    queue.push(start, Self::distance(start.block, dest));

    let mut min_losses = HashMap::<Node, HeatLoss>::new();
    min_losses.insert(start, HeatLoss::ZERO);

    while let Some((cur, _)) = queue.pop() {
      if cur.block == dest {
        return Some(self.backtrace(&backtraces, cur));
      }
      let cur_min = min_losses[&cur];
      for (adj, adj_loss) in self.adjacent_nodes(cur, &backtraces) {
        let adj_min = *min_losses.get(&adj).unwrap_or(&HeatLoss::MAX);
        let new_adj_min = cur_min + adj_loss;
        if new_adj_min.0 < adj_min.0 {
          min_losses.insert(adj, new_adj_min);
          queue.push(adj, new_adj_min + Self::distance(adj.block, dest));
          backtraces.insert(adj, cur);
        }
      }
    }

    None
  }

  fn go(
    &self,
    node: Node,
    dir: Edge,
    c: usize,
    steps: usize,
  ) -> Option<(Node, HeatLoss)> {
    match dir {
      Edge::N if node.block.row >= steps => {
        let block = Block { row: node.block.row - steps, col: node.block.col };
        Some((
          Node { block, edge: Some((dir.reverse(), c)) },
          self.heat_loss_range(node.block, block),
        ))
      }
      Edge::W if node.block.col >= steps => {
        let block = Block { row: node.block.row, col: node.block.col - steps };
        Some((
          Node { block, edge: Some((dir.reverse(), c)) },
          self.heat_loss_range(node.block, block),
        ))
      }
      Edge::S if node.block.row + steps < self.rows => {
        let block = Block { row: node.block.row + steps, col: node.block.col };
        Some((
          Node { block, edge: Some((dir.reverse(), c)) },
          self.heat_loss_range(node.block, block),
        ))
      }
      Edge::E if node.block.col + steps < self.cols => {
        let block = Block { row: node.block.row, col: node.block.col + steps };
        Some((
          Node { block, edge: Some((dir.reverse(), c)) },
          self.heat_loss_range(node.block, block),
        ))
      }
      _ => None,
    }
  }

  fn adjacent_nodes(
    &self,
    node: Node,
    _backtraces: &HashMap<Node, Node>,
  ) -> Vec<(Node, HeatLoss)> {
    let mut nodes = Vec::new();
    match node.edge {
      Some((Edge::N, n)) => {
        if n < 10 {
          self
            .go(node, Edge::S, n + 1, 1)
            .and_then(|adj| Some(nodes.push(adj)));
        }
        self.go(node, Edge::W, 4, 4).and_then(|adj| Some(nodes.push(adj)));
        self.go(node, Edge::E, 4, 4).and_then(|adj| Some(nodes.push(adj)));
      }
      Some((Edge::S, n)) => {
        if n < 10 {
          self
            .go(node, Edge::N, n + 1, 1)
            .and_then(|adj| Some(nodes.push(adj)));
        }
        self.go(node, Edge::E, 4, 4).and_then(|adj| Some(nodes.push(adj)));
        self.go(node, Edge::W, 4, 4).and_then(|adj| Some(nodes.push(adj)));
      }
      Some((Edge::W, n)) => {
        if n < 10 {
          self
            .go(node, Edge::E, n + 1, 1)
            .and_then(|adj| Some(nodes.push(adj)));
        }
        self.go(node, Edge::S, 4, 4).and_then(|adj| Some(nodes.push(adj)));
        self.go(node, Edge::N, 4, 4).and_then(|adj| Some(nodes.push(adj)));
      }
      Some((Edge::E, n)) => {
        if n < 10 {
          self
            .go(node, Edge::W, n + 1, 1)
            .and_then(|adj| Some(nodes.push(adj)));
        }
        self.go(node, Edge::N, 4, 4).and_then(|adj| Some(nodes.push(adj)));
        self.go(node, Edge::S, 4, 4).and_then(|adj| Some(nodes.push(adj)));
      }
      None => {
        self.go(node, Edge::N, 4, 4).and_then(|adj| Some(nodes.push(adj)));
        self.go(node, Edge::W, 4, 4).and_then(|adj| Some(nodes.push(adj)));
        self.go(node, Edge::S, 4, 4).and_then(|adj| Some(nodes.push(adj)));
        self.go(node, Edge::E, 4, 4).and_then(|adj| Some(nodes.push(adj)));
      }
    }
    nodes
  }

  fn distance(a: Block, b: Block) -> HeatLoss {
    let r0 = a.row.min(b.row);
    let r1 = a.row.max(b.row);
    let c0 = a.col.min(b.col);
    let c1 = a.col.max(b.col);

    HeatLoss((r1 - r0) + (c1 - c0))
  }

  fn heat_loss_range(&self, a: Block, b: Block) -> HeatLoss {
    let r0 = a.row.min(b.row);
    let r1 = a.row.max(b.row);
    let c0 = a.col.min(b.col);
    let c1 = a.col.max(b.col);

    let mut loss = 0;
    for r in r0..=r1 {
      for c in c0..=c1 {
        loss += self.map[r][c].0;
      }
    }
    loss -= self.map[a.row][a.col].0;

    assert_ne!(0, loss);
    HeatLoss(loss)
  }

  fn backtrace(
    &self,
    backtraces: &HashMap<Node, Node>,
    mut node: Node,
  ) -> Vec<(Node, HeatLoss)> {
    let mut nodes = Vec::new();
    while let Some(prev) = backtraces.get(&node) {
      let loss = self.heat_loss_range(prev.block, node.block);
      nodes.push((*prev, loss));
      node = *prev;
    }
    nodes.reverse();
    nodes
  }
}
