use nom::{
  bytes::complete::{tag, take_while_m_n},
  combinator::map_res,
  error::Error,
  IResult, Parser,
};
use std::num::ParseIntError;

pub fn ws0(input: &str) -> IResult<&str, ()> {
  take_while_m_n(0, 10, |c: char| c.is_ascii_whitespace())(input)
    .map(|(tail, _)| (tail, ()))
}

pub fn ws1(input: &str) -> IResult<&str, ()> {
  take_while_m_n(1, 10, |c: char| c.is_ascii_whitespace())(input)
    .map(|(tail, _)| (tail, ()))
}

pub fn parse_num(input: &str) -> IResult<&str, usize> {
  fn p(input: &str) -> Result<usize, ParseIntError> {
    usize::from_str_radix(input, 10)
  }
  map_res(take_while_m_n(1, 20, |c: char| c.is_ascii_digit()), p)(input)
}

pub fn parse_sep<'a>(
  sep: &'static str,
) -> impl Parser<&'a str, (), Error<&'a str>> {
  move |input| {
    let (input, _) = ws0(input)?;
    tag(sep)(input).map(|(tail, _)| (tail, ()))
  }
}
