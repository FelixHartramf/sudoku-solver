
#[cfg(test)]
mod impl_sudoku_test {

    use sudoku::sudokus::{Sudoku, FastSudoku};

    
    #[test]
    fn solve() {

        for sudoku_path in Sudoku::UNSOLVED_SUDOKU_FILES {
            let mut sudoku = Sudoku::from_file(sudoku_path);

            sudoku.solve();
            assert!(sudoku.is_part_correct());
            assert!(sudoku.is_correct());
            assert!(sudoku.is_solved());

            let mut sudoku = FastSudoku::from_file(sudoku_path);
            sudoku.solve();
            assert!(sudoku.is_part_correct());
            assert!(sudoku.is_correct());
            assert!(sudoku.is_solved());
        }
    }

    #[test]
    fn file_cross_check() {
        for  path in Sudoku::SUDOKUS_WITH_SOLUTION {
            let mut sudoku_unsolved = Sudoku::from_file(path[0]);
            let sudoku_solved = Sudoku::from_file(path[1]);

            sudoku_unsolved.solve();

            assert_eq!(sudoku_solved, sudoku_unsolved);
        }
    }

    #[test]
    fn implementation_cross_check() {

        // For a classic sudoku both should have the same implementation
        for path in FastSudoku::UNSOLVED_SUDOKU_FILES {
            let mut fast_sudoku = FastSudoku::from_file(path);
            let mut sudoku = Sudoku::from_file(path);

            fast_sudoku.solve();
            sudoku.solve();

            assert_eq!(fast_sudoku.field, sudoku.field);
        }

    }
}
