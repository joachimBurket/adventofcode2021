extern crate ndarray;
use ndarray::prelude::*;


pub struct BingoBoard {
    pub id: u32,
    pub has_won: bool,
    board: Array2<u32>,
    checked: Array2<u32>,
    side_length: usize,
}


impl BingoBoard {

    pub fn new(id: u32, board: Array2<u32>) -> BingoBoard {
        let side_length = board.nrows();
        let checked = Array2::<u32>::zeros((side_length,side_length));

        BingoBoard {
            id,
            has_won: false,
            board,
            checked,
            side_length,
        }
    }

    pub fn check_number(&mut self, number: u32) -> Option<(usize,usize)> {
        for (y, row) in self.board.outer_iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if number == *val {
                    let position = (y, x);
                    println!("Card{}: I Found number {} in pos {:?}", self.id, number, position);
                    self.checked[position] = 1;
                    return Some(position);
                }
            }
        }
        None
    }

    /// Check if a line (row or col) is full 
    pub fn is_full_line(&mut self, crossing: (usize, usize)) -> bool {
        let row_sum = self.checked.index_axis(Axis(0), crossing.0).sum();
        let col_sum = self.checked.index_axis(Axis(1), crossing.1).sum();
        let side_length: u32 = self.side_length.try_into().unwrap();

        if (row_sum == side_length) || col_sum == side_length {
            self.has_won = true;
            return true;
        }
        return false;
    }

    pub fn calculate_score(&self) -> u32 {
        let filtered: Vec<u32> = self.board
            .indexed_iter()
            .filter(|x| self.checked[x.0] == 0)
            .map(|x| *x.1)
            .collect();
        filtered.iter().sum()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_board() {
        let array = array![
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ];
        let board: BingoBoard = BingoBoard::new(1, array);

        assert_eq!(board.id, 1);
        assert_eq!(board.side_length, 3);
        assert_eq!(board.checked.nrows(), 3);
        assert_eq!(board.board[[1,1]], 5);
        assert_eq!(board.checked[[0,0]], 0);
    }

    #[test]
    fn test_check_number() {
        let array = array![
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ];
        let mut board: BingoBoard = BingoBoard::new(1, array);

        assert_eq!(Some((1,2)), board.check_number(6));
        assert_eq!(board.checked[[1,2]], 1);
        assert_eq!(Some((2, 1)), board.check_number(8));
        assert_eq!(board.checked[[2, 1]], 1);
        assert_eq!(None, board.check_number(36));
    }

    #[test]
    fn test_is_full_line() {
        let array = array![
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ];
        let mut board: BingoBoard = BingoBoard::new(1, array);
        board.checked[[0,0]] = 1;
        board.checked[[0,1]] = 1;
        
        // No full line
        assert_eq!(board.is_full_line((0,0)), false);
        assert_eq!(board.is_full_line((1,2)), false);

        board.checked[[0,2]] = 1;

        // First row is full
        assert_eq!(board.is_full_line((0,0)), true);
        assert_eq!(board.is_full_line((0,2)), true);

        board.checked[[1,2]] = 1;
        board.checked[[2,2]] = 1;

        // First row and last col are full
        assert_eq!(board.is_full_line((2,1)), false);
        assert_eq!(board.is_full_line((1,2)), true);
        assert_eq!(board.is_full_line((0,2)), true);
    }
}