use crate::common;

pub fn challenge7() {
    println!("Seventh day. Challenge 1");

    let mut positions: Vec<u64> =
    common::read_lines(common::in_data_folder("challenge_7_1.txt"))
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    positions.sort();
    // The median is the best place to go to
    let median = positions[positions.len()/2];
    let traveled: u64 = positions.iter().map(|&x| if x > median {x - median} else {median - x}).sum();
    println!("Traveled to {} took {} fuel", median, traveled);

    println!("Challenge 2");

    fn sum_of_integers(n: u64) -> u64 {
        (n*(n + 1))/2
    }

    let min_pos = *positions.iter().min().unwrap();
    let max_pos = *positions.iter().max().unwrap();
    let mut least_fuel = 9999999999;
    let mut best_place = 0;
    for pos in min_pos..=max_pos {
        let fuel = positions.iter().map(|&x| sum_of_integers(if x > pos {x - pos} else {pos - x})).sum();
        if fuel < least_fuel {
            least_fuel = fuel;
            best_place = pos;
        }
    }

    println!("Traveled to {} took {} fuel", best_place, least_fuel);

}
