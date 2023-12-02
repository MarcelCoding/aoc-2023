use std::io;

const START: u32 = '0' as u32;

const NUMBERS: [&str; 9] = [
  "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

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

    let input = input.strip_suffix('\n').unwrap();

    let first = find_first_num(input);
    let last = find_last_num(input).unwrap_or(first);

    let num = first * 10 + last;
    sum += num;
  }

  println!("Sum: {}", sum);

  Ok(())
}

fn find_first_num(input: &str) -> u32 {
  if input.is_empty() {
    panic!("missing digit");
  }

  let first_char = input.chars().next().unwrap();
  if first_char.is_ascii_digit() {
    return to_num(first_char);
  }

  for (idx, num_str) in NUMBERS.iter().enumerate() {
    if input.starts_with(num_str) {
      return idx as u32 + 1;
    }
  }

  find_first_num(&input[1..])
}

fn find_last_num(input: &str) -> Option<u32> {
  if input.is_empty() {
    return None;
  }

  let last_char = input.chars().last().unwrap();
  if last_char.is_ascii_digit() {
    return Some(to_num(last_char));
  }

  for (idx, num_str) in NUMBERS.iter().enumerate() {
    if input.ends_with(num_str) {
      return Some(idx as u32 + 1);
    }
  }

  find_last_num(&input[..input.len() - 1])
}

fn to_num(char: char) -> u32 {
  char as u32 - START
}
