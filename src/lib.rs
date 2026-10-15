//! A fast sudoku solver for classic sudokus and some variants.
//!
//! ```
//! use sudoku::sudokus::Sudoku;
//!
//! let mut sudoku = Sudoku::from_file("test-sudokus/text/welt-23-11-11-hard-unsolved").unwrap();
//! assert!(sudoku.solve());
//! println!("{sudoku}");
//! ```

pub mod sample_files;
pub mod sudokus;
