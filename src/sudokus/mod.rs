pub mod sudoku;
pub mod fast_sudoku;
mod x_sudoku;
mod sudoku_rule;
mod even_odd_sudoku;

pub use sudoku::Sudoku;
pub use fast_sudoku::FastSudoku;
use sudoku_rule::SudokuRule;
