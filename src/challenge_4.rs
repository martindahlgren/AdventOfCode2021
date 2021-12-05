use crate::common;

const NR_WIN_CONDITIONS_PER_BOARD:usize = 10;
const BOARD_SIDE:usize = 5;
const BOARD_N_ELEMENTS:usize = BOARD_SIDE*BOARD_SIDE;

type BoardNum = u8;

#[derive(Debug, Default, Clone)]
struct Element {
    value: BoardNum,
    is_picked: bool
}

#[derive(Debug, Default, Clone)]
struct Board {
    left_until_win: [u8; NR_WIN_CONDITIONS_PER_BOARD],
    elements: [Element; BOARD_N_ELEMENTS], /* By rows */
}

impl Board {
    fn new(values: &[BoardNum; BOARD_N_ELEMENTS]) -> Board {
        let mut board = Board::default();
        for (i, &v) in values.iter().enumerate() {
            board.elements[i].value = v;
        }
        board.left_until_win = [BOARD_SIDE as BoardNum; NR_WIN_CONDITIONS_PER_BOARD];
    return board
    }

    // Returns true if board just bingoed!
    fn drawn(self: &mut Self, val: BoardNum) -> bool {

        for i in 0..BOARD_N_ELEMENTS {
            if self.elements[i].value == val {
                self.elements[i].is_picked = true;
                self.left_until_win[i/BOARD_SIDE] -= 1; // Row win condition
                self.left_until_win[i % BOARD_SIDE + BOARD_SIDE] -= 1; // Col win condition
            }
        }

        return self.left_until_win.iter().any(|l| *l == 0);
    }

    fn sum_all_unmarked(self: &Self) -> u32 {
        self.elements.iter().filter(|e| !e.is_picked).map(|e| e.value as u32).sum()
    }


}


pub fn challenge4() {
    println!("Fourth day. Challenge 1");

    let mut boards: Vec<Board> = Vec::new();

    let mut line_iter = common::read_lines(common::in_data_folder("challenge_4_1.txt"));
    let drawn: Vec<BoardNum> = line_iter.next().unwrap().unwrap().split(",").map(|x| x.parse().unwrap()).collect();

    let mut row_count = 0;
    let mut elements_count = 0;

    /* Construct boards */
    let mut elements: [BoardNum; BOARD_N_ELEMENTS] = [0; BOARD_N_ELEMENTS];
    for row in line_iter {
        for val in row.unwrap().split_whitespace().map(|x| x.parse().unwrap()) {
            elements[elements_count] = val;
            elements_count += 1;
        }
        row_count += 1;

        if row_count == 6 {
            assert_eq!(elements_count, BOARD_N_ELEMENTS);
            elements_count = 0;
            row_count = 0;
            println!("{:?}", elements);
            boards.push(Board::new(&elements));
        }
    }


    /* Draw numbers until bingo */
    let mut winning_board = None;
    let mut winning_num = None;
    let mut last_board = None;
    let mut last_num = None;
    for d in drawn {
        let mut idces_to_remove = Vec::new();
        for (idx, board) in boards.iter_mut().enumerate() {
            if board.drawn(d) {
                println!("BINGO!");
                if winning_board.is_none() {
                    winning_board = Some(board.clone());
                    winning_num = Some(d);
                }
                last_board = Some(board.clone());
                last_num = Some(d);
                idces_to_remove.push(idx);
            }
        }
        idces_to_remove.reverse();
        for idx in idces_to_remove {
            boards.remove(idx);
        }
    }


    /* Multiply elements */
    let answer = winning_board.expect("Somene needs to get bingo...").sum_all_unmarked() * winning_num.unwrap() as u32;
    println!("Winning board answer: {}", answer);

    let answer = last_board.unwrap().sum_all_unmarked() * last_num.unwrap() as u32;
    println!("Losing board answer: {}", answer);


}
