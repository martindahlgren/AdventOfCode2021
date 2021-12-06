use crate::common;

const DAYS: u16 = 256;
const FISH_STATES: usize = 9;

pub fn challenge6() {
    println!("Sixth day. Challenge 1");

    let initial_fishes: Vec<u8> =
        common::read_lines(common::in_data_folder("challenge_6_1.txt"))
            .next()
            .unwrap()
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();

    let mut fish_counters: [u64; FISH_STATES] = [0; FISH_STATES];

    for f in initial_fishes.into_iter() {
        fish_counters[f as usize] += 1;
    }

    for _ in  0..DAYS {
        fish_counters.rotate_left(1);
        fish_counters[6] += fish_counters[8];
    }

    println!("{:?}", fish_counters);
    let nr_fish: u64 = fish_counters.into_iter().map(|x| x as u64).sum();

    println!("Number of fish after {} days: {}", DAYS, nr_fish);


}
