
#[cfg(test)]
mod impl_sudoku_test {

    use sudoku::Sudoku;

    
    #[test]
    fn solve() {

        for sudoku_path in Sudoku::UNSOLVED_SUDOKU_FILES {
            let mut sudoku = Sudoku::from_file(sudoku_path);

            sudoku.solve();
            assert!(sudoku.is_part_correct());
            assert!(sudoku.is_correct());
            assert!(sudoku.is_solved());
        }
    }
}
