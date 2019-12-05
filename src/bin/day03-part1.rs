use std::fs;
use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    None
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let contents = fs::read_to_string("day03.txt")
        .expect("Something went wrong reading the file");
    let mut lines = contents.lines();
    let wire_1 = lines.next();
    let wire_2 = lines.next();

    let wire_1: Vec<&str> = wire_1.unwrap().split(',').collect();
    let wire_2: Vec<&str> = wire_2.unwrap().split(',').collect();

    let mut set_1: HashSet<(i32,i32)> = HashSet::new();
    let mut set_2: HashSet<(i32,i32)> = HashSet::new();

    // add origin
    set_1.insert((0,0));
    set_2.insert((0,0));

    set_1 = trace(set_1, wire_1);
    set_2 = trace(set_2, wire_2);


    set_1.remove(&(0,0)); // origin should not count as intersection point

    // find intersection points by finding intersection of sets
    let intersection = set_1.intersection(&set_2);

    // print the point with minimum manhattan distance
    println!("{:?}", intersection.map(|x| x.0.abs() + x.1.abs()).min());
}

fn get_direction(l: &str) -> Direction {
    // function to map wire 'code' to a direction type
    match l.chars().next() {
        Some('L') => Direction::Left,
        Some('R') => Direction::Right,
        Some('U') => Direction::Up,
        Some('D') => Direction::Down,
        Some(_)   => Direction::None,
        None      => Direction::None,
    }
}

fn trace(mut set: HashSet<(i32,i32)>, wire: Vec<&str>) -> HashSet<(i32,i32)> {
    // Function to 'trace' a route of the wire in a HashSet

    // Instantiate a `Point`
    let mut point: Point = Point { x: 0, y: 0 };

    for x in wire.iter() {
        let direction = get_direction(x);
        let steps = x.get(1..).unwrap().parse::<i32>().unwrap();

        match direction {
            Direction::Left => {
                for i in 1..=steps {
                    set.insert((point.x - i, point.y));
                }
                point = Point { x: point.x - steps, y: point.y};
            },
            Direction::Right => {
                for i in 1..=steps {
                    set.insert((point.x + i, point.y));
                }
                point = Point { x: point.x + steps, y: point.y};
            },
            Direction::Up => {
                for i in 1..=steps {
                    set.insert((point.x, point.y + i));
                }
                point = Point { x: point.x , y: point.y + steps};
            },
            Direction::Down => {
                for i in 1..=steps {
                    set.insert((point.x, point.y - i));
                }
                point = Point { x: point.x , y: point.y - steps};
            },
            Direction::None => (),
        }
    }
    set // return the traced route
}
