use std::io;

#[derive(Default)]
struct Cubes {
  red: u32,
  green: u32,
  blue: u32,
}

const AVAILABLE: Cubes = Cubes {
  red: 12,
  green: 13,
  blue: 14,
};

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

    let (id, game) = input.split_once(':').expect("Invalid input");
    let id: u32 = id
      .strip_prefix("Game ")
      .expect("Invalid input")
      .parse()
      .expect("Invalid number");

    let impossible = game
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
      .any(|cubes| {
        cubes.red > AVAILABLE.red || cubes.green > AVAILABLE.green || cubes.blue > AVAILABLE.blue
      });

    if !impossible {
      sum += id;
    }
  }

  println!("Sum: {}", sum);

  Ok(())
}
