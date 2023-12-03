use aoc::pc::{parse_num, parse_sep, ws0, ws1};
use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::{separated_list0, separated_list1},
    IResult,
};

fn main() {
    let mut games = Games { sum: 0 };
    aoc::read_from_stdin(&mut games);
    println!("{:?}", games.sum);
}

#[derive(Debug)]
struct Games {
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

fn color_red(input: &str) -> IResult<&str, Color> {
    tag("red")(input).map(|(rem, _)| (rem, Color::Red))
}

fn color_green(input: &str) -> IResult<&str, Color> {
    tag("green")(input).map(|(rem, _)| (rem, Color::Green))
}

fn color_blue(input: &str) -> IResult<&str, Color> {
    tag("blue")(input).map(|(rem, _)| (rem, Color::Blue))
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    alt((color_red, color_green, color_blue))(input)
}

fn parse_cube(input: &str) -> IResult<&str, (usize, Color)> {
    let (input, _) = ws0(input)?;
    let (input, id) = parse_num(input)?;
    let (input, _) = ws1(input)?;
    let (input, color) = parse_color(input)?;

    Ok((input, (id, color)))
}

fn parse_set(input: &str) -> IResult<&str, Vec<(usize, Color)>> {
    separated_list1(parse_sep(","), parse_cube)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game")(input)?;
    let (input, _) = ws1(input)?;
    let (input, id) = parse_num(input)?;
    let (input, _) = tag(":")(input)?;

    let (input, sets) = separated_list0(parse_sep(";"), parse_set)(input)?;

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

        self.sum += max_rgb.iter().product::<usize>();
    }
}
