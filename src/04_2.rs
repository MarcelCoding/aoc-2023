use std::io;

fn main() -> io::Result<()> {
  let stdin = std::io::stdin();

  let mut input = String::new();

  loop {
    stdin.read_line(&mut input)?;
    if input.ends_with("\n\n") {
      break;
    }
  }

  let input: Vec<&str> = input.trim().lines().collect();

  let mut winning_nums = Vec::new();

  for x in input {
    let (_, x) = x.split_once(':').expect("Invalid input");
    let (winning, actual) = x.split_once('|').expect("Invalid input");

    let mut winning: Vec<u32> = winning
      .split(' ')
      .flat_map(|num| {
        let x =
          num
            .chars()
            .filter(|char| char.is_ascii_digit())
            .rev()
            .enumerate()
            .fold(0, |last, (idx, val)| {
              10_u32.pow(idx as u32) * (val as u32 - '0' as u32) + last
            });
        if x == 0 { None } else { Some(x) }
      })
      .collect();

    let actual: Vec<u32> = actual
      .split(' ')
      .flat_map(|num| {
        let x =
          num
            .chars()
            .filter(|char| char.is_ascii_digit())
            .rev()
            .enumerate()
            .fold(0, |last, (idx, val)| {
              10_u32.pow(idx as u32) * (val as u32 - '0' as u32) + last
            });
        if x == 0 { None } else { Some(x) }
      })
      .collect();

    let mut count = 0;
    for x in actual {
      if winning.contains(&x) {
        count += 1;
      }
    }

    winning_nums.push(count);
  }

  let sum = process(&winning_nums, winning_nums.len());

  println!("Sum: {}", sum);

  Ok(())
}

fn process(winning: &[u32], len: usize) -> usize {
  if len == 0 {
    return 0;
  }

  let mut sum = len.min(winning.len());

  for (idx, val) in winning.iter().take(len).enumerate() {
    sum += process(&winning[idx+1..], *val as usize);
  }

  sum
}
