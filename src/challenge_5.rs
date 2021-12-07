use crate::common;
use regex::Regex;
use std::collections::HashMap;
use std::cmp::min;
use std::cmp::max;

#[derive(Debug, Copy, Clone)]
struct Line {
    x0: i16,
    y0: i16,
    x1: i16,
    y1: i16,
}

fn get_input() -> Vec<Line> {
    fn parse_line(input: &str) -> Line {
        lazy_static::lazy_static! { // Only compile the regex once
            static ref RE: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        }

        let capture = RE.captures(input).unwrap();
        Line {
            x0: capture.get(1).unwrap().as_str().parse().unwrap(),
            y0: capture.get(2).unwrap().as_str().parse().unwrap(),
            x1: capture.get(3).unwrap().as_str().parse().unwrap(),
            y1: capture.get(4).unwrap().as_str().parse().unwrap(),
        }
    }

    common::read_lines(common::in_data_folder("challenge_5_1.txt"))
        .map(|x| parse_line(&x.unwrap()))
        .collect()
}

fn get_points(line: Line, consider_diagonal: bool) -> Vec<(i16, i16)> {
    let output: Vec<(i16,i16)>;

    output = if line.x0 == line.x1 {
        (min(line.y1, line.y0)..=max(line.y1, line.y0)).map(|y| (line.x0, y)).collect()
    } else if line.y0 == line.y1 {
        (min(line.x1, line.x0)..=max(line.x1, line.x0)).map(|x| (x, line.y0)).collect()
    } else if consider_diagonal {
        let reverse_y = (line.y1 < line.y0 && line.x0 < line.x1) || (line.y0 < line.y1 && line.x1 < line.x0);
        let x_iter = min(line.x1, line.x0)..=max(line.x1, line.x0);
        let y_iter = min(line.y1, line.y0)..=max(line.y1, line.y0);
        if reverse_y {
            x_iter.zip(y_iter.rev()).collect()
        } else {
            x_iter.zip(y_iter).collect()
        }
    } else {
        Vec::new()
    };


    return output;
}

pub fn challenge5() {
    println!("Fifth day");
    let lines = get_input();

    for (challenge, consider_diagonal) in [false, true].into_iter().enumerate() {
        println!("Challenge {}", challenge);
        let mut counts = HashMap::new();
        for line in lines.iter() {
            for p in get_points(*line, consider_diagonal) {
                *counts.entry(p).or_insert(0) += 1;
            }
        }
    
        let morethan2 = counts.values().filter(|x| **x > 1).count();
    
        println!("Answer: {}", morethan2);
    }

}
