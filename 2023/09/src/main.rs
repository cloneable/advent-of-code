fn main() {
  let lines = std::io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

  let mut sum = 0;

  for line in lines {
    let nums = line
      .split_ascii_whitespace()
      .map(|v| v.parse::<isize>().unwrap())
      .collect::<Vec<_>>();

    let mut deltas = nums.clone();
    let mut lasts = vec![nums.last().map(|v| *v).unwrap()];
    while deltas.iter().any(|&n| n != 0) {
      deltas = deltas.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();
      lasts.push(deltas.last().map(|v| *v).unwrap());
    }

    sum += lasts.into_iter().reduce(|c, n| c + n).unwrap_or_default();
  }

  println!("{sum}");
}
