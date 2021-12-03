use crate::common;

const NR_BITS: usize = 12;

/* Get bit at index index, starting at 0 */
fn get_bit(index: usize, value: u16) -> u16 {
    (value >> index) & 0x1
}

fn get_values() -> impl Iterator<Item = u16> {
    common::read_lines(common::in_data_folder("challenge_3_1.txt")).map(|x| u16::from_str_radix(&x.unwrap(), 2).expect("Non-integer in data"))
}

pub fn challenge3() {
    println!("Third day. Challenge 1");

    let values: Vec<u16> = get_values().collect();
    let nr_of_values = values.len() as u16;
    let mut gamma: u16 = 0;

    for index in 0..NR_BITS {
        let nr_ones: u16 = values.iter().map(|x| get_bit(index, *x)).sum();
        let most_common = (2*nr_ones >= nr_of_values) as u16;

        gamma += most_common << index;
    }

    let epsilon = (!gamma) & ((1 << (NR_BITS)) - 1);

    println!("Epsilon: {} Gamma: {}, Multiplied: {}", epsilon, gamma, epsilon as u32 * gamma as u32);

}
