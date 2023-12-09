fn main() {
  let lines = std::io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

  let mut sum = 0;

  for line in lines {
    let nums = line
      .split_ascii_whitespace()
      .map(|v| v.parse::<isize>().unwrap())
      .collect::<Vec<_>>();

    let mut deltas = nums.clone();
    let mut firsts = vec![nums.first().copied().unwrap()];
    while deltas.iter().any(|&n| n != 0) {
      deltas = deltas.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();
      firsts.push(deltas.first().copied().unwrap());
    }
    firsts.reverse();

    sum += firsts.into_iter().reduce(|a, b| b - a).unwrap();
  }

  println!("{sum}");
}
