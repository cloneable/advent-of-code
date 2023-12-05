use aoc::pc::{parse_num, ws0, ws1};
use nom::{branch::alt, bytes::complete::tag, multi::separated_list1, IResult};
use std::{collections::HashMap, io, ops::Range};

fn main() {
    let mut lines = io::stdin().lines().peekable();
    let line0 = lines.next().unwrap().unwrap();
    let (rem, seeds) = parse_seeds(&line0).unwrap();
    assert_eq!(0, rem.len());

    let mut mapped_ranges = HashMap::<Type, (Type, Vec<Mapping>)>::default();
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

        mapped_ranges.insert(title.from, (title.to, mappings));
    }

    let mut ranges = seeds;
    let mut src_type = &Type::Seed;
    while src_type != &Type::Location {
        let (dest_type, dest_ranges) = &mapped_ranges[src_type];

        let mut new_ranges = Vec::new();
        let mut stack = ranges.iter().rev().map(|r| r.clone()).collect::<Vec<_>>();

        while let Some(seed_range) = stack.pop() {
            let mut mapped = false;
            for mapped_range in dest_ranges {
                let (prefix, infix, suffix) =
                    map_range(&seed_range, &mapped_range.src, mapped_range.dest);

                if let Some(prefix) = prefix {
                    stack.push(prefix);
                    mapped = true;
                }
                if let Some(infix) = infix {
                    new_ranges.push(infix);
                    mapped = true;
                }
                if let Some(suffix) = suffix {
                    stack.push(suffix);
                    mapped = true;
                }
            }
            if !mapped {
                new_ranges.push(seed_range);
            }
        }

        new_ranges.extend(stack.into_iter());

        ranges = new_ranges;
        src_type = dest_type;
    }

    ranges.sort_by_key(|r| r.start);
    let location = ranges.first().unwrap().start;

    println!("{location}");
}

fn map_range(
    a: &Range<usize>,
    m: &Range<usize>,
    d: usize,
) -> (
    Option<Range<usize>>,
    Option<Range<usize>>,
    Option<Range<usize>>,
) {
    assert!(a.start < a.end);
    assert!(m.start < m.end);

    let prefix = if a.start < m.start && a.end > m.start {
        Some(a.start..m.start)
    } else {
        None
    };

    let suffix = if a.end > m.end && a.start < m.end {
        Some(m.end..a.end)
    } else {
        None
    };

    let overlap_start = a.start.max(m.start);
    let overlap_end = a.end.min(m.end);
    let infix = if overlap_start < overlap_end {
        let start = overlap_start + d - m.start;
        let end = overlap_end + d - m.start;
        Some(start..end)
    } else {
        None
    };

    (prefix, infix, suffix)
}

fn parse_range(input: &str) -> IResult<&str, Range<usize>> {
    let (input, offset) = parse_num(input)?;
    let (input, _) = ws1(input)?;
    let (input, length) = parse_num(input)?;

    Ok((
        input,
        Range {
            start: offset,
            end: offset + length,
        },
    ))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<Range<usize>>> {
    let (input, _) = tag("seeds:")(input)?;
    let (input, _) = ws0(input)?;
    let (input, ranges) = separated_list1(ws1, parse_range)(input)?;

    Ok((input, ranges))
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
