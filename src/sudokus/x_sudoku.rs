use super::sudoku_rule::SudokuRule;
use super::{has_duplicates, Field, Mask, ParseError};

/// X-Sudoku: Both diagonals must contain every digit exactly once.
#[derive(Debug, Clone, Copy, Default)]
pub struct XSudoku;

impl XSudoku {
    /// Name of the section in a sudoku file
    pub const IDENTIFIER: &'static str = "x-sudoku";

    /// Parses the content of the `x-sudoku` section, which has to be empty
    pub fn parse(body: &str) -> Result<Self, ParseError> {
        if !body.trim().is_empty() {
            return Err(ParseError::InvalidRule {
                rule: Self::IDENTIFIER.to_string(),
                reason: "the section takes no content".to_string(),
            });
        }
        Ok(Self)
    }

    /// Cells from top left to bottom right
    fn main_diagonal(field: &Field) -> impl Iterator<Item = Mask> + '_ {
        (0..9).map(|i| field[i][i])
    }

    /// Cells from top right to bottom left
    fn anti_diagonal(field: &Field) -> impl Iterator<Item = Mask> + '_ {
        (0..9).map(|i| field[i][8 - i])
    }
}

impl SudokuRule for XSudoku {
    fn excluded(&self, field: &Field, row: usize, column: usize) -> Mask {
        let mut excluded = 0;

        // The center cell is on both diagonals
        if row == column {
            excluded |= Self::main_diagonal(field).fold(0, |set, cell| set | cell);
        }
        if row + column == 8 {
            excluded |= Self::anti_diagonal(field).fold(0, |set, cell| set | cell);
        }

        excluded
    }

    fn complies(&self, field: &Field) -> bool {
        !has_duplicates(Self::main_diagonal(field)) && !has_duplicates(Self::anti_diagonal(field))
    }
}

#[cfg(test)]
mod x_sudoku_test {
    use super::XSudoku;
    use crate::sudokus::sudoku_rule::SudokuRule;
    use crate::sudokus::{digit_mask, field_from_digits};

    // Numbers from: https://de.wikipedia.org/wiki/Sudoku#/media/Datei:Sudoku_variant.png
    const GOOD_FIELD: [[u8; 9]; 9] = [
        [1, 2, 3, 7, 8, 9, 4, 5, 6],
        [4, 5, 6, 1, 2, 3, 7, 8, 9],
        [7, 8, 9, 4, 5, 6, 1, 2, 3],
        [2, 3, 1, 8, 9, 7, 5, 6, 4],
        [5, 6, 4, 2, 3, 1, 8, 9, 7],
        [8, 9, 7, 5, 6, 4, 2, 3, 1],
        [3, 1, 2, 9, 7, 8, 6, 4, 5],
        [6, 4, 5, 3, 1, 2, 9, 7, 8],
        [9, 7, 8, 6, 4, 5, 3, 1, 2],
    ];

    #[test]
    fn complies() {
        let x = XSudoku;
        let mut field = field_from_digits(GOOD_FIELD);
        assert!(x.complies(&field));

        // Main diagonal: 5 at (1,1) appears a second time
        field[0][0] = digit_mask(5);
        assert!(!x.complies(&field));

        // Anti diagonal: 9 at (8,0) appears a second time
        let mut field = field_from_digits(GOOD_FIELD);
        field[0][8] = digit_mask(9);
        assert!(!x.complies(&field));
    }

    #[test]
    fn excluded() {
        let x = XSudoku;
        let mut digits = [[0; 9]; 9];
        digits[0][0] = 1;
        digits[8][0] = 2;
        let field = field_from_digits(digits);

        // Cells on the main diagonal can't be 1, on the anti diagonal not 2
        assert_eq!(x.excluded(&field, 3, 3), digit_mask(1));
        assert_eq!(x.excluded(&field, 2, 6), digit_mask(2));
        assert_eq!(x.excluded(&field, 4, 4), digit_mask(1) | digit_mask(2));

        // Cells off the diagonals are not affected
        assert_eq!(x.excluded(&field, 0, 1), 0);
        assert_eq!(x.excluded(&field, 4, 5), 0);
    }

    #[test]
    fn parse() {
        assert!(XSudoku::parse("").is_ok());
        assert!(XSudoku::parse(" \n").is_ok());
        assert!(XSudoku::parse("1 2 3").is_err());
    }
}
