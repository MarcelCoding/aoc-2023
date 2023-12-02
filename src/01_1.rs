use std::io;

const START: u32 = '0' as u32;

fn main() -> io::Result<()> {
  let stdin = std::io::stdin();
  let mut sum = 0;

  loop {
    let mut input = String::new();
    stdin.read_line(&mut input)?;
    if input == "\n" {
      if sum == 0 {
        continue;
      } else {
        break;
      }
    }

    let mut nums = input.chars().filter(|char| char.is_ascii_digit());
    let first = to_num(nums.next().expect("missing digit"));
    let last = nums.last().map(to_num).unwrap_or(first);

    let num = first * 10 + last;
    sum += num;
  }

  println!("Sum: {}", sum);

  Ok(())
}

fn to_num(char: char) -> u32 {
  char as u32 - START
}
