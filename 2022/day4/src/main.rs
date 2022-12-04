use std::str::FromStr;

fn main() {
    let mut pairs = Pairs::default();
    aoc::read_from_stdin(&mut pairs);
    println!("{:?}", pairs.overlapping);
}

#[derive(Default, Debug)]
struct Pairs {
    overlapping: usize,
}

impl aoc::LineParser for Pairs {
    fn parse_line(&mut self, line: &str) {
        let (f, s) = line.split_once(',').unwrap();
        let (first, second): (Section, Section) = (f.parse().unwrap(), s.parse().unwrap());
        if first.overlaps(&second) {
            self.overlapping += 1;
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Section {
    start: usize,
    end: usize,
}

impl Section {
    fn overlaps(&self, other: &Section) -> bool {
        !(self.end < other.start || self.start > other.end)
    }
}

impl FromStr for Section {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or(anyhow::Error::msg("bad section"))?;
        let (start, end): (usize, usize) = (start.parse()?, end.parse()?);
        debug_assert!(start <= end, "{} <= {}", start, end);
        Ok(Section { start, end })
    }
}
