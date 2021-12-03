use crate::common;
use regex::Regex;
use std::ops::AddAssign;


#[derive(Default, Debug, Copy, Clone)]
struct Position {
    depth: i32,
    horizontal: i32,
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        self.depth += other.depth;
        self.horizontal += other.horizontal;
    }
}


fn parse_position(command: &str) -> Position {

    lazy_static::lazy_static! { // Only compile the regex once
        static ref RE: Regex = Regex::new(r"(\w+) (\d)").unwrap();
    }

    let capture = RE.captures(command).unwrap();
    let command = capture.get(1).unwrap().as_str();
    let amount: i32 = capture.get(2).unwrap().as_str().parse().unwrap();

    match command {
        "forward" => Position{horizontal: amount, depth: 0},
        "down" => Position{horizontal: 0, depth: amount},
        "up" => Position{horizontal: 0, depth: -amount},
        _ => panic!()
    }
}

pub fn challenge2() {
    challenge2_1();
    challenge2_2();
}

fn challenge2_1() {
    println!("Second day. Challenge 1");

    let mut position: Position = Position::default();

    let line_iter = common::read_lines(common::in_data_folder("challenge_2_1.txt"));
    for line in line_iter.flatten() {
        position += parse_position(&line);
    }

    println!("{:?}", position);
    println!("Result: {}", position.horizontal * position.depth)

}

fn challenge2_2() {
    println!("Second day. Challenge 2");

    let mut position: Position = Position::default();
    let mut aim: i32 = 0;

    let line_iter = common::read_lines(common::in_data_folder("challenge_2_1.txt"));
    for line in line_iter.flatten() {
        let read_command = parse_position(&line); // Due to implementation of part 1, this is unfortunately the same type
        aim += read_command.depth;
        position.horizontal += read_command.horizontal;
        position.depth += aim * read_command.horizontal;
    }

    println!("{:?}", position);
    println!("Result: {}", position.horizontal * position.depth)

}