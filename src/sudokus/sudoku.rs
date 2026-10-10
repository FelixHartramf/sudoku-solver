use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use super::parse::parse_grid;
use super::{
    bits, digit_mask, has_duplicates, mask_digit, Field, Mask, Moves, ParseError, RuleSet,
    SudokuRule, VariantRules, ALL_DIGITS,
};

/// A sudoku with any combination of variants, e.g. loaded from a file.
///
/// ```
/// use sudoku::sudokus::Sudoku;
///
/// let mut sudoku = Sudoku::from_file("test-sudokus/text/sudoku-com-24-01-06-x-sudoku-master-unsolved").unwrap();
/// assert!(sudoku.solve());
/// assert!(sudoku.is_correct());
/// ```
pub type Sudoku = GenericSudoku<VariantRules>;

/// The 27 units (9 rows, 9 columns, 9 squares) as lists of `(row, column)`
const UNITS: [[(usize, usize); 9]; 27] = build_units();

const fn build_units() -> [[(usize, usize); 9]; 27] {
    let mut units = [[(0, 0); 9]; 27];
    let mut i = 0;
    while i < 9 {
        let mut j = 0;
        while j < 9 {
            units[i][j] = (i, j);
            units[9 + i][j] = (j, i);
            units[18 + i][j] = (i / 3 * 3 + j / 3, i % 3 * 3 + j % 3);
            j += 1;
        }
        i += 1;
    }
    units
}

#[inline]
const fn square_index(row: usize, column: usize) -> usize {
    row / 3 * 3 + column / 3
}

/// The board found a cell or unit that can't be filled anymore
struct Contradiction;

/// The state of the field.
///
/// Besides the cells it stores the digits used in every row, column and
/// square, so the candidates of a cell are just three ORs away.
/// It is `Copy`, so saving the state before a guess is a cheap memcpy.
#[derive(Debug, Clone, Copy)]
struct Board {
    field: Field,
    rows: [Mask; 9],
    columns: [Mask; 9],
    squares: [Mask; 9],
    unset: u8,
    moves: Moves,
}

impl Board {
    const EMPTY: Board = Board {
        field: [[0; 9]; 9],
        rows: [0; 9],
        columns: [0; 9],
        squares: [0; 9],
        unset: 81,
        moves: Moves {
            missing_moves: 0,
            alone_moves: 0,
            bruteforce_moves: 0,
            player_moves: 0,
            bruteforce_failed_tries: 0,
            failed_missing_moves: 0,
            failed_alone_moves: 0,
        },
    };

    /// Sets an empty cell
    #[inline]
    fn place(&mut self, row: usize, column: usize, mask: Mask) {
        debug_assert!(self.field[row][column] == 0 && mask.count_ones() == 1);
        self.field[row][column] = mask;
        self.rows[row] |= mask;
        self.columns[column] |= mask;
        self.squares[square_index(row, column)] |= mask;
        self.unset -= 1;
    }

    /// Empties a cell
    fn clear(&mut self, row: usize, column: usize) {
        if self.field[row][column] == 0 {
            return;
        }
        self.field[row][column] = 0;
        self.unset += 1;
        // Recalculate, as the digit could be in a unit twice when a player made a mistake
        self.rows[row] = self.field[row].iter().fold(0, |set, cell| set | cell);
        self.columns[column] = (0..9).fold(0, |set, r| set | self.field[r][column]);
        let square = square_index(row, column);
        self.squares[square] = UNITS[18 + square]
            .iter()
            .fold(0, |set, &(r, c)| set | self.field[r][c]);
    }

    /// Returns the digits that can be set in an empty cell
    #[inline]
    fn candidates<R: RuleSet>(&self, rules: &R, row: usize, column: usize) -> Mask {
        let used = self.rows[row]
            | self.columns[column]
            | self.squares[square_index(row, column)]
            | rules.excluded(&self.field, row, column);
        ALL_DIGITS & !used
    }

    /// "Set missing": Sets every cell with only a single candidate.
    ///
    /// Returns true if a cell was set.
    fn set_missing_numbers<R: RuleSet>(&mut self, rules: &R) -> Result<bool, Contradiction> {
        let mut progress = false;
        for row in 0..9 {
            for column in 0..9 {
                if self.field[row][column] != 0 {
                    continue;
                }
                let candidates = self.candidates(rules, row, column);
                match candidates.count_ones() {
                    0 => return Err(Contradiction),
                    1 => {
                        self.place(row, column, candidates);
                        self.moves.missing_moves += 1;
                        progress = true;
                    }
                    _ => {}
                }
            }
        }
        Ok(progress)
    }

    /// "Set alone": Sets every digit that is only possible in a single cell
    /// of a row, column or square.
    ///
    /// Returns true if a cell was set.
    fn set_alone_numbers<R: RuleSet>(&mut self, rules: &R) -> Result<bool, Contradiction> {
        let mut progress = false;
        for unit in &UNITS {
            let mut used = 0;
            let mut once = 0;
            let mut twice = 0;
            let mut candidates = [0; 9];

            for (i, &(row, column)) in unit.iter().enumerate() {
                if self.field[row][column] != 0 {
                    used |= self.field[row][column];
                    continue;
                }
                candidates[i] = self.candidates(rules, row, column);
                twice |= once & candidates[i];
                once |= candidates[i];
            }

            // A digit that is neither set nor possible anywhere in the unit
            if used | once != ALL_DIGITS {
                return Err(Contradiction);
            }

            for digit in bits(once & !twice) {
                let i = candidates.iter().position(|c| c & digit != 0).unwrap_or(9);
                let (row, column) = unit[i];
                // Two digits that can only go into the same cell
                if self.field[row][column] != 0 || self.candidates(rules, row, column) & digit == 0
                {
                    return Err(Contradiction);
                }
                self.place(row, column, digit);
                self.moves.alone_moves += 1;
                progress = true;
            }
        }
        Ok(progress)
    }

    /// Uses the logical strategies until they don't find anything anymore
    fn propagate<R: RuleSet>(&mut self, rules: &R) -> Result<(), Contradiction> {
        loop {
            if self.set_missing_numbers(rules)? {
                continue;
            }
            if !self.set_alone_numbers(rules)? {
                return Ok(());
            }
        }
    }

    /// Returns the empty cell with the fewest candidates.
    /// Guessing there needs the fewest tries.
    fn most_constrained_cell<R: RuleSet>(&self, rules: &R) -> (usize, usize, Mask) {
        let mut best = (9, 9, ALL_DIGITS);
        let mut best_count = u32::MAX;
        for row in 0..9 {
            for column in 0..9 {
                if self.field[row][column] != 0 {
                    continue;
                }
                let candidates = self.candidates(rules, row, column);
                let count = candidates.count_ones();
                if count < best_count {
                    best = (row, column, candidates);
                    best_count = count;
                    if count <= 2 {
                        return best;
                    }
                }
            }
        }
        best
    }

    /// Solves the board with the strategies and guessing (backtracking).
    ///
    /// Returns true if it is solved. If not, the board is in an undefined state.
    fn solve<R: RuleSet>(&mut self, rules: &R) -> bool {
        if self.propagate(rules).is_err() {
            return false;
        }
        if self.unset == 0 {
            return true;
        }

        let (row, column, candidates) = self.most_constrained_cell(rules);
        let before_guess = *self;
        for digit in bits(candidates) {
            self.place(row, column, digit);
            self.moves.bruteforce_moves += 1;

            if self.solve(rules) {
                return true;
            }

            let moves = before_guess.moves.rolled_back(&self.moves);
            *self = before_guess;
            self.moves = moves;
        }
        false
    }

    /// Counts the solutions, but stops at `limit`
    fn count_solutions<R: RuleSet>(&mut self, rules: &R, limit: usize) -> usize {
        if self.propagate(rules).is_err() {
            return 0;
        }
        if self.unset == 0 {
            return 1;
        }

        let (row, column, candidates) = self.most_constrained_cell(rules);
        let before_guess = *self;
        let mut found = 0;
        for digit in bits(candidates) {
            if found >= limit {
                break;
            }
            self.place(row, column, digit);
            found += self.count_solutions(rules, limit - found);
            *self = before_guess;
        }
        found
    }
}

/// A sudoku that is solved with the rules `R`.
///
/// Use the aliases [`Sudoku`] (variants chosen at runtime) or
/// [`FastSudoku`](super::FastSudoku) (only classic rules).
#[derive(Debug, Clone)]
pub struct GenericSudoku<R: RuleSet> {
    board: Board,
    rules: R,
}

impl<R: RuleSet> Default for GenericSudoku<R> {
    fn default() -> Self {
        Self {
            board: Board::EMPTY,
            rules: R::default(),
        }
    }
}

impl<R: RuleSet> PartialEq for GenericSudoku<R> {
    fn eq(&self, other: &Self) -> bool {
        self.board.field == other.board.field
    }
}

impl<R: RuleSet> fmt::Display for GenericSudoku<R> {
    /// Prints the field in the same format as the files, `0` is an empty cell
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..9 {
            for column in 0..9 {
                write!(f, "{} ", self.get_number(row, column).unwrap_or(0))?;
                if column % 3 == 2 && column != 8 {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
            if row % 3 == 2 && row != 8 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl<R: RuleSet> FromStr for GenericSudoku<R> {
    type Err = ParseError;

    /// Reads a sudoku. For the format see the files in `test-sudokus/text/`:
    ///
    /// The file consists of sections that end with `;`. Each section starts
    /// with its name, followed by `:` and its content. The `field` section
    /// contains the 81 cells in reading order, `0` is an empty cell.
    /// All other sections add the rules of a variant.
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut sudoku = Self::default();
        let mut has_field = false;

        for section in input.split(';').map(str::trim).filter(|s| !s.is_empty()) {
            let (name, body) = section.split_once(':').unwrap_or((section, ""));
            let name = name.trim();

            if name != "field" {
                sudoku.rules.add_section(name, body)?;
                continue;
            }

            if has_field {
                return Err(ParseError::DuplicateSection(name.to_string()));
            }
            has_field = true;

            let digits = parse_grid(name, body, |c| match c {
                '.' => Some(0),
                _ => c.to_digit(10).map(|digit| digit as u8),
            })?;
            for (row, digits) in digits.iter().enumerate() {
                for (column, &digit) in digits.iter().enumerate() {
                    if digit != 0 {
                        sudoku.board.place(row, column, digit_mask(digit));
                    }
                }
            }
        }

        if !has_field {
            return Err(ParseError::MissingField);
        }
        Ok(sudoku)
    }
}

impl Sudoku {
    /// Adds a variant rule
    /// ```
    /// use sudoku::sudokus::{Sudoku, XSudoku};
    ///
    /// let mut sudoku = Sudoku::empty();
    /// sudoku.add_rule(XSudoku);
    /// assert!(sudoku.solve());
    /// ```
    pub fn add_rule(&mut self, rule: impl SudokuRule + 'static) {
        self.rules.push(rule);
    }
}

impl<R: RuleSet> GenericSudoku<R> {
    /// Returns an empty Sudoku
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let sudoku = Sudoku::empty();
    /// assert_eq!(sudoku.count_unset(), 81);
    /// ```
    pub fn empty() -> Self {
        Self::default()
    }

    /// Returns a solved Sudoku
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let sudoku = Sudoku::example_solved();
    /// assert!(sudoku.is_correct());
    /// ```
    pub fn example_solved() -> Self {
        "field:
            6 3 5  4 9 2  1 8 7
            1 7 4  3 5 8  2 9 6
            2 8 9  1 7 6  5 4 3

            5 1 8  9 3 7  4 6 2
            7 2 6  8 4 5  9 3 1
            4 9 3  6 2 1  8 7 5

            3 5 1  7 8 9  6 2 4
            9 6 7  2 1 4  3 5 8
            8 4 2  5 6 3  7 1 9;"
            .parse()
            .expect("the example is valid")
    }

    /// Reads a Sudoku from a file, see the `FromStr` implementation for the format
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let sudoku = Sudoku::from_file("test-sudokus/text/sudoku-com-12-11-12-master-solved").unwrap();
    /// assert!(sudoku.is_solved());
    ///
    /// assert!(Sudoku::from_file("does-not-exist").is_err());
    /// ```
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ParseError> {
        fs::read_to_string(path)?.parse()
    }

    /// Returns the rules of the sudoku
    pub fn rules(&self) -> &R {
        &self.rules
    }

    /// Returns the cells, indexed as `field[row][column]`
    pub fn field(&self) -> &Field {
        &self.board.field
    }

    /// Returns the statistics of the moves done so far
    pub fn moves(&self) -> &Moves {
        &self.board.moves
    }

    /// Returns the number of moves that set a cell
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let mut sudoku = Sudoku::from_file("test-sudokus/text/sudoku-com-12-11-12-master-unsolved").unwrap();
    /// let unset = sudoku.count_unset();
    /// sudoku.solve();
    /// assert_eq!(sudoku.count_moves(), unset);
    /// ```
    pub fn count_moves(&self) -> u32 {
        self.board.moves.total()
    }

    /// Returns the number of a cell or `None` if it is empty
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let sudoku = Sudoku::example_solved();
    /// assert_eq!(sudoku.get_number(0, 0), Some(6));
    /// assert_eq!(Sudoku::empty().get_number(0, 0), None);
    /// ```
    pub fn get_number(&self, row: usize, column: usize) -> Option<u8> {
        match self.board.field[row][column] {
            0 => None,
            mask => Some(mask_digit(mask)),
        }
    }

    /// Returns true if the cell is set
    pub fn is_set(&self, row: usize, column: usize) -> bool {
        self.board.field[row][column] != 0
    }

    /// Returns the digits that can still be set in an empty cell.
    /// For a set cell the list is empty.
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let mut sudoku = Sudoku::example_solved();
    /// sudoku.clear(0, 0);
    /// assert_eq!(sudoku.candidates(0, 0), vec![6]);
    /// ```
    pub fn candidates(&self, row: usize, column: usize) -> Vec<u8> {
        if self.is_set(row, column) {
            return Vec::new();
        }
        bits(self.board.candidates(&self.rules, row, column))
            .map(mask_digit)
            .collect()
    }

    /// Sets the number of a cell as a player move.
    ///
    /// The number may break the rules, check with [`is_part_correct`](Self::is_part_correct).
    ///
    /// # Panics
    /// If `value` is not between 1 and 9.
    ///
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let mut sudoku = Sudoku::example_solved();
    /// sudoku.set_number(0, 0, 9);
    /// assert_eq!(sudoku.get_number(0, 0), Some(9));
    /// assert!(!sudoku.is_part_correct());
    /// ```
    pub fn set_number(&mut self, row: usize, column: usize, value: u8) {
        assert!((1..=9).contains(&value), "{value} is not a sudoku digit");
        self.board.clear(row, column);
        self.board.place(row, column, digit_mask(value));
        self.board.moves.player_moves += 1;
    }

    /// Empties a cell
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let mut sudoku = Sudoku::example_solved();
    /// sudoku.clear(0, 0);
    /// assert_eq!(sudoku.get_number(0, 0), None);
    /// ```
    pub fn clear(&mut self, row: usize, column: usize) {
        self.board.clear(row, column);
    }

    /// Counts the cells that are not set
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// // A solved sudoku doesn't have any unset cells
    /// assert_eq!(Sudoku::example_solved().count_unset(), 0);
    /// ```
    pub fn count_unset(&self) -> u32 {
        self.board.unset.into()
    }

    /// Returns true if all cells are set. They don't have to be correct.
    pub fn is_solved(&self) -> bool {
        self.board.unset == 0
    }

    /// Returns true if no rule is broken. Empty cells are ignored.
    pub fn is_part_correct(&self) -> bool {
        let field = &self.board.field;
        UNITS
            .iter()
            .all(|unit| !has_duplicates(unit.iter().map(|&(row, column)| field[row][column])))
            && self.rules.complies(field)
    }

    /// Returns true if the sudoku is solved correctly
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let sudoku = Sudoku::example_solved();
    /// assert!(sudoku.is_correct());
    /// assert_eq!(sudoku.is_correct(), sudoku.is_solved() && sudoku.is_part_correct());
    /// ```
    pub fn is_correct(&self) -> bool {
        self.is_solved() && self.is_part_correct()
    }

    /// Solves the sudoku and returns true if it has a solution.
    ///
    /// If there are multiple solutions, one of them is chosen.
    /// If there is none, the sudoku stays unchanged.
    pub fn solve(&mut self) -> bool {
        if !self.is_part_correct() {
            return false;
        }

        let mut board = self.board;
        if !board.solve(&self.rules) {
            return false;
        }
        self.board = board;
        true
    }

    /// Counts the solutions, but stops searching after `limit` solutions
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// assert_eq!(Sudoku::empty().count_solutions(5), 5);
    /// assert_eq!(Sudoku::example_solved().count_solutions(5), 1);
    /// ```
    pub fn count_solutions(&self, limit: usize) -> usize {
        if limit == 0 || !self.is_part_correct() {
            return 0;
        }
        self.board.clone().count_solutions(&self.rules, limit)
    }

    /// Returns true if the sudoku has exactly one solution, as a proper sudoku should
    pub fn has_unique_solution(&self) -> bool {
        self.count_solutions(2) == 1
    }
}

#[cfg(test)]
mod sudoku_test {
    use super::UNITS;
    use crate::sample_files;
    use crate::sudokus::{FastSudoku, ParseError, Sudoku};

    #[test]
    fn units() {
        for unit in UNITS {
            let mut seen = [[false; 9]; 9];
            for (row, column) in unit {
                assert!(!seen[row][column]);
                seen[row][column] = true;
            }
        }
        assert_eq!(UNITS[18 + 4][4], (4, 4));
        assert_eq!(UNITS[9 + 2][5], (5, 2));
    }

    #[test]
    fn set_and_clear() {
        let mut sudoku = Sudoku::empty();

        for row in 0..9 {
            for column in 0..9 {
                for value in 1..=9 {
                    sudoku.set_number(row, column, value);
                    assert_eq!(sudoku.get_number(row, column), Some(value));
                    assert_eq!(sudoku.count_unset(), 80);
                }
                sudoku.clear(row, column);
                assert_eq!(sudoku.get_number(row, column), None);
                assert_eq!(sudoku.count_unset(), 81);
                assert_eq!(sudoku.candidates(row, column), (1..=9).collect::<Vec<_>>());
            }
        }
        assert_eq!(sudoku.moves().player_moves, 81 * 9);
    }

    #[test]
    fn clear_after_mistake() {
        let mut sudoku = Sudoku::empty();
        sudoku.set_number(0, 0, 5);
        sudoku.set_number(0, 1, 5);
        assert!(!sudoku.is_part_correct());

        // The other 5 is still in the row
        sudoku.clear(0, 1);
        assert!(sudoku.is_part_correct());
        assert!(!sudoku.candidates(0, 1).contains(&5));
    }

    #[test]
    fn read_from_file() {
        let sudoku =
            Sudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-solved").unwrap();
        assert_eq!(sudoku, Sudoku::example_solved());

        let unsolved =
            Sudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-unsolved").unwrap();
        assert!(!unsolved.is_solved());
        assert!(unsolved.is_part_correct());
    }

    #[test]
    fn parse_errors() {
        assert!(matches!(
            "".parse::<Sudoku>(),
            Err(ParseError::MissingField)
        ));
        assert!(matches!(
            "field: 1 2 3;".parse::<Sudoku>(),
            Err(ParseError::WrongCellCount { found: 3, .. })
        ));
        assert!(matches!(
            format!("field: {};", "0".repeat(81))
                .repeat(2)
                .parse::<Sudoku>(),
            Err(ParseError::DuplicateSection(_))
        ));
        assert!(matches!(
            format!("field: {}; killer-sudoku;", "0".repeat(81)).parse::<Sudoku>(),
            Err(ParseError::UnknownSection(_))
        ));
        assert!(matches!(
            FastSudoku::from_file("test-sudokus/text/sudoku-com-24-01-06-x-sudoku-easy-unsolved"),
            Err(ParseError::UnsupportedRule(_))
        ));
        assert!(matches!(
            Sudoku::from_file("test-sudokus/text/does-not-exist"),
            Err(ParseError::Io(_))
        ));
    }

    #[test]
    fn display_round_trip() {
        let sudoku = Sudoku::from_file("test-sudokus/text/welt-23-11-11-hard-unsolved").unwrap();
        let text = format!("field:\n{sudoku};");
        assert_eq!(text.parse::<Sudoku>().unwrap(), sudoku);
    }

    #[test]
    fn correct() {
        let sudoku =
            Sudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-solved").unwrap();
        assert!(sudoku.is_solved());
        assert!(sudoku.is_part_correct());
        assert!(sudoku.is_correct());

        let sudoku =
            Sudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-wrong").unwrap();
        assert!(sudoku.is_solved());
        assert!(!sudoku.is_part_correct());
        assert!(!sudoku.is_correct());
    }

    #[test]
    fn missing() {
        let solved =
            Sudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-solved").unwrap();

        // These can be solved with "Set missing" alone
        for path in [
            "test-sudokus/text/tagesspiegel-23-11-11-easy-few-missing",
            "test-sudokus/text/tagesspiegel-23-11-11-easy-some-missing",
            "test-sudokus/text/tagesspiegel-23-11-11-easy-unsolved",
        ] {
            let mut sudoku = Sudoku::from_file(path).unwrap();
            while sudoku.board.set_missing_numbers(&sudoku.rules).ok() == Some(true) {
                assert!(sudoku.is_part_correct());
            }
            assert_eq!(sudoku, solved);
            assert_eq!(sudoku.moves().alone_moves, 0);
        }
    }

    #[test]
    fn alone() {
        // Medium can be solved with "Set missing" and "Set alone"
        let mut sudoku =
            Sudoku::from_file("test-sudokus/text/welt-23-11-11-medium-unsolved").unwrap();
        assert!(sudoku.board.propagate(&sudoku.rules).is_ok());
        assert!(sudoku.is_correct());

        // Hard can't, but every step has to be correct
        let mut sudoku =
            Sudoku::from_file("test-sudokus/text/welt-23-11-11-hard-unsolved").unwrap();
        loop {
            let missing = sudoku.board.set_missing_numbers(&sudoku.rules);
            assert!(sudoku.is_part_correct());
            if missing.ok() == Some(true) {
                continue;
            }
            let alone = sudoku.board.set_alone_numbers(&sudoku.rules);
            assert!(sudoku.is_part_correct());
            if alone.ok() != Some(true) {
                break;
            }
        }
        assert!(!sudoku.is_solved());
    }

    #[test]
    fn count_moves() {
        for path in sample_files::unsolved() {
            let mut sudoku = Sudoku::from_file(path).unwrap();
            let unset = sudoku.count_unset();

            assert!(sudoku.solve());
            assert_eq!(unset, sudoku.count_moves(), "{path}");
        }

        // Player moves count too
        let mut sudoku =
            Sudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-unsolved").unwrap();
        let unset = sudoku.count_unset();
        sudoku.set_number(0, 2, 5);
        assert!(sudoku.solve());
        assert_eq!(sudoku.count_moves(), unset);
        assert_eq!(sudoku.moves().player_moves, 1);
    }

    #[test]
    fn no_solution() {
        // Two 5 in the first row
        let mut sudoku = Sudoku::empty();
        sudoku.set_number(0, 0, 5);
        sudoku.set_number(0, 8, 5);
        let before = sudoku.clone();
        assert!(!sudoku.solve());
        assert_eq!(sudoku, before);
        assert_eq!(sudoku.count_solutions(2), 0);

        // Correct so far, but the first cell has no candidate left
        let mut sudoku = Sudoku::empty();
        for (column, value) in (1..9).zip(1..9) {
            sudoku.set_number(0, column, value);
        }
        sudoku.set_number(1, 0, 9);
        assert!(sudoku.is_part_correct());
        assert!(!sudoku.solve());
    }

    #[test]
    fn unique() {
        for path in sample_files::unsolved() {
            assert!(
                Sudoku::from_file(path).unwrap().has_unique_solution(),
                "{path}"
            );
        }

        assert!(!Sudoku::empty().has_unique_solution());
        assert_eq!(Sudoku::empty().count_solutions(0), 0);
    }
}
