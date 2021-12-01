use crate::common;

pub fn challenge1() {
    println!("First Advent. Challenge 1");

    let nincreases = get_values().collect::<Vec<_>>().windows(2).filter(|w| w[0] < w[1]).count();
    println!("There are {} positive steps", nincreases);




}

fn get_values() -> impl Iterator<Item = i16> {
    common::read_lines(common::in_data_folder("challenge_1_1.txt")).map(|x| x.unwrap().parse::<i16>().expect("Non-integer in data"))
}