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

  let mut sum = 0;

  for x in input {
    let (_, x) = x.split_once(':').expect("Invalid input");
    let (winning, actual) = x.split_once('|').expect("Invalid input");

    let winning: Vec<u32> = winning
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
        if x == 0 {None} else {Some(x)}
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
        if x == 0 {None} else {Some(x)}
      })
      .collect();

    let mut local_sum = 0;

    for x in actual {
      if winning.contains(&x) {
        if local_sum == 0 {
          local_sum = 1;
        } else {
          local_sum *= 2;
        }
      }
    }

    sum+=local_sum;
  }

  println!("Sum: {}", sum);

  Ok(())
}
