use std::str::FromStr;

fn main() {
    let mut guide = StrategyGuide::default();
    aoc::read_from_stdin(&mut guide);
    println!("{}", guide.game.player_total_score);
}

#[derive(Debug, Default)]
struct Game {
    rounds: Vec<Round>,
    player_total_score: usize,
}

#[derive(Debug, Copy, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Hand {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err(anyhow::Error::msg(format!("unknown character: {}", s))),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug)]
struct Round {
    player: Hand,
    opponent: Hand,
}

impl Round {
    fn play(&self) -> Outcome {
        use Hand::*;
        use Outcome::*;
        match (self.player, self.opponent) {
            (Rock, Scissors) => Win,
            (Rock, Paper) => Loss,
            (Paper, Rock) => Win,
            (Paper, Scissors) => Loss,
            (Scissors, Rock) => Loss,
            (Scissors, Paper) => Win,
            (_, _) => Draw,
        }
    }

    fn player_score(&self) -> usize {
        self.play() as usize + self.player as usize
    }
}

#[derive(Default, Debug)]
struct StrategyGuide {
    game: Game,
}

impl aoc::LineParser for StrategyGuide {
    fn parse_line(&mut self, line: &str) {
        let opponent: Hand = line[0..1].parse().unwrap();
        let player: Hand = line[2..3].parse().unwrap();
        let round = Round { player, opponent };

        let score = round.player_score();
        self.game.player_total_score += score;
        self.game.rounds.push(round);
    }
}
