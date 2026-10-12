//! Paths of the sudokus in `test-sudokus/text/`, relative to the crate root.
//!
//! The file names have the format `{website}-{date of retrieval}-{level}-{solved/unsolved}`.

/// Classic sudokus without a solution file
pub const CLASSIC_UNSOLVED: [&str; 9] = [
    "test-sudokus/text/tagesspiegel-23-11-11-easy-unsolved",
    "test-sudokus/text/welt-23-11-11-easy-unsolved",
    "test-sudokus/text/welt-23-11-11-medium-unsolved",
    "test-sudokus/text/welt-23-11-11-hard-unsolved",
    "test-sudokus/text/sudoku-com-12-11-12-master-unsolved",
    "test-sudokus/text/sudoku-com-12-11-12-2-master-unsolved",
    "test-sudokus/text/wikipedia-17-given-unsolved",
    "test-sudokus/text/inkala-26-10-12-hardest-unsolved",
    "test-sudokus/text/inkala-26-10-12-ai-escargot-unsolved",
];

/// Sudokus with variant rules
pub const VARIANT_UNSOLVED: [&str; 4] = [
    "test-sudokus/text/sudoku-com-24-01-06-x-sudoku-easy-unsolved",
    "test-sudokus/text/sudoku-com-24-01-06-x-sudoku-master-unsolved",
    "test-sudokus/text/sudokuzok-de-24-01-07-evenodd-easy-unsolved",
    "test-sudokus/text/sudokuzok-de-24-01-07-evenodd-extra-hard-unsolved",
];

/// Pairs of `(unsolved, solved)`
pub const WITH_SOLUTION: [(&str, &str); 10] = [
    // Classic
    (
        "test-sudokus/text/sudoku-com-12-11-12-master-unsolved",
        "test-sudokus/text/sudoku-com-12-11-12-master-solved",
    ),
    (
        "test-sudokus/text/welt-23-11-11-medium-unsolved",
        "test-sudokus/text/welt-23-11-11-medium-solved",
    ),
    (
        "test-sudokus/text/welt-23-11-11-easy-unsolved",
        "test-sudokus/text/welt-23-11-11-easy-solved",
    ),
    (
        "test-sudokus/text/tagesspiegel-23-11-11-easy-unsolved",
        "test-sudokus/text/tagesspiegel-23-11-11-easy-solved",
    ),
    (
        "test-sudokus/text/inkala-26-10-12-hardest-unsolved",
        "test-sudokus/text/inkala-26-10-12-hardest-solved",
    ),
    (
        "test-sudokus/text/inkala-26-10-12-ai-escargot-unsolved",
        "test-sudokus/text/inkala-26-10-12-ai-escargot-solved",
    ),
    // X-Sudoku
    (
        "test-sudokus/text/sudoku-com-24-01-06-x-sudoku-easy-unsolved",
        "test-sudokus/text/sudoku-com-24-01-06-x-sudoku-easy-solved",
    ),
    (
        "test-sudokus/text/sudoku-com-24-01-06-x-sudoku-master-unsolved",
        "test-sudokus/text/sudoku-com-24-01-06-x-sudoku-master-solved",
    ),
    // Even-Odd-Sudoku
    (
        "test-sudokus/text/sudokuzok-de-24-01-07-evenodd-easy-unsolved",
        "test-sudokus/text/sudokuzok-de-24-01-07-evenodd-easy-solved",
    ),
    (
        "test-sudokus/text/sudokuzok-de-24-01-07-evenodd-extra-hard-unsolved",
        "test-sudokus/text/sudokuzok-de-24-01-07-evenodd-extra-hard-solved",
    ),
];

/// All unsolved sudokus, classic and variants
pub fn unsolved() -> impl Iterator<Item = &'static str> {
    CLASSIC_UNSOLVED.into_iter().chain(VARIANT_UNSOLVED)
}
