use std::io;

#[derive(Default)]
struct Cubes {
  red: u32,
  green: u32,
  blue: u32,
}

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

    let (_, game) = input.split_once(':').expect("Invalid input");

    let cubes = game
      .split(';')
      .map(|input| {
        let mut cubes = Cubes::default();

        input
          .split(',')
          .map(|input| input.trim().split_once(' ').expect("Invalid input"))
          .for_each(|(count, color)| match color {
            "red" => cubes.red = count.parse().expect("invalid number"),
            "green" => cubes.green = count.parse().expect("invalid number"),
            "blue" => cubes.blue = count.parse().expect("invalid number"),
            color => panic!("Invalid color: {color}"),
          });

        cubes
      })
      .collect::<Vec<Cubes>>();

    let multiply = cubes.iter().map(|cubes| cubes.red).max().unwrap_or(0)
      * cubes.iter().map(|cubes| cubes.green).max().unwrap_or(0)
      * cubes.iter().map(|cubes| cubes.blue).max().unwrap_or(0);

    sum += multiply;
  }

  println!("Sum: {}", sum);

  Ok(())
}
