use indexmap::IndexMap;
use nom::{bytes::complete::tag, bytes::complete::take_while_m_n, IResult};

fn parse_node(input: &str) -> IResult<&str, (String, (String, String))> {
  let (input, name) =
    take_while_m_n(1, 3, |c: char| c.is_ascii_alphanumeric())(input)?;
  let (input, _) = tag(" = (")(input)?;
  let (input, left) =
    take_while_m_n(1, 3, |c: char| c.is_ascii_alphanumeric())(input)?;
  let (input, _) = tag(", ")(input)?;
  let (input, right) =
    take_while_m_n(1, 3, |c: char| c.is_ascii_alphanumeric())(input)?;
  let (input, _) = tag(")")(input)?;

  Ok((input, (name.to_owned(), (left.to_owned(), right.to_owned()))))
}

fn main() {
  let mut lines = std::io::stdin().lines();

  let instructions = lines
    .next()
    .unwrap()
    .unwrap()
    .into_bytes()
    .into_iter()
    .map(|b| b == b'L')
    .collect::<Vec<_>>();
  let _ = lines.next().unwrap().unwrap();

  let mut all_nodes = IndexMap::<String, (usize, usize, bool)>::new();
  let mut nodes = Vec::<usize>::new();
  for line in lines {
    let line = line.unwrap();
    let (_, (node, (left, right))) = parse_node(&line).unwrap();

    let left = {
      let entry = all_nodes.entry(left);
      let index = entry.index();
      entry.or_insert((usize::MAX, usize::MAX, false));
      index
    };
    let right = {
      let entry = all_nodes.entry(right);
      let index = entry.index();
      entry.or_insert((usize::MAX, usize::MAX, false));
      index
    };

    let start_node = node.ends_with('A');
    let end_node = node.ends_with('Z');

    let entry = all_nodes.entry(node);
    let index = entry.index();
    entry
      .and_modify(|e| {
        *e = (left, right, end_node);
      })
      .or_insert((left, right, end_node));

    if start_node {
      nodes.push(index);
    }
  }

  let steps = nodes
    .iter_mut()
    .map(|&mut node| find_cycle_steps(&instructions, &all_nodes, node))
    .map(|v| {
      assert_eq!(v.len(), 1);
      v[0]
    })
    .reduce(num::integer::lcm)
    .unwrap();

  println!("{steps}");
}

fn find_cycle_steps(
  instructions: &[bool],
  all_nodes: &IndexMap<String, (usize, usize, bool)>,
  start: usize,
) -> Vec<usize> {
  let mut steps = Vec::new();
  let mut index = start;
  let mut step = 0;
  loop {
    let (left, right, _) = all_nodes[index];
    if instructions[step % instructions.len()] {
      index = left;
    } else {
      index = right;
    }
    step += 1;
    if let (_, _, true) = all_nodes[index] {
      if steps.len() != 0 {
        let d = step - steps[steps.len() - 1];
        if steps[0] != d {
          steps.push(d);
        } else {
          return steps;
        }
      } else {
        steps.push(step);
      }
    }
  }
}
