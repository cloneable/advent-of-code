use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};

fn main() {
    let mut pairs = Pairs::default();
    aoc::read_from_stdin(&mut pairs);
    println!("{:?}", pairs.numbers);
}

#[derive(Default, Debug)]
struct Pairs {
    numbers: u128,
}

fn map_strings(s: &str) -> u8 {
    match s {
        "0" | "zero" | "orez" => 0,
        "1" | "one" | "eno" => 1,
        "2" | "two" | "owt" => 2,
        "3" | "three" | "eerht" => 3,
        "4" | "four" | "ruof" => 4,
        "5" | "five" | "evif" => 5,
        "6" | "six" | "xis" => 6,
        "7" | "seven" | "neves" => 7,
        "8" | "eight" | "thgie" => 8,
        "9" | "nine" | "enin" => 9,
        _ => unreachable!("{s}"),
    }
}

impl aoc::LineParser for Pairs {
    fn parse_line(&mut self, line: &str) {
        static RE: Lazy<Regex> = Lazy::new(|| {
            RegexBuilder::new(r"([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)")
                .case_insensitive(true)
                .build()
                .unwrap()
        });
        static RE2: Lazy<Regex> = Lazy::new(|| {
            RegexBuilder::new(r"([0-9]|orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)")
                .case_insensitive(true)
                .build()
                .unwrap()
        });

        let rev = line.chars().rev().collect::<String>();

        let first = RE.find(line);
        let last = RE2.find(&rev);

        match (first, last) {
            (Some(first), Some(last)) => {
                let first = map_strings(&first.as_str().to_lowercase());
                let last = map_strings(&last.as_str().to_lowercase());
                let v: u8 = first * 10 + last;
                self.numbers += v as u128;
            }
            _ => (),
        }
    }
}
