use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "day01.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut fuel:i32 = 0;
    let mut total:i32 = 0;
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        let mass = match line.parse::<i32>() {
          Ok(mass) => mass,
          Err(e) => {
            -1
          }
        };

        fuel = fuel_calc(mass,fuel);
        total = total + fuel;
        // reset fuel variable
        fuel = 0;
    }

    println!("{:?}",total);
}

fn fuel_calc(mass: i32, mut fuel: i32) -> i32 {
    if (mass/3)-2 <= 0 {
        return fuel;
    }
    fuel = fuel + ((mass/3)-2);
    return fuel_calc((mass/3)-2,fuel);
}
