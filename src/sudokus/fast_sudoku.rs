use super::{GenericSudoku, NoRules};

/// A classic sudoku without any variants.
///
/// It shares the code with [`Sudoku`](super::Sudoku), but as the rules are
/// known at compile time all rule checks are optimized away.
///
/// ```
/// use sudoku::sudokus::FastSudoku;
///
/// let mut sudoku = FastSudoku::from_file("test-sudokus/text/sudoku-com-12-11-12-master-unsolved").unwrap();
/// assert!(sudoku.solve());
/// assert!(sudoku.is_correct());
/// ```
pub type FastSudoku = GenericSudoku<NoRules>;
