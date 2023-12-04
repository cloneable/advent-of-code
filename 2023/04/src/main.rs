use aoc::pc::{parse_num, ws0, ws1};
use nom::{bytes::complete::tag, multi::separated_list1, IResult};
use std::{collections::HashSet, io};

fn main() {
    let mut sum = 0;
    let mut copy_counters = Vec::<usize>::default();

    for (num, line) in io::stdin().lines().enumerate() {
        let line = line.unwrap();

        let (_, original) = parse_card(&line).unwrap();

        let mut affected_cards = 0;
        for w in original.winning {
            if original.selected.contains(&w) {
                affected_cards += 1;
            }
        }

        let mut total_copies = 1;
        if let Some(extra_copies) = copy_counters.get(num) {
            total_copies += extra_copies;
        }

        copy_counters.resize(copy_counters.len().max(num + 1 + affected_cards), 0);
        for copy_index in num..(num + affected_cards) {
            copy_counters[copy_index + 1] += total_copies;
        }

        sum += total_copies;
    }

    println!("{:?}", sum);
}

#[derive(Debug)]
struct Card {
    _id: usize,
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
            _id: id,
            selected: selected.into_iter().collect(),
            winning,
        },
    ))
}
