use super::sudoku_rule::SudokuRule;

#[derive(Debug, Clone)]
pub struct XSudoku {}

impl SudokuRule for XSudoku {
    fn str_identifier() -> &'static str {
        "x-sudoku"
    }

    fn from_str(_input: &str) -> Box<Self> {
        Box::new(Self {})
    }

    fn complys(&self, field: &[[i32; 9]; 9]) -> bool {
        let mut tl_to_br = 0;
        let mut tr_to_bl = 0;

        for i in 0..9 {
            if field[i][i].count_ones() == 1 {
                if tl_to_br & field[i][i] != 0 {
                    return false;
                }

                tl_to_br |= field[i][i];
            }

            if field[8 - i][i].count_ones() == 1 {
                if tr_to_bl & field[8- i][i] != 0 {
                    return false;
                }

                tr_to_bl |= field[8 - i][i];
            }
        }
        true
    }

    fn get_not_possible_numbers_raw(
        &self,
        field: &[[i32; 9]; 9],
        row: usize,
        collum: usize,
    ) -> i32 {
        // If the cell is not on the x their is no number that is not possible
        if row != collum || row != 9 - collum {
            return 0;
        }
        let mut tl_to_br = 0;
        let mut tr_to_bl = 0;

        for i in 0..9 {
            if field[i][i].count_ones() == 1 {
                tl_to_br |= field[i][i];
            }

            if field[8 - i][i].count_ones() == 1 {
                tr_to_bl |= field[8 - i][i];
            }
        }

        if row == 4 && collum == 4 {
            return tr_to_bl | tl_to_br;
        }

        if row == collum {
            return tl_to_br;
        }

        tr_to_bl
    }
}

#[cfg(test)]
mod x_sudoku_test {
    use crate::sudokus::sudoku_rule::SudokuRule;
    use super::XSudoku;

    #[test]
    fn comply() {
        // Numbers from: https://de.wikipedia.org/wiki/Sudoku#/media/Datei:Sudoku_variant.png
        let good_field = [[1,2,3,7,8,9,4,5,6],
        [4,5,6,1,2,3,7,8,9],
        [7,8,9,4,5,6,1,2,3],
        [2,3,1,8,9,7,5,6,4],
        [5,6,4,2,3,1,8,9,7],
        [8,9,7,5,6,4,2,3,1],
        [3,1,2,9,7,8,6,4,5],
        [6,4,5,3,1,2,9,7,8],
        [9,7,8,6,4,5,3,1,2]];

        let bad_field = [[2,2,3,7,8,9,4,5,6],
        [4,5,6,1,2,3,7,8,9],
        [7,8,9,4,5,6,1,2,3],
        [2,3,1,8,9,7,5,6,4],
        [5,6,4,2,3,1,8,9,7],
        [8,9,7,5,6,4,2,3,1],
        [3,1,2,9,7,8,6,4,5],
        [6,4,5,3,1,2,9,7,8],
        [9,7,8,6,4,5,3,1,2]];

        let x = XSudoku::from_str("");
        assert!(x.complys(&good_field));
        assert!(!x.complys(&bad_field));
        
    }
}
