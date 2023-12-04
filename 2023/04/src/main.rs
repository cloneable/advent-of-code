use aoc::pc::{parse_num, ws0, ws1};
use nom::{bytes::complete::tag, multi::separated_list1, IResult};
use std::collections::HashSet;

fn main() {
    let mut problem = Problem { sum: 0 };
    aoc::read_from_stdin(&mut problem);
    println!("{:?}", problem.sum);
}

#[derive(Debug)]
struct Problem {
    sum: usize,
}

impl aoc::LineParser for Problem {
    fn parse_line(&mut self, line: &str) {
        let (rem, card) = parse_card(line).unwrap();

        let mut points = 0;
        for w in card.winning {
            if card.selected.contains(&w) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        self.sum += points;
    }
}

#[derive(Debug)]
struct Card {
    id: usize,
    selected: HashSet<usize>,
    winning: Vec<usize>,
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = ws1(input)?;
    let (input, id) = parse_num(input)?;
    let (input, _) = tag(":")(input)?;

    let (input, _) = ws0(input)?;
    let (input, selected) = separated_list1(ws1, parse_num)(input)?;
    let (input, _) = ws0(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, _) = ws0(input)?;
    let (input, winning) = separated_list1(ws1, parse_num)(input)?;

    Ok((
        input,
        Card {
            id,
            selected: selected.into_iter().collect(),
            winning,
        },
    ))
}
