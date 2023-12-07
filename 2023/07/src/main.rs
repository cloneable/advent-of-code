use std::cmp::Ordering;

fn main() {
  let mut plays = Vec::new();
  for line in std::io::stdin().lines() {
    let line = line.unwrap();

    let values = line.split_ascii_whitespace().collect::<Vec<_>>();
    let hand = translate_hand(values[0]);
    let bid = values[1].parse::<usize>().unwrap();
    let rank = calc_type(&hand);

    plays.push(Play { rank, hand, bid });
  }

  plays.sort_unstable_by(|a, b| {
    let rank_ordering = a.rank.cmp(&b.rank);
    if rank_ordering == Ordering::Equal {
      a.hand.cmp(&b.hand)
    } else {
      rank_ordering
    }
  });

  let winnings =
    plays.into_iter().zip(1usize..).map(|(p, i)| p.bid * i).sum::<usize>();

  println!("{winnings}");
}

#[derive(Debug)]
struct Play {
  rank: usize,
  hand: Vec<u8>,
  bid: usize,
}

fn calc_type(hand: &[u8]) -> usize {
  let mut types = 0;
  let mut max = 0;

  let mut counts = vec![0u8; 16];
  let mut jokers = 0;
  for &card in hand {
    let card = card as usize;
    if card == 1 {
      jokers += 1;
      continue;
    }
    if counts[card] == 0 {
      types += 1;
    }
    counts[card] += 1;
    if counts[card] > max {
      max = counts[card];
    }
  }

  if max == 0 {
    max = jokers;
    types = 1;
  } else {
    max += jokers;
  }

  match (max, types) {
    (5, 1) => 6,
    (4, 2) => 5,
    (3, 2) => 4,
    (3, 3) => 3,
    (2, 3) => 2,
    (2, 4) => 1,
    (1, 5) => 0,
    _ => unreachable!("{max} {types}"),
  }
}

fn translate_hand(hand: &str) -> Vec<u8> {
  hand.bytes().map(card_value).collect()
}

fn card_value(card: u8) -> u8 {
  match card {
    b'A' => 14,
    b'K' => 13,
    b'Q' => 12,
    b'T' => 10,

    b'9' => 9,
    b'8' => 8,
    b'7' => 7,
    b'6' => 6,
    b'5' => 5,
    b'4' => 4,
    b'3' => 3,
    b'2' => 2,

    b'J' => 1,

    _ => unreachable!("{card}"),
  }
}
