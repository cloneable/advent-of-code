use aoc::pc::parse_num;
use indexmap::IndexMap;
use nom::{
  branch::alt,
  bytes::complete::{tag, take_while_m_n},
  multi::separated_list1,
  IResult,
};
use std::io;

fn main() {
  let input = io::stdin().lines().map(Result::unwrap).next().unwrap();
  let (_, sequence) = parse_sequence(&input).unwrap();

  let mut boxes = vec![IndexMap::<&str, usize>::new(); 256];
  for s in sequence {
    let box_num = hash(s.0);
    match s.1 {
      Operation::Add(focal_length) => {
        *boxes[box_num].entry(s.0).or_default() = focal_length;
      }
      Operation::Remove => {
        boxes[box_num].shift_remove(s.0);
      }
    }
  }

  let mut sum = 0;
  for (lenses, bx) in boxes.into_iter().zip(1usize..) {
    sum += lenses
      .iter()
      .zip(1usize..)
      .map(|(focal_length, lens)| bx * lens * focal_length.1)
      .sum::<usize>();
  }

  println!("{sum}");
}

#[derive(Debug)]
enum Operation {
  Add(usize),
  Remove,
}

fn hash(s: &str) -> usize {
  let mut v = 0;
  for &c in s.as_bytes() {
    v += c as usize;
    v *= 17;
    v %= 256;
  }
  v
}

fn parse_sequence(input: &str) -> IResult<&str, Vec<(&str, Operation)>> {
  separated_list1(tag(","), parse_step)(input)
}

fn parse_step(input: &str) -> IResult<&str, (&str, Operation)> {
  let (input, label) = parse_label(input)?;
  let (input, op) = parse_operation(input)?;
  Ok((input, (label, op)))
}

fn parse_label(input: &str) -> IResult<&str, &str> {
  take_while_m_n(1, 20, |c: char| c.is_ascii_alphabetic())(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
  alt((parse_add, parse_remove))(input)
}

fn parse_remove(input: &str) -> IResult<&str, Operation> {
  let (input, _) = tag("-")(input)?;
  Ok((input, Operation::Remove))
}

fn parse_add(input: &str) -> IResult<&str, Operation> {
  let (input, _) = tag("=")(input)?;
  let (input, focal_length) = parse_num(input)?;
  Ok((input, Operation::Add(focal_length)))
}
