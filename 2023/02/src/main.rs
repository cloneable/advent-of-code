use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    multi::{separated_list0, separated_list1},
    IResult,
};
use std::num::ParseIntError;

fn main() {
    let mut games = Games {
        rgb: [12, 13, 14],
        sum: 0,
    };
    aoc::read_from_stdin(&mut games);
    println!("{:?}", games.sum);
}

#[derive(Debug)]
struct Games {
    rgb: [usize; 3],
    sum: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Vec<(usize, Color)>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

impl<'a> TryFrom<&'a str> for Color {
    type Error = ();

    fn try_from(s: &'a str) -> Result<Self, ()> {
        Ok(match s {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => return Err(()),
        })
    }
}

fn ws0(input: &str) -> IResult<&str, ()> {
    take_while_m_n(0, 1, |c: char| c.is_ascii_whitespace())(input).map(|(tail, _)| (tail, ()))
}

fn ws1(input: &str) -> IResult<&str, ()> {
    take_while_m_n(1, 1, |c: char| c.is_ascii_whitespace())(input).map(|(tail, _)| (tail, ()))
}

fn parse_num(input: &str) -> IResult<&str, usize> {
    fn p(input: &str) -> Result<usize, ParseIntError> {
        usize::from_str_radix(input, 10)
    }
    map_res(take_while_m_n(1, 10, |c: char| c.is_ascii_digit()), p)(input)
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    map_res(
        alt((tag("red"), tag("green"), tag("blue"))),
        Color::try_from,
    )(input)
}

fn parse_cube(input: &str) -> IResult<&str, (usize, Color)> {
    let (input, _) = ws0(input)?;
    let (input, id) = parse_num(input)?;
    let (input, _) = ws1(input)?;
    let (input, color) = parse_color(input)?;

    Ok((input, (id, color)))
}

fn parse_cubes_separator(input: &str) -> IResult<&str, ()> {
    let (input, _) = ws0(input)?;
    tag(",")(input).map(|(tail, _)| (tail, ()))
}

fn parse_set(input: &str) -> IResult<&str, Vec<(usize, Color)>> {
    separated_list1(parse_cubes_separator, parse_cube)(input)
}

fn parse_set_separator(input: &str) -> IResult<&str, ()> {
    let (input, _) = ws0(input)?;
    tag(";")(input).map(|(tail, _)| (tail, ()))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game")(input)?;
    let (input, _) = ws1(input)?;
    let (input, id) = parse_num(input)?;
    let (input, _) = tag(":")(input)?;

    let (input, sets) = separated_list0(parse_set_separator, parse_set)(input)?;

    Ok((input, Game { id, sets }))
}

impl aoc::LineParser for Games {
    fn parse_line(&mut self, line: &str) {
        let (rem, game) = parse_game(line).unwrap();
        assert_eq!(0, rem.len(), "{rem}");
        assert_ne!(0, game.id);

        let mut max_rgb = [0usize; 3];
        for set in game.sets {
            for (count, color) in set {
                if max_rgb[color as usize] < count {
                    max_rgb[color as usize] = count;
                }
            }
        }
        for color in 0..3 {
            if self.rgb[color] < max_rgb[color] {
                return;
            }
        }

        self.sum += game.id;
    }
}
