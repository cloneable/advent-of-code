use nom::{bytes::complete::tag, bytes::complete::take_while_m_n, IResult};
use std::collections::HashMap;

fn main() {
  let mut lines = std::io::stdin().lines();

  let instructions = lines
    .next()
    .unwrap()
    .unwrap()
    .into_bytes()
    .into_iter()
    .map(|b| {
      if b == b'L' {
        Instruction::Left
      } else {
        assert_eq!(b, b'R');
        Instruction::Right
      }
    })
    .collect::<Vec<_>>();
  let _ = lines.next().unwrap().unwrap();

  let mut nodes = HashMap::<String, (String, String)>::new();
  for line in lines {
    let line = line.unwrap();
    let (rem, (node, left_right)) = parse_node(&line).unwrap();
    assert_eq!(0, rem.len());
    assert!(nodes.insert(node, left_right).is_none());
  }

  let mut steps = 0;
  let mut ic = 0;
  let mut next = "AAA";
  while next != "ZZZ" {
    steps += 1;

    let (left, right) = nodes.get(next).unwrap();
    match instructions[ic] {
      Instruction::Left => next = left,
      Instruction::Right => next = right,
    }

    ic = (ic + 1) % instructions.len();
  }

  println!("{steps}");
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
  Left,
  Right,
}

struct Node {
  name: String,
  left: String,
  right: String,
}

fn parse_node(input: &str) -> IResult<&str, (String, (String, String))> {
  let (input, name) =
    take_while_m_n(1, 3, |c: char| c.is_ascii_alphabetic())(input)?;
  let (input, _) = tag(" = (")(input)?;
  let (input, left) =
    take_while_m_n(1, 3, |c: char| c.is_ascii_alphabetic())(input)?;
  let (input, _) = tag(", ")(input)?;
  let (input, right) =
    take_while_m_n(1, 3, |c: char| c.is_ascii_alphabetic())(input)?;
  let (input, _) = tag(")")(input)?;

  Ok((input, (name.to_owned(), (left.to_owned(), right.to_owned()))))
}
