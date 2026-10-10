use super::parse::parse_grid;
use super::sudoku_rule::SudokuRule;
use super::{Field, Mask, ParseError};

/// Digits 1, 3, 5, 7 and 9
const ODD_DIGITS: Mask = 0b101_010_101;
/// Digits 2, 4, 6 and 8
const EVEN_DIGITS: Mask = 0b010_101_010;

/// Even-Odd-Sudoku: Every cell is marked as even or odd.
///
/// In the file `0` marks an even cell and `1` an odd cell.
#[derive(Debug, Clone)]
pub struct EvenOddSudoku {
    even_cells: [[bool; 9]; 9],
}

impl EvenOddSudoku {
    /// Name of the section in a sudoku file
    pub const IDENTIFIER: &'static str = "even-odd-sudoku";

    /// A classic sudoku has 4 even digits in each of the 9 rows
    const EVEN_CELL_COUNT: usize = 36;

    /// Creates the rule from the even cells, indexed as `even_cells[row][column]`
    pub fn new(even_cells: [[bool; 9]; 9]) -> Result<Self, ParseError> {
        let count = even_cells.iter().flatten().filter(|&&even| even).count();
        if count != Self::EVEN_CELL_COUNT {
            return Err(ParseError::InvalidRule {
                rule: Self::IDENTIFIER.to_string(),
                reason: format!(
                    "{} cells must be even, but {count} are",
                    Self::EVEN_CELL_COUNT
                ),
            });
        }
        Ok(Self { even_cells })
    }

    /// Parses the content of the `even-odd-sudoku` section
    pub fn parse(body: &str) -> Result<Self, ParseError> {
        let even_cells = parse_grid(Self::IDENTIFIER, body, |c| match c {
            '0' => Some(true),
            '1' => Some(false),
            _ => None,
        })?;
        Self::new(even_cells)
    }

    /// Returns true if the cell has to be even
    pub fn is_even(&self, row: usize, column: usize) -> bool {
        self.even_cells[row][column]
    }
}

impl SudokuRule for EvenOddSudoku {
    #[inline]
    fn excluded(&self, _field: &Field, row: usize, column: usize) -> Mask {
        if self.is_even(row, column) {
            ODD_DIGITS
        } else {
            EVEN_DIGITS
        }
    }

    fn complies(&self, field: &Field) -> bool {
        (0..9).all(|row| {
            (0..9).all(|column| field[row][column] & self.excluded(field, row, column) == 0)
        })
    }
}

#[cfg(test)]
mod even_odd_sudoku_test {
    use super::EvenOddSudoku;
    use crate::sudokus::sudoku_rule::SudokuRule;
    use crate::sudokus::{digit_mask, ParseError};

    /// Even cells are the ones with an even column
    fn mask() -> String {
        (0..81)
            .map(|i| if i % 9 % 2 == 1 { "0 " } else { "1 " })
            .collect()
    }

    #[test]
    fn parse() {
        let rule = EvenOddSudoku::parse(&mask()).unwrap();
        assert!(rule.is_even(0, 1));
        assert!(!rule.is_even(0, 0));
        assert!(rule.is_even(8, 7));
    }

    #[test]
    fn parse_errors() {
        // All cells odd
        assert!(matches!(
            EvenOddSudoku::parse(&"1 ".repeat(81)),
            Err(ParseError::InvalidRule { .. })
        ));
        // 36 odd and 45 even cells
        let swapped = mask().replace('0', "x").replace('1', "0").replace('x', "1");
        assert!(matches!(
            EvenOddSudoku::parse(&swapped),
            Err(ParseError::InvalidRule { .. })
        ));
        assert!(matches!(
            EvenOddSudoku::parse("2"),
            Err(ParseError::InvalidCharacter { character: '2', .. })
        ));
    }

    #[test]
    fn complies_and_excluded() {
        let rule = EvenOddSudoku::parse(&mask()).unwrap();
        let mut field = [[0; 9]; 9];
        assert!(rule.complies(&field));

        field[0][1] = digit_mask(4);
        field[0][0] = digit_mask(7);
        assert!(rule.complies(&field));

        field[0][1] = digit_mask(3);
        assert!(!rule.complies(&field));

        // Even cells exclude odd digits and the other way around
        assert_eq!(rule.excluded(&field, 0, 1) & digit_mask(3), digit_mask(3));
        assert_eq!(rule.excluded(&field, 0, 1) & digit_mask(4), 0);
        assert_eq!(rule.excluded(&field, 0, 0) & digit_mask(4), digit_mask(4));
    }
}
