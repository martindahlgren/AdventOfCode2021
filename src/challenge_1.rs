use crate::common;

pub fn challenge1() {
    println!("First Advent. Challenge 1");

    let values = get_values().collect::<Vec<_>>();
    let nincreases = number_of_increases(&values);
    println!("There are {} positive steps", nincreases);

    println!("First Advent. Challenge 2");
    let moving_sum: Vec<i16> = values.windows(3).map(|w| w[0] + w[1] + w[2]).collect();
    let nincreases_three_summed = number_of_increases(&moving_sum);
    println!("There are {} positive steps when making a moving average(/sum)", nincreases_three_summed);

}

fn number_of_increases(input: &Vec<i16>) -> usize {
    input.windows(2).filter(|w| w[0] < w[1]).count()
}

fn get_values() -> impl Iterator<Item = i16> {
    common::read_lines(common::in_data_folder("challenge_1_1.txt")).map(|x| x.unwrap().parse::<i16>().expect("Non-integer in data"))
}