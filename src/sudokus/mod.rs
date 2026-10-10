mod even_odd_sudoku;
mod fast_sudoku;
mod moves;
mod parse;
mod sudoku;
mod sudoku_rule;
mod x_sudoku;

pub use even_odd_sudoku::EvenOddSudoku;
pub use fast_sudoku::FastSudoku;
pub use moves::Moves;
pub use parse::ParseError;
pub use sudoku::{GenericSudoku, Sudoku};
pub use sudoku_rule::{NoRules, RuleSet, SudokuRule, VariantRules};
pub use x_sudoku::XSudoku;

/// A set of digits stored as bits: bit `n - 1` stands for digit `n`.
///
/// ```text
/// 0b1:           1
/// 0b100:         3
/// 0b101:         1 and 3
/// 0b111_111_111: all digits
/// ```
pub type Mask = u16;

/// The cells of a sudoku, indexed as `field[row][column]`.
///
/// A set cell holds exactly one bit, an empty cell is `0`.
pub type Field = [[Mask; 9]; 9];

/// Mask with all digits from 1 to 9
pub(crate) const ALL_DIGITS: Mask = 0b111_111_111;

/// Returns the mask of a single digit (1 to 9)
#[inline]
pub(crate) const fn digit_mask(digit: u8) -> Mask {
    1 << (digit - 1)
}

/// Returns the digit of a mask with a single bit set
#[inline]
pub(crate) const fn mask_digit(mask: Mask) -> u8 {
    mask.trailing_zeros() as u8 + 1
}

/// Iterates over the single bits of a mask, lowest digit first
pub(crate) fn bits(mut mask: Mask) -> impl Iterator<Item = Mask> {
    std::iter::from_fn(move || {
        if mask == 0 {
            return None;
        }
        let lowest = mask & mask.wrapping_neg();
        mask ^= lowest;
        Some(lowest)
    })
}

/// Returns true if a digit appears more than once in the given cells
pub(crate) fn has_duplicates(cells: impl IntoIterator<Item = Mask>) -> bool {
    let mut seen = 0;
    for cell in cells {
        if seen & cell != 0 {
            return true;
        }
        seen |= cell;
    }
    false
}

/// Converts a grid of digits (0 = empty) to a field
#[cfg(test)]
pub(crate) fn field_from_digits(digits: [[u8; 9]; 9]) -> Field {
    digits.map(|row| row.map(|digit| if digit == 0 { 0 } else { digit_mask(digit) }))
}
