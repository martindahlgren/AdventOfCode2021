use crate::common;
use  std::collections::HashSet;
use std::collections::VecDeque;

fn get_digit(lines: &Vec<String>, col: i16, row: i16) -> u32 {
    let n_rows = lines.len();
    let n_cols = lines[0].len();

    return if row == -1 || col == -1 || col == n_cols as i16 || row == n_rows as i16 {
        9
    } else {
        lines[row as usize].chars().nth(col as usize).unwrap().to_digit(10).unwrap()
    }
}


fn get_low_points(lines: &Vec<String>) -> Vec<(u8, (i16, i16))> {
    

    let n_rows = lines.len();
    let n_cols = lines[0].len();

    fn min_condition(row: i16, col: i16, lines: &Vec<String>) -> Option<(u8, (i16, i16))> {

        let center = get_digit(lines, col, row);
        let above = get_digit(lines, col, row - 1);
        let below = get_digit(lines, col, row + 1);
        let left = get_digit(lines, col - 1, row);
        let right = get_digit(lines, col + 1, row);

        if center < above && center < below && center < left && center < right {
            Some((center as u8, (col, row)))
        } else {
            None
        }

    }

    (0..n_rows).map(|row_i| {let lines = lines; (0..n_cols).map(move |col_i| min_condition(row_i as i16, col_i as i16, lines))}).flatten().filter_map(|x| x).collect()

}

fn get_basin_size(lines: &Vec<String>, low_point: (i16, i16)) -> usize {

    let mut added: HashSet<(i16,i16)> = HashSet::from([low_point]);
    let mut to_check: VecDeque<(i16, i16)> = VecDeque::from([low_point]);
    let mut size: usize = 0;

    while !to_check.is_empty() {
        let (col, row) = to_check.pop_front().unwrap();
        if get_digit(lines, col, row) < 9 {
            size += 1;
            let surrounding = [(col + 1, row), (col - 1, row), (col, row + 1), (col, row - 1)];
            to_check.extend(surrounding.iter().filter(|(col, row)| added.insert((*col, *row))));
        }
    }

    size

}

pub fn challenge9() {
    println!("Ninth day");
    let lines: Vec<String> = common::read_lines(common::in_data_folder("challenge_9_1.txt")).flatten().collect();
    let low_points = get_low_points(&lines);
    let answer1: u16 = low_points.iter().map(|&x| x.0 as u16 + 1).sum();

    let mut basin_sizes: Vec<usize> = low_points.iter().map(|(_, low_point)| get_basin_size(&lines, *low_point)).collect();
    basin_sizes.sort();
    let answer2 = basin_sizes[basin_sizes.len() -1] * basin_sizes[basin_sizes.len() - 2] * basin_sizes[basin_sizes.len() - 3];
    println!("First {:?}, second {}", answer1, answer2);
}
