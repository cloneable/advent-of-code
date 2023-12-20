use nom::{
  branch::alt,
  bytes::complete::{tag, take_while1},
  combinator::map_res,
  multi::many0,
  sequence::terminated,
  IResult,
};
use std::{collections::HashMap, num::ParseIntError};

fn main() {
  let mut lines = std::io::stdin().lines().map(Result::unwrap);

  let mut workflows = HashMap::<String, Workflow>::new();
  while let Some(line) = lines.next() {
    if line.is_empty() {
      break;
    }
    let (name, workflow) = parse_workflow(&line).unwrap().1;
    workflows.insert(name, workflow);
  }

  let mut parts = Vec::new();
  while let Some(line) = lines.next() {
    let part = parse_part(&line).unwrap().1;
    parts.push(part);
  }

  let mut accepted = Vec::new();
  for part in parts {
    if check(&workflows, "in", &part) {
      accepted.push(part);
    }
  }

  let mut sum = 0;
  for part in accepted {
    sum += part.x + part.m + part.a + part.s;
  }
  eprintln!("{sum}");
}

fn check(
  workflows: &HashMap<String, Workflow>,
  name: &str,
  part: &Part,
) -> bool {
  let workflow = &workflows[name];
  for (cond, rule) in &workflow.conditions {
    if cond.matches(part) {
      return match rule {
        Rule::Accept => true,
        Rule::Reject => false,
        Rule::Workflow(name) => check(&workflows, &name, part),
      };
    }
  }
  match &workflow.rule {
    Rule::Accept => true,
    Rule::Reject => false,
    Rule::Workflow(name) => check(&workflows, &name, part),
  }
}

#[derive(Debug)]
struct Part {
  x: usize,
  m: usize,
  a: usize,
  s: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Category {
  X,
  M,
  A,
  S,
}

#[derive(Debug)]
struct Workflow {
  conditions: Vec<(Condition, Rule)>,
  rule: Rule,
}

#[derive(Debug)]
struct Condition {
  category: Category,
  greater: bool,
  value: usize,
}

impl Condition {
  fn matches(&self, part: &Part) -> bool {
    match (self.category, self.greater) {
      (Category::X, false) => part.x < self.value,
      (Category::X, true) => part.x > self.value,
      (Category::M, false) => part.m < self.value,
      (Category::M, true) => part.m > self.value,
      (Category::A, false) => part.a < self.value,
      (Category::A, true) => part.a > self.value,
      (Category::S, false) => part.s < self.value,
      (Category::S, true) => part.s > self.value,
    }
  }
}

#[derive(Debug)]
enum Rule {
  Accept,
  Reject,
  Workflow(String),
}

fn parse_workflow(s: &str) -> IResult<&str, (String, Workflow)> {
  let (s, name) = parse_name(s)?;
  let (s, _) = tag("{")(s)?;
  let (s, conditions) = many0(terminated(parse_condition_rule, tag(",")))(s)?;
  let (s, rule) = parse_rule(s)?;
  let (s, _) = tag("}")(s)?;

  Ok((s, (name, Workflow { conditions, rule })))
}

fn parse_condition_rule(s: &str) -> IResult<&str, (Condition, Rule)> {
  let (s, condition) = parse_condition(s)?;
  let (s, _) = tag(":")(s)?;
  let (s, rule) = parse_rule(s)?;

  Ok((s, (condition, rule)))
}

fn parse_condition(s: &str) -> IResult<&str, Condition> {
  let (s, category) = parse_category(s)?;
  let (s, op) = alt((tag("<"), tag(">")))(s)?;
  let (s, value) = parse_num(s)?;

  let greater = op == ">";
  Ok((s, Condition { category, greater, value }))
}

fn parse_name(s: &str) -> IResult<&str, String> {
  let (s, name) = take_while1(|c: char| c.is_ascii_lowercase())(s)?;
  Ok((s, name.to_string()))
}

fn parse_rule(s: &str) -> IResult<&str, Rule> {
  alt((parse_rule_accept, parse_rule_reject, parse_rule_workflow))(s)
}

fn parse_rule_accept(s: &str) -> IResult<&str, Rule> {
  let (s, _) = tag("A")(s)?;
  Ok((s, Rule::Accept))
}

fn parse_rule_reject(s: &str) -> IResult<&str, Rule> {
  let (s, _) = tag("R")(s)?;
  Ok((s, Rule::Reject))
}

fn parse_rule_workflow(s: &str) -> IResult<&str, Rule> {
  let (s, name) = parse_name(s)?;
  Ok((s, Rule::Workflow(name)))
}

fn parse_num(s: &str) -> IResult<&str, usize> {
  fn p(s: &str) -> Result<usize, ParseIntError> {
    usize::from_str_radix(s, 10)
  }
  map_res(take_while1(|c: char| c.is_ascii_digit()), p)(s)
}

fn parse_category(s: &str) -> IResult<&str, Category> {
  let (s, category) = alt((tag("x"), tag("m"), tag("a"), tag("s")))(s)?;
  let category = match category {
    "x" => Category::X,
    "m" => Category::M,
    "a" => Category::A,
    "s" => Category::S,
    _ => unreachable!(),
  };
  Ok((s, category))
}

fn parse_part(s: &str) -> IResult<&str, Part> {
  let (s, _) = tag("{x=")(s)?;
  let (s, x_num) = parse_num(s)?;
  let (s, _) = tag(",m=")(s)?;
  let (s, m_num) = parse_num(s)?;
  let (s, _) = tag(",a=")(s)?;
  let (s, a_num) = parse_num(s)?;
  let (s, _) = tag(",s=")(s)?;
  let (s, s_num) = parse_num(s)?;
  let (s, _) = tag("}")(s)?;

  Ok((s, Part { x: x_num, m: m_num, a: a_num, s: s_num }))
}
