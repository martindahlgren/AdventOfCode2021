use crate::common;

const NR_BITS: u16 = 12;
const NR_BITS_MASK: u16 = (1 << (NR_BITS)) - 1;

/* Get bit at index index, starting at 0 */
fn get_bit(index: u16, value: u16) -> u16 {
    (value >> index) & 0x1
}

fn get_values() -> impl Iterator<Item = u16> {
    common::read_lines(common::in_data_folder("challenge_3_1.txt")).map(|x| u16::from_str_radix(&x.unwrap(), 2).expect("Non-integer in data"))
}

/* For each bit (starting MSB) only keep the values according to selection critera until only one value remains
   For each value and each index the selection criteria can be based only on the most common bit and the bit in the value */
fn part2<F : Fn(u16, u16) -> bool>(values: &Vec<u16>, selection_critera: F) -> u16 {
    let mut remaining_options = values.clone();
    for index in 0..NR_BITS {
        let nr_of_values = remaining_options.len() as u16;
        let bit = NR_BITS - index - 1;
        let nr_ones: u16 = remaining_options.iter().map(|x| get_bit(bit, *x)).sum();
        let most_common = (2*nr_ones >= nr_of_values) as u16;
        remaining_options = remaining_options.into_iter().filter(|x| selection_critera(get_bit(bit, *x), most_common)).collect();
        if remaining_options.len() == 1 {
            break;
        }
    }
    remaining_options[0]
}

pub fn challenge3() {
    println!("Third day. Challenge 1");

    let values: Vec<u16> = get_values().collect();

    let mut gamma: u16 = 0; // Most common bit for each index
    for index in 0..NR_BITS {
        let nr_of_values = values.len() as u16;
        let nr_ones: u16 = values.iter().map(|x| get_bit(index, *x)).sum();
        let most_common = (2*nr_ones >= nr_of_values) as u16;
    
        gamma += most_common << index;
    }
    let epsilon = (!gamma) & NR_BITS_MASK; // Least common


    println!("Epsilon: {} Gamma: {}, Multiplied: {}", epsilon, gamma, epsilon as u32 * gamma as u32);

    println!("Third day. Challenge 2");
    let oxygen = part2(&values, |bit: u16, most_common: u16 | bit == most_common);
    let scrubber = part2(&values, |bit: u16, most_common: u16 | bit != most_common);

    println!("Oxygen Level: {:?} CO2 Scrubber: {}, Multiplied: {}", oxygen, scrubber, oxygen as u32 * scrubber as u32);


}
