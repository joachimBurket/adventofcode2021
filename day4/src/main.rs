use std::fs;
mod bingo;
use bingo::BingoBoard;
use ndarray::prelude::*;
use regex::Regex;

fn create_bingo_boards(content: &str) -> Vec<BingoBoard> {
    let mut boards: Vec<BingoBoard> = Vec::new();

    let lines = content.lines().skip(2);    // skip the two first lines
    let mut board: Vec<u32> = Vec::new();
    
    for line in lines {
        if !line.is_empty() {
            let re = Regex::new(r"[ \t]+").unwrap();
            let mut elts: Vec<u32> = re.split(line.trim())
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            board.append(&mut elts);
        }
        else {
            // board is complete
            let board_array = Array::from_shape_vec((5,5), board).unwrap();
            let mut id: u32 = boards.len().try_into().unwrap();
            id += 1;
            println!("Board completed:\n{:?}", board_array);
            let bingo_board = BingoBoard::new(
                id,
                board_array
            );
            boards.push(bingo_board);
            
            board = Vec::new(); // re-init board
        }
    }
    boards
}

fn get_numbers(content: &str) -> Vec<u32> {
    content
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

/// Returns the index of the winning card in the Vector of boards and the last number yelded
fn play_bingo1(boards: &mut Vec<BingoBoard>, numbers: &Vec<u32>) -> (Option<u32>, u32) {
    println!("------------------------------------------------------");
    println!("Let's play bingo (part 1)");
    println!("------------------------------------------------------");   

    for number in numbers {
        println!("***** NUMBER {} !! *****", number);

        for board in boards.iter_mut() {

            if let Some(pos) = board.check_number(*number) {
                if board.is_full_line(pos) {
                    return (Some(board.id), *number);
                }
            }
        }
    }
    (None, numbers[numbers.len()-1])
}

/// Returns the index of the loosing card in the Vector of boards and the last number yelded
fn play_bingo2(boards: &mut Vec<BingoBoard>, numbers: &Vec<u32>) -> (Option<u32>, u32) {
    println!("------------------------------------------------------");
    println!("Let's play bingo (part 2)");
    println!("------------------------------------------------------");

    let number_of_boards = boards.len();
    let mut board_have_won = 0;

    for number in numbers {
        println!("***** NUMBER {} !! *****", number);

        for (i, board) in boards.iter_mut().enumerate() {
            if board.has_won {
                continue;
            }

            if let Some(pos) = board.check_number(*number) {
                if board.is_full_line(pos) {
                    board_have_won += 1;
                    if board_have_won >= number_of_boards {
                        return (Some(board.id), *number);
                    }
                }
            }
        }
    }
    (None, numbers[numbers.len()-1])
}

fn main() {
    // Part1
    const INPUT_FILENAME: &str = "resources/bingo";
    let content = fs::read_to_string(INPUT_FILENAME).unwrap();

    // pass content to game constructor method
    let mut boards = create_bingo_boards(&content);
    let numbers = get_numbers(&content);
    
    let (winner, last_number)= play_bingo1(&mut boards, &numbers);
    let winner: usize = winner.unwrap().try_into().unwrap();
    let score = boards[winner-1].calculate_score();
    println!("Board{} wins with {} points!", winner, score);
    
    let part1_answer = score * last_number;
    println!("Part1 answer: {}", part1_answer);

    // Part2
    // Reinitialize boards
    let mut boards = create_bingo_boards(&content);

    let (looser, last_number)= play_bingo2(&mut boards, &numbers);
    let looser: usize = looser.unwrap().try_into().unwrap();
    let score = boards[looser-1].calculate_score();
    println!("Board{} looses with {} points!", looser, score);
    
    let part2_answer = score * last_number;
    println!("Part2 answer: {}", part2_answer);
}
