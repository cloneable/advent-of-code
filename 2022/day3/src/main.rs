use std::{collections::HashSet, str::FromStr};

fn main() {
    let mut guide = RucksackReader::default();
    aoc::read_from_stdin(&mut guide);
    println!("{}", guide.items_sum);
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Item(u8);

impl FromStr for Item {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(anyhow::Error::msg(format!("wrong len: {}", s)));
        }
        let c = s.bytes().into_iter().next().unwrap();
        if c >= b'a' && c <= b'z' {
            Ok(Item(c - b'a' + 1))
        } else if c >= b'A' && c <= b'Z' {
            Ok(Item(c - b'A' + 1 + 26))
        } else {
            Err(anyhow::Error::msg(format!("wrong char: {}", s)))
        }
    }
}

#[derive(Default, Debug)]
struct RucksackReader {
    items_sum: usize,
}

impl aoc::LineParser for RucksackReader {
    fn parse_line(&mut self, line: &str) {
        let (first, second) = line.split_at(line.len() / 2);

        let mut first_set = HashSet::new();
        for i in 0..first.len() {
            let item: Item = first[i..i + 1].parse().unwrap();
            first_set.insert(item);
        }

        let mut set = HashSet::new();
        for i in 0..second.len() {
            let item: Item = second[i..i + 1].parse().unwrap();
            if first_set.contains(&item) {
                set.insert(item);
            }
        }

        for item in set.into_iter() {
            self.items_sum += item.0 as usize;
        }
    }
}
