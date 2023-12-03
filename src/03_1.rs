use std::io;

#[derive(Debug)]
struct Number {
  line: usize,
  column: usize,
  len: usize,
  val: u32,
}

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

  let mut numbers = Vec::new();

  for (idx, line) in input.iter().enumerate() {
    let mut buf = Vec::new();
    for (column, char) in line.chars().enumerate() {
      if char.is_ascii_digit() {
        buf.push(char as u32 - '0' as u32);
      } else if !buf.is_empty() {
        numbers.push(Number {
          line: idx,
          column: column - buf.len(),
          len: buf.len(),
          val: buf.iter().enumerate().fold(0, |last, (idx, val)| {
            10_u32.pow((buf.len() - idx - 1) as u32) * val + last
          }),
        });
        buf.clear();
      }
    }
    if !buf.is_empty() {
      numbers.push(Number {
        line: idx,
        column: line.len() - buf.len(),
        len: buf.len(),
        val: buf.iter().enumerate().fold(0, |last, (idx, val)| {
          10_u32.pow((buf.len() - idx - 1) as u32) * val + last
        }),
      });
      buf.clear();
    }
  }

  let sum: u32 = numbers
    .iter()
    .filter(|num| {
      input
        .iter()
        .skip(if num.line == 0 { 0 } else { num.line - 1 })
        .take(if num.line == 0 { 2 } else { 3 })
        .flat_map(|line| {
          line
            .chars()
            .skip(if num.column == 0 { 0 } else { num.column - 1 })
            .take(num.len + if num.column == 0 { 1 } else { 2 })
        })
        .any(|c| !c.is_ascii_digit() && c != '.')
    })
    .map(|num| num.val)
    .sum();

  println!("Sum: {}", sum);

  Ok(())
}
