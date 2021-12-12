use crate::common;
use std::collections::HashMap;

fn pop_with_n_sections(n: usize, seen: &mut Vec<String>) -> String {
    let index = seen.iter().position(|x| x.len() == n).unwrap();
    seen.swap_remove(index)
}


fn get_mapping(seen: &Vec<String>) -> HashMap<String, u8> {
    let mut reverse_map = HashMap::new();
    let mut seen_left = seen.clone();

    /* 
        8 has 7 segments
        1 has 2 segments
        4 has 4 segments
        7 has 3 segments
        9 has 6 semenents including all sections from 4
        0 has 6 semenents including all sections from 1
        6 has 6 segments
        3 has 5 semenents including all sections from 1
        5 has 5 semenents including all but one section from 6
        2 has 5 segments
    */
    reverse_map.insert(8, pop_with_n_sections(7, &mut seen_left));
    reverse_map.insert(1, pop_with_n_sections(2, &mut seen_left));
    reverse_map.insert(4, pop_with_n_sections(4, &mut seen_left));
    reverse_map.insert(7, pop_with_n_sections(3, &mut seen_left));
    let index_of9 = seen_left.iter().position(|x| (x.len() == 6) && reverse_map[&4].chars().all(|c| x.contains(c))).unwrap();
    reverse_map.insert(9, seen_left.swap_remove(index_of9));
    let index_of0 = seen_left.iter().position(|x| (x.len() == 6) && reverse_map[&1].chars().all(|c| x.contains(c))).unwrap();
    reverse_map.insert(0, seen_left.swap_remove(index_of0));
    reverse_map.insert(6, pop_with_n_sections(6, &mut seen_left));
    let index_of3 = seen_left.iter().position(|x| (x.len() ==  5) && reverse_map[&1].chars().all(|c| x.contains(c))).unwrap();
    reverse_map.insert(3, seen_left.swap_remove(index_of3));
    let index_of5 = seen_left.iter().position(|x| (x.len() ==  5) && reverse_map[&6].chars().filter(|&c| !x.contains(c)).count() == 1).unwrap();
    reverse_map.insert(5, seen_left.swap_remove(index_of5));
    assert_eq!(seen_left.len(), 1);
    reverse_map.insert(2, seen_left.pop().unwrap());

    reverse_map.iter().map(|kv| (kv.1.clone(), *kv.0)).collect()
}

fn sortstr(string: &str) -> String {
    let mut chars: Vec<char> = string.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    String::from_iter(chars)
}

pub fn challenge8() {
    println!("Eigth day");
    let lines: Vec<String> = common::read_lines(common::in_data_folder("challenge_8_1.txt")).flatten().collect();

    let mut part1_count = 0;
    let mut part2_sum: u32 = 0;

    for line in lines.iter() {
        let mut split = line.split("|");
        let calib_line = split.next().unwrap().trim().to_owned();

        let values_line = split.next().unwrap().trim().to_owned();

        let part2_mappings = get_mapping(&calib_line.split(" ").map(|x| sortstr(x)).collect());


        let numbers: Vec<u8> = values_line.split(" ").map(|x| *part2_mappings.get(&sortstr(x)).unwrap_or_else(|| panic!("{} should be in the map", sortstr(x)))).collect();
        part1_count += numbers.iter().filter(|&x| match x {
            1 => true,
            4 => true,
            7 => true,
            8 => true,
            _ => false
        }).count();

        fn vec_to_number(x: &Vec<u8>) -> u32 {
            x[0] as u32 * 1000 + x[1] as u32 * 100 + x[2] as u32 * 10 + x[3] as u32
        }

        part2_sum += vec_to_number(&numbers)

    }

    println!("Number of 1s,4s,7s and 8s is: {}", part1_count);
    println!("Sum of readings is: {}", part2_sum);
}
