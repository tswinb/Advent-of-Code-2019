use std::fs;
use std::collections::HashMap;
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

    // hashmap containing point and distance
    let mut dict_1: HashMap<(i32,i32),i32> = HashMap::new();
    let mut dict_2: HashMap<(i32,i32),i32> = HashMap::new();

    // add origin
    dict_1.insert((0,0),0);
    dict_2.insert((0,0),0);

    dict_1 = trace(dict_1, wire_1);
    dict_2 = trace(dict_2, wire_2);


    dict_1.remove(&(0,0)); // origin should not count as intersection point

    let set_1: HashSet<&(i32,i32)> = dict_1.keys().collect::<HashSet<_>>();
    let set_2: HashSet<&(i32,i32)> = dict_2.keys().collect::<HashSet<_>>();

    // find intersection points by finding intersection of dicts
    let intersection = set_1.intersection(&set_2);

    // print the fewest combined steps the wires must take to reach an intersection?
    println!("{:?}", intersection.map(|x| dict_1.get(x).unwrap() + dict_2.get(x).unwrap()).min().unwrap());

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

fn trace(mut dict: HashMap<(i32,i32),i32>, wire: Vec<&str>) -> HashMap<(i32,i32),i32> {
    // Function to 'trace' a route of the wire in a Hashdict

    // Instantiate a `Point`
    let mut point: Point = Point { x: 0, y: 0 };

    let mut distance = 0;

    for x in wire.iter() {
        let direction = get_direction(x);
        let steps = x.get(1..).unwrap().parse::<i32>().unwrap();

        match direction {
            Direction::Left => {
                for i in 1..=steps {
                    distance = distance + 1;
                    dict.insert((point.x - i, point.y),distance);
                }
                point = Point { x: point.x - steps, y: point.y};
            },
            Direction::Right => {
                for i in 1..=steps {
                    distance = distance + 1;
                    dict.insert((point.x + i, point.y),distance);
                }
                point = Point { x: point.x + steps, y: point.y};
            },
            Direction::Up => {
                for i in 1..=steps {
                    distance = distance + 1;
                    dict.insert((point.x, point.y + i),distance);
                }
                point = Point { x: point.x , y: point.y + steps};
            },
            Direction::Down => {
                for i in 1..=steps {
                    distance = distance + 1;
                    dict.insert((point.x, point.y - i),distance);
                }
                point = Point { x: point.x , y: point.y - steps};
            },
            Direction::None => (),
        }
    }
    dict // return the traced route
}
