use sudoku::sample_files;
use sudoku::sudokus::{FastSudoku, Sudoku};

#[test]
fn solve() {
    for path in sample_files::unsolved() {
        let mut sudoku = Sudoku::from_file(path).unwrap();

        assert!(sudoku.solve(), "{path}");
        assert!(sudoku.is_part_correct(), "{path}");
        assert!(sudoku.is_solved(), "{path}");
        assert!(sudoku.is_correct(), "{path}");
    }

    for path in sample_files::CLASSIC_UNSOLVED {
        let mut sudoku = FastSudoku::from_file(path).unwrap();

        assert!(sudoku.solve(), "{path}");
        assert!(sudoku.is_correct(), "{path}");
    }
}

#[test]
fn file_cross_check() {
    for (unsolved, solved) in sample_files::WITH_SOLUTION {
        let mut sudoku = Sudoku::from_file(unsolved).unwrap();
        let solution = Sudoku::from_file(solved).unwrap();

        assert!(sudoku.solve(), "{unsolved}");
        assert_eq!(sudoku, solution, "{unsolved}");
    }
}

#[test]
fn implementation_cross_check() {
    // For a classic sudoku both have to find the same solution
    for path in sample_files::CLASSIC_UNSOLVED {
        let mut fast_sudoku = FastSudoku::from_file(path).unwrap();
        let mut sudoku = Sudoku::from_file(path).unwrap();

        fast_sudoku.solve();
        sudoku.solve();

        assert_eq!(fast_sudoku.field(), sudoku.field(), "{path}");
        assert_eq!(fast_sudoku.moves(), sudoku.moves(), "{path}");
    }
}

#[test]
fn variant_needs_its_rule() {
    // Without the diagonals this sudoku has 45 solutions, with them only one
    let path = "test-sudokus/text/sudoku-com-24-01-06-x-sudoku-master-unsolved";
    let text = std::fs::read_to_string(path).unwrap();
    let field = text.split(';').next().unwrap();
    let classic: Sudoku = field.parse().unwrap();

    assert!(classic.rules().is_empty());
    assert_eq!(classic.count_solutions(100), 45);
    assert_eq!(Sudoku::from_file(path).unwrap().count_solutions(100), 1);
}
