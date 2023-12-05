use aoc::pc::{parse_num, ws0, ws1};
use nom::{branch::alt, bytes::complete::tag, multi::separated_list1, IResult};
use std::{collections::HashMap, io, ops::Range};

fn main() {
    let mut problem = Problem::default();

    let mut lines = io::stdin().lines().peekable();
    let line0 = lines.next().unwrap().unwrap();
    let (rem, seeds) = parse_seeds(&line0).unwrap();
    assert_eq!(0, rem.len());
    problem.seeds = seeds.nums;

    while let Some(empty) = lines.next() {
        assert_eq!(0, empty.as_ref().unwrap().len());

        let line = lines.next().unwrap().unwrap();
        let (rem, title) = parse_title(&line).unwrap();
        assert_eq!(0, rem.len());

        let mut mappings = Vec::<Mapping>::new();
        while let Some(next) = lines.peek() {
            if next.as_ref().unwrap().len() == 0 {
                break;
            }

            let line = lines.next().unwrap().unwrap();
            let (rem, mapping) = parse_mapping(&line).unwrap();
            assert_eq!(0, rem.len());

            mappings.push(mapping);
        }
        problem.mappings.insert(title.from, (title.to, mappings));
    }

    // eprintln!("{problem:#?}");

    problem.location = usize::MAX;
    for seed in problem.seeds {
        let mut src_type = Type::Seed;
        let mut offset = seed;
        loop {
            let (dest_type, mappings) = &problem.mappings[&src_type];
            offset = mappings
                .iter()
                .find_map(|m| m.translate(offset))
                .unwrap_or(offset);

            if *dest_type == Type::Location {
                if problem.location > offset {
                    problem.location = offset;
                }
                break;
            }
            src_type = *dest_type;
        }
    }

    println!("{}", problem.location);
}

#[derive(Default, Debug)]
struct Problem {
    seeds: Vec<usize>,
    mappings: HashMap<Type, (Type, Vec<Mapping>)>,

    location: usize,
}

#[derive(Debug)]
struct Seeds {
    nums: Vec<usize>,
}

fn parse_seeds(input: &str) -> IResult<&str, Seeds> {
    let (input, _) = tag("seeds:")(input)?;
    let (input, _) = ws0(input)?;
    let (input, nums) = separated_list1(ws1, parse_num)(input)?;

    Ok((input, Seeds { nums }))
}

#[derive(Debug)]
struct Title {
    from: Type,
    to: Type,
}

fn parse_title(input: &str) -> IResult<&str, Title> {
    let (input, from) = parse_type(input)?;
    let (input, _) = tag("-to-")(input)?;
    let (input, to) = parse_type(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag("map:")(input)?;

    Ok((input, Title { from, to }))
}

#[derive(Debug)]
struct Mapping {
    src: Range<usize>,
    dest: usize,
}

impl Mapping {
    fn translate(&self, i: usize) -> Option<usize> {
        if self.src.contains(&i) {
            Some((i - self.src.start) + self.dest)
        } else {
            None
        }
    }
}

fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
    let (input, dest) = parse_num(input)?;
    let (input, _) = ws1(input)?;
    let (input, src) = parse_num(input)?;
    let (input, _) = ws1(input)?;
    let (input, len) = parse_num(input)?;

    Ok((
        input,
        Mapping {
            dest,
            src: src..src + len,
        },
    ))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
enum Type {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

fn type_seed(input: &str) -> IResult<&str, Type> {
    tag("seed")(input).map(|(rem, _)| (rem, Type::Seed))
}

fn type_soil(input: &str) -> IResult<&str, Type> {
    tag("soil")(input).map(|(rem, _)| (rem, Type::Soil))
}

fn type_fertilizer(input: &str) -> IResult<&str, Type> {
    tag("fertilizer")(input).map(|(rem, _)| (rem, Type::Fertilizer))
}

fn type_water(input: &str) -> IResult<&str, Type> {
    tag("water")(input).map(|(rem, _)| (rem, Type::Water))
}

fn type_light(input: &str) -> IResult<&str, Type> {
    tag("light")(input).map(|(rem, _)| (rem, Type::Light))
}

fn type_temperature(input: &str) -> IResult<&str, Type> {
    tag("temperature")(input).map(|(rem, _)| (rem, Type::Temperature))
}

fn type_humidity(input: &str) -> IResult<&str, Type> {
    tag("humidity")(input).map(|(rem, _)| (rem, Type::Humidity))
}

fn type_location(input: &str) -> IResult<&str, Type> {
    tag("location")(input).map(|(rem, _)| (rem, Type::Location))
}

fn parse_type(input: &str) -> IResult<&str, Type> {
    alt((
        type_seed,
        type_soil,
        type_fertilizer,
        type_water,
        type_light,
        type_temperature,
        type_humidity,
        type_location,
    ))(input)
}
