use std::fmt;
use std::fs;
use std::fmt::Debug;
use crate::sudokus::SudokuRule;

use super::x_sudoku::XSudoku;

#[derive(Debug, Clone)]
pub struct Moves {
    missing_moves: u32,
    alone_moves: u32,
    player_moves: u32,
    bruteforce_moves: u32,
    bruteforce_failed_tries: u32,
    failed_missing_moves: u32,
    failed_alone_moves: u32,
}

impl Moves {
    pub fn empty() -> Self {
        Self {
            missing_moves: 0,
            alone_moves: 0,
            bruteforce_moves: 0,
            bruteforce_failed_tries: 0,
            player_moves: 0,
            failed_alone_moves: 0,
            failed_missing_moves: 0,
        }
    }
}

/// Sudoku. It contains the data of the field and counts the moves
pub struct Sudoku {
    pub field: [[i32; 9]; 9],
    moves: Moves,

    rules: Vec<Box<dyn SudokuRule>>,
}

impl PartialEq for Sudoku {
    fn eq(&self, other: &Self) -> bool {
        self.field == other.field
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for collum in 0..9 {
            for row in 0..9 {
                write!(f, "{} ", self.get_number(row, collum))?;
                if row % 3 == 2 {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
            if collum % 3 == 2 {
                write!(f, "\n")?;
            }
        }

        write!(f, "Moves: {:#?} ", self.moves)?;
        Ok(())
    }
}

impl fmt::Debug for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for collum in 0..9 {
            for row in 0..9 {
                write!(f, "{} ", self.get_number(row, collum))?;
                if row % 3 == 2 {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
            if collum % 3 == 2 {
                write!(f, "\n")?;
            }
        }

        write!(f, "Moves: {:#?} ", self.moves)?;
        Ok(())
    }
}

impl Sudoku {
    /// Relative Path of solveable Sudokus.
    /// The following format is used:
    /// {website}-{date of retival}-{level}-{solved/unsolved}
    ///
    /// For data structure see Sudoku::from_file
    pub const UNSOLVED_SUDOKU_FILES: [&'static str; 9] = [
        "test-sudokus/text/tagesspiegel-23-11-11-easy-unsolved",
        "test-sudokus/text/welt-23-11-11-medium-unsolved",
        "test-sudokus/text/welt-23-11-11-hard-unsolved",
        "test-sudokus/text/welt-23-11-11-easy-unsolved",
        "test-sudokus/text/tagesspiegel-23-11-11-easy-unsolved",
        "test-sudokus/text/sudoku-com-12-11-12-master-unsolved",
        "test-sudokus/text/sudoku-com-12-11-12-2-master-unsolved",
        "test-sudokus/text/wikipedia-17-given-unsolved",
        "test-sudokus/text/sudoku-com-24-01-06-x-sudoku-easy-unsolved",
    ];

    pub const SUDOKUS_WITH_SOLUTION: [[&'static str; 2]; 6] = [
        // Normal
        ["test-sudokus/text/sudoku-com-12-11-12-master-unsolved", "test-sudokus/text/sudoku-com-12-11-12-master-solved"],
        ["test-sudokus/text/welt-23-11-11-medium-unsolved", "test-sudokus/text/welt-23-11-11-medium-solved"],
        ["test-sudokus/text/welt-23-11-11-easy-unsolved", "test-sudokus/text/welt-23-11-11-easy-solved"],
        ["test-sudokus/text/tagesspiegel-23-11-11-easy-unsolved", "test-sudokus/text/tagesspiegel-23-11-11-easy-solved"],

        // X-Sudoku
        ["test-sudokus/text/sudoku-com-24-01-06-x-sudoku-easy-unsolved", "test-sudokus/text/sudoku-com-24-01-06-x-sudoku-easy-solved"],
        ["test-sudokus/text/sudoku-com-24-01-06-x-sudoku-master-unsolved", "test-sudokus/text/sudoku-com-24-01-06-x-sudoku-master-solved"],
    ];

    /// Returns an empty Sudoku
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let sudoku = Sudoku::empty();
    /// ```
    pub fn empty() -> Self {
        Self {
            field: [[0; 9]; 9],
            moves: Moves::empty(),
            rules: vec![],
        }
    }

    /// Returns an solved Sudoku
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let sudoku = Sudoku::example_solved();
    /// ```
    pub fn example_solved() -> Self {
        let mut sudoku = Sudoku::empty();
        let numbers: [i32; 81] = [
            6, 3, 5, 4, 9, 2, 1, 8, 7, 1, 7, 4, 3, 5, 8, 2, 9, 6, 2, 8, 9, 1, 7, 6, 5, 4, 3, 5, 1,
            8, 9, 3, 7, 4, 6, 2, 7, 2, 6, 8, 4, 5, 9, 3, 1, 4, 9, 3, 6, 2, 1, 8, 7, 5, 3, 5, 1, 7,
            8, 9, 6, 2, 4, 9, 6, 7, 2, 1, 4, 3, 5, 8, 8, 4, 2, 5, 6, 3, 7, 1, 9,
        ];
        let mut count = 0;
        for num in numbers {
            sudoku.add_number(count % 9, count / 9, num);

            count += 1;
        }

        return sudoku;
    }

    /// Returns an Sudoku from File
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let sudoku = Sudoku::from_file("test-sudokus/text/sudoku-com-12-11-12-master-solved");
    /// ```
    pub fn from_file(file_path: &str) -> Self {
        let mut sudoku = Sudoku::empty();

        for data in fs::read_to_string(file_path).expect("todo").split(";") {
            
            if data.len() == 0 {
                continue;
            }
            if data.contains("field") {
                let mut count = 0;
                for c in data.chars() {
                    if count == 81 {
                        break;
                    }
                    if c >= '0' && c <= '9' {
                        sudoku.add_number(
                            count % 9,
                            count / 9,
                            c.to_digit(10)
                                .expect("Internal Conversion Error") // This should not happen as we first check c to be confertable
                                .try_into()
                                .unwrap(),
                        );
                        count += 1;
                    }
                }
            }

            if data.contains(XSudoku::str_identifier()){
                sudoku.rules.push(XSudoku::from_str(data));
            }
        }
        return sudoku;
    }

    /// Returns the number of already taken moves
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let mut sudoku = Sudoku::from_file("test-sudokus/text/sudoku-com-12-11-12-master-unsolved");
    /// sudoku.solve();
    /// assert!(sudoku.count_moves() > 0);
    /// ```
    pub fn count_moves(&self) -> u32 {
        return self.moves.alone_moves + self.moves.bruteforce_moves + self.moves.missing_moves;
    }

    /// Returns true, if the field is set. A single Number is writen.
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let sudoku = Sudoku::example_solved();
    /// println!("Left upper Corner is set: {}", sudoku.is_set(0, 0));
    /// ```
    #[inline]
    pub fn is_set(&self, row: usize, collum: usize) -> bool {
        debug_assert!(row < 9 && collum < 9);
        return i32::count_ones(self.field[row][collum]) == 1;
    }

    /// Counts the number of cells in the sudoku that are not set
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let sudoku = Sudoku::example_solved();
    ///
    /// // A solved sudoku doesn't have any unset cells
    /// assert_eq!(sudoku.count_unset(), 0);
    /// ```
    pub fn count_unset(&self) -> u32 {
        let mut count = 0;
        for row in 0..9 {
            for collum in 0..9 {
                if self.is_set(row, collum) {
                    continue;
                }

                count += 1;
            }
        }
        return count;
    }

    /// Clears a cell
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let mut sudoku = Sudoku::example_solved();
    ///
    /// // Clear the first cell
    /// sudoku.clear(0,0);
    ///
    /// assert_eq!(sudoku.get_number(0,0), 0);
    /// ```
    pub fn clear(&mut self, row: usize, collum: usize) {
        debug_assert!(row < 9 && collum < 9);
        self.field[row][collum] = 0;
    }

    /// Adds the number of cell
    ///
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let mut sudoku = Sudoku::example_solved();
    ///
    /// // Clear the first cell
    /// sudoku.clear(0,0);
    ///
    /// // Set it to 9
    /// sudoku.add_number(0, 0, 9);
    ///
    /// assert_eq!(sudoku.get_number(0,0), 9);
    /// ```
    pub fn add_number(&mut self, row: usize, collum: usize, value: i32) {
        if value == 0 {
            return;
        }
        debug_assert!(value >= 1 && value <= 9);
        debug_assert!(row < 9 && collum < 9);
        self.field[row][collum] |= 0b1 << (value - 1);
    }

    /// Sets the number of a cell
    ///
    /// It behaves just like a clear follwed by
    /// a single add
    ///
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let mut sudoku = Sudoku::example_solved();
    ///
    /// // Set it to 9
    /// sudoku.set_number(0, 0, 9);
    ///
    /// assert_eq!(sudoku.get_number(0,0), 9);
    /// ```
    pub fn set_number(&mut self, row: usize, collum: usize, value: i32) {
        if value == 0 {
            return;
        }
        debug_assert!(value >= 1 && value <= 9);
        debug_assert!(row < 9 && collum < 9);

        self.moves.player_moves += 1;
        self.field[row][collum] = 0b1 << (value - 1);
    }

    /// Brutforces the next number
    /// Due to the implementation it solves the sudoku in the progress and returns true
    /// if it is sovable. If it isn't it returns false.
    fn brute_force(&mut self) -> bool {
        let mut row_to_check = 9;
        let mut collum_to_check = 9;
        'findfield: for row in 0..9 {
            for collum in 0..9 {
                if !self.is_set(row, collum) {
                    row_to_check = row;
                    collum_to_check = collum;

                    break 'findfield;
                }
            }
        }
        if row_to_check == 9 || collum_to_check == 9 {
            return false;
        }

        let field = self.field.clone();
        let missing_moves = self.moves.missing_moves;
        let alone_moves = self.moves.alone_moves;

        let numbers_to_check = !self.get_not_possible_numbers_raw(row_to_check, collum_to_check);

        for num in 0..9 {
            if (0b1 << num) & numbers_to_check == 0 {
                continue;
            }
            self.field[row_to_check][collum_to_check] = 0b1 << num;

            while !self.is_correct() && self.is_solveable() && self.single_solve() {}

            if self.is_correct() {
                self.moves.bruteforce_moves += 1;
                return true;
            }

            self.moves.failed_alone_moves += self.moves.alone_moves - alone_moves;
            self.moves.failed_missing_moves += self.moves.missing_moves - missing_moves;
            self.field = field;
            self.moves.missing_moves = missing_moves;
            self.moves.alone_moves = alone_moves;
            self.moves.bruteforce_failed_tries += 1;
        }

        return false;
    }

    /// Performs a single solving step
    /// If no tactic sets a cell it calls
    /// the bruteforce methode and theirfore solves
    /// it recursivly
    /// It returns true, if it is solvable, and false otherwise
    fn single_solve(&mut self) -> bool {
        if !self.is_solveable() {
            return false;
        }
        if self.is_correct() {
            return true;
        }

        if self.set_missing_numbers() {
            return true;
        }

        if self.set_alone_number() {
            return true;
        }

        return self.brute_force();
    }

    /// Solves the sudoku and returns true if it is solvable
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let mut sudoku = Sudoku::from_file("test-sudokus/text/sudoku-com-12-11-12-master-unsolved");
    /// sudoku.solve();
    /// assert!(sudoku.is_correct());
    /// ```
    pub fn solve(&mut self) -> bool {
        while !self.is_correct() {
            if !self.single_solve() {
                return false;
            }
        }

        return true;
    }

    /// Removes a number from a field
    pub fn remove_number(&mut self, row: usize, collum: usize, value: i32) {
        debug_assert!(value >= 1 && value <= 9);
        debug_assert!(row < 9 && collum < 9);
        self.field[row][collum] &= !(0b1 << (value - 1));
    }

    /// Returns the number of a cell
    /// If no or multible numbers are set it returns 0
    pub fn get_number(&self, row: usize, collum: usize) -> i32 {
        debug_assert!(row < 9 && collum < 9);
        let numbers = self.get_numbers(row, collum);

        if numbers.len() != 1 {
            return 0;
        }

        return numbers[0];
    }

    /// Gets all numbers that are set in a cell as a Vec
    pub fn get_numbers(&self, row: usize, collum: usize) -> Vec<i32> {
        debug_assert!(row < 9 && collum < 9);
        let number = self.field[row][collum];
        let mut vec = Vec::new();

        for i in 0..9 {
            if number & (0b1 << i) == 0b1 << i {
                vec.push(i + 1);
            }
        }

        return vec;
    }

    /// Returns true if all numbers are set
    /// It doesnt have to be solved correctly
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let sudoku = Sudoku::example_solved();
    /// assert!(sudoku.is_solved());
    /// ```
    #[inline]
    pub fn is_solved(&self) -> bool {
        for row in 0..9 {
            for collum in 0..9 {
                if i32::count_ones(self.field[row][collum]) != 1 {
                    return false;
                }
            }
        }
        return true;
    }

    /// Returns true if a sudoku is correctly solved
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let sudoku = Sudoku::example_solved();
    /// assert!(sudoku.is_correct());
    ///
    /// //The following is always true
    /// assert!(sudoku.is_solved() && sudoku.is_part_correct() == sudoku.is_correct());
    /// ```
    #[inline]
    pub fn is_correct(&self) -> bool {
        //return self.is_solved() && self.is_part_correct();

        for i in 0..9 {
            let mut row_nums = 0;
            let mut collum_nums = 0;
            let mut square_nums = 0;
            for j in 0..9 {
                if row_nums & self.field[i][j] != 0
                    || collum_nums & self.field[j][i] != 0
                    || square_nums & self.field[i / 3 * 3 + j / 3][i % 3 * 3 + j % 3] != 0
                    || self.field[i][j] == 0
                {
                    return false;
                }

                row_nums |= self.field[i][j];
                collum_nums |= self.field[j][i];
                square_nums |= self.field[i / 3 * 3 + j / 3][i % 3 * 3 + j % 3];
            }
        }

        for rule in &self.rules {
            if !rule.complys(&self.field) {
                return false;
            }
        }

        return true;
    }

    /// Returns true if their are numbers that can be set.
    ///
    /// It doesn't check if the sudoku has a solution.
    #[inline]
    fn is_solveable(&self) -> bool {
        for row in 0..9 {
            for collum in 0..9 {
                if !self.is_set(row, collum)
                    && self.get_not_possible_numbers_raw(row, collum) & 0b111_111_111
                        == 0b111_111_111
                {
                    return false;
                }
            }
        }
        return true;
    }

    /// Returns true, if there are no errors
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let sudoku = Sudoku::example_solved();
    /// assert!(sudoku.is_part_correct());
    /// ```
    #[inline]
    pub fn is_part_correct(&self) -> bool {
        // check rows and collums
        for i in 0..9 {
            if !self.is_collum_part_correct(i)
                || !self.is_row_part_correct(i)
                || !self.is_square_part_correct(i / 3, i % 3)
            {
                return false;
            }
        }

        for rule in &self.rules {
            if !rule.complys(&self.field) {
                return false;
            }
        }

        return true;
    }

    /// Returns true if a 3x3 square is correct
    fn is_square_part_correct(&self, big_row: usize, big_collum: usize) -> bool {
        debug_assert!(big_row < 3 && big_collum < 3);
        let mut set_numbers = 0;
        for row in 0..3 {
            for collum in 0..3 {
                if !self.is_set(big_row * 3 + row, big_collum * 3 + collum) {
                    continue;
                }
                if set_numbers & self.field[big_row * 3 + row][big_collum * 3 + collum] != 0 {
                    return false;
                }

                set_numbers |= self.field[big_row * 3 + row][big_collum * 3 + collum];
            }
        }
        return true;
    }

    /// Returns true if row is correct
    fn is_row_part_correct(&self, row: usize) -> bool {
        debug_assert!(row < 9);
        let mut set_numbers = 0;
        for collum in 0..9 {
            if !self.is_set(row, collum) {
                continue;
            }
            if set_numbers & self.field[row][collum] != 0 {
                return false;
            }

            set_numbers |= self.field[row][collum];
        }

        return true;
    }

    /// Returns true if collum is correct
    fn is_collum_part_correct(&self, collum: usize) -> bool {
        debug_assert!(collum < 9);
        let mut set_numbers = 0;
        for row in 0..9 {
            if !self.is_set(row, collum) {
                continue;
            }
            if set_numbers & self.field[row][collum] != 0 {
                return false;
            }

            set_numbers |= self.field[row][collum];
        }

        return true;
    }

    /// Finds and sets a missing number in the sudoku field
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let mut sudoku = Sudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-few-missing");
    /// sudoku.set_missing_numbers();
    /// ```
    pub fn set_missing_numbers(&mut self) -> bool {
        let mut number_set = false;
        for row in 0..9 {
            for collum in 0..9 {
                if self.is_set(row, collum) {
                    continue;
                }

                if self.set_missing_number(row, collum) {
                    number_set = true;
                    self.moves.missing_moves += 1;
                }
            }
        }

        return number_set;
    }

    /// Clears and adds the missing numbers in a cell
    /// If a single number is set, it returns true
    #[inline]
    fn set_missing_number(&mut self, row: usize, collum: usize) -> bool {
        debug_assert!(row < 9 && collum < 9);

        let possible = 0b111_111_111 & !self.get_not_possible_numbers_raw(row, collum);

        self.field[row][collum] = possible;

        return i32::count_ones(possible) == 1;
    }

    /// Returns the numbers of cell that are not possible to set in the raw format
    #[inline]
    fn get_not_possible_numbers_raw(&self, _row: usize, _collum: usize) -> i32 {
        debug_assert!(_row < 9 && _collum < 9);
        let mut set = 0;

        for i in 0..9 {
            if _row != i && i32::count_ones(self.field[i][_collum]) == 1 {
                set |= self.field[i][_collum];
            }

            if _collum != i && i32::count_ones(self.field[_row][i]) == 1 {
                set |= self.field[_row][i];
            }

            if _row / 3 * 3 + i / 3 != _row
                && _collum / 3 * 3 + i % 3 != _collum
                && i32::count_ones(self.field[_row / 3 * 3 + i / 3][_collum / 3 * 3 + i % 3]) == 1
            {
                set |= self.field[_row / 3 * 3 + i / 3][_collum / 3 * 3 + i % 3];
            }
        }
        for rule in &self.rules {
            set |= rule.get_not_possible_numbers_raw(&self.field, _row, _collum);
        }
        return set;
    }

    /// Sets the first alone number
    /// A alone number is a number that only appears once
    /// in a row, collum or 3x3 square in the possible numbers
    ///
    /// The function returns true, if it was able to set a
    /// alone number
    /// ```
    /// use sudoku::sudokus::Sudoku;
    ///
    /// let mut sudoku = Sudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-few-missing");
    /// sudoku.set_alone_number();
    /// ```
    pub fn set_alone_number(&mut self) -> bool {
        for i in 0..9 {
            if self.set_alone_number_row(i)
                || self.set_alone_number_collum(i)
                || self.set_alone_number_square(i / 3, i % 3)
            {
                return true;
            }
        }

        return false;
    }

    /// Sets the first alone number in row
    ///
    /// The function returns true, if it was able to set a
    /// alone number
    fn set_alone_number_row(&mut self, row: usize) -> bool {
        'numloop: for num in 0..9 {
            let mut appearances = 0;
            let mut alone_collum = 9;
            for collum in 0..9 {
                if !self.is_set(row, collum)
                    && ((0b1 << num) & !self.get_not_possible_numbers_raw(row, collum)) != 0
                {
                    appearances += 1;
                    if appearances >= 2 {
                        continue 'numloop;
                    }

                    alone_collum = collum;
                }
            }

            if appearances != 1 {
                continue;
            }
            debug_assert!(alone_collum < 9);

            self.field[row][alone_collum] = 0b1 << num;
            self.moves.alone_moves += 1;
            return true;
        }
        return false;
    }

    /// Sets the first alone number in collum
    ///
    /// The function returns true, if it was able to set a
    /// alone number
    fn set_alone_number_collum(&mut self, collum: usize) -> bool {
        'numloop: for num in 0..9 {
            let mut appearances = 0;
            let mut alone_row = 9;
            for row in 0..9 {
                if !self.is_set(row, collum)
                    && ((0b1 << num) & !self.get_not_possible_numbers_raw(row, collum)) != 0
                {
                    appearances += 1;
                    if appearances >= 2 {
                        continue 'numloop;
                    }
                    alone_row = row;
                }
            }

            if appearances != 1 {
                continue;
            }
            debug_assert!(alone_row < 9);

            self.field[alone_row][collum] = 0b1 << num;
            self.moves.alone_moves += 1;
            return true;
        }
        return false;
    }

    /// Sets the first alone number in the square big_row big_collum
    ///
    /// The function returns true, if it was able to set a
    /// alone number
    fn set_alone_number_square(&mut self, big_row: usize, big_collum: usize) -> bool {
        'numloop: for num in 0..9 {
            let mut appearances = 0;
            let mut alone_row = 9;
            let mut alone_collum = 9;
            for row in 0..3 {
                for collum in 0..3 {
                    if !self.is_set(big_row * 3 + row, big_collum * 3 + collum)
                        && ((0b1 << num)
                            & !self.get_not_possible_numbers_raw(
                                big_row * 3 + row,
                                big_collum * 3 + collum,
                            ))
                            != 0
                    {
                        appearances += 1;

                        if appearances >= 2 {
                            continue 'numloop;
                        }

                        alone_collum = big_collum * 3 + collum;
                        alone_row = big_row * 3 + row;
                    }
                }
            }

            if appearances != 1 {
                continue;
            }
            debug_assert!(alone_row < 9 && alone_collum < 9);

            self.field[alone_row][alone_collum] = 0b1 << num;
            self.moves.alone_moves += 1;
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod sudoku_test {
    use crate::sudokus::Sudoku;

    #[test]
    fn set_and_get() {
        let mut sudoku = Sudoku::empty();

        // Add -> check -> remove -> check
        for row in 0..9 {
            for collum in 0..9 {
                for i in 1..=9 {
                    sudoku.add_number(row, collum, i);

                    assert_eq!(sudoku.get_number(row, collum), i);

                    sudoku.remove_number(row, collum, i);

                    assert_eq!(sudoku.get_number(row, collum), 0);
                }
            }
        }

        // multi Add -> check -> multi remove -> check
        for row in 0..9 {
            for collum in 0..9 {
                for i in 1..=9 {
                    sudoku.add_number(row, collum, i);
                }
                assert_eq!(sudoku.get_number(row, collum), 0);
                for i in 1..=9 {
                    sudoku.remove_number(row, collum, i);
                }
                assert_eq!(sudoku.get_number(row, collum), 0);
            }
        }

        // get_numbers
        for row in 0..9 {
            for collum in 0..9 {
                // Multi Add
                let mut test_vec: Vec<i32> = Vec::new();
                for i in 1..=9 {
                    sudoku.add_number(row, collum, i);
                    test_vec.push(i);

                    assert_eq!(sudoku.get_numbers(row, collum), test_vec);
                }

                // Multi Remove
                for i in (1..=9).rev() {
                    sudoku.remove_number(row, collum, i);
                    test_vec.pop();

                    assert_eq!(sudoku.get_numbers(row, collum), test_vec);
                }
            }
        }
    }

    #[test]
    fn read_from_file() {
        let mut sudoku_from_file =
            Sudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-unsolved");

        sudoku_from_file.is_correct();
        sudoku_from_file = Sudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-solved");

        let mut sudoku_check = Sudoku::empty();
        let numbers: [i32; 81] = [
            6, 3, 5, 4, 9, 2, 1, 8, 7, 1, 7, 4, 3, 5, 8, 2, 9, 6, 2, 8, 9, 1, 7, 6, 5, 4, 3, 5, 1,
            8, 9, 3, 7, 4, 6, 2, 7, 2, 6, 8, 4, 5, 9, 3, 1, 4, 9, 3, 6, 2, 1, 8, 7, 5, 3, 5, 1, 7,
            8, 9, 6, 2, 4, 9, 6, 7, 2, 1, 4, 3, 5, 8, 8, 4, 2, 5, 6, 3, 7, 1, 9,
        ];
        let mut count = 0;
        for num in numbers {
            sudoku_check.add_number(count % 9, count / 9, num);

            count += 1;
        }

        assert_eq!(sudoku_check, sudoku_from_file);
    }

    #[test]
    fn correct() {
        let mut sudoku = Sudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-unsolved");

        assert!(!sudoku.is_solved());
        assert!(sudoku.is_part_correct());

        sudoku = Sudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-solved");

        assert!(sudoku.is_solved());
        assert!(sudoku.is_part_correct());
        assert!(sudoku.is_correct());

        sudoku = Sudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-wrong");

        assert!(sudoku.is_solved());
        assert!(!sudoku.is_part_correct());
        assert!(!sudoku.is_correct());

    }

    #[test]
    fn missing() {
        let solved_sudoku =
            Sudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-solved");

        // Easy Test that can be done in a single go
        let mut few_sudoku =
            Sudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-few-missing");
        few_sudoku.set_missing_numbers();
        assert!(few_sudoku.is_correct());
        assert_eq!(few_sudoku, solved_sudoku);

        let mut some_sudoku =
            Sudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-some-missing");
        while !some_sudoku.is_correct() && some_sudoku.set_missing_numbers() {
            assert!(some_sudoku.is_part_correct());
        }
        assert!(some_sudoku.is_correct());
        assert_eq!(some_sudoku, solved_sudoku);

        // Easy Test that can be done in a single go
        let mut easy_sudoku =
            Sudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-unsolved");

        while !easy_sudoku.is_correct() {
            easy_sudoku.set_missing_numbers();
            assert!(easy_sudoku.is_part_correct());
        }

        assert!(easy_sudoku.is_correct());
        assert_eq!(easy_sudoku, solved_sudoku);

        // Easy Test that can be done in a single go
        let mut easy_sudoku2 = Sudoku::from_file("test-sudokus/text/welt-23-11-11-easy-unsolved");

        while !easy_sudoku2.is_correct() && easy_sudoku2.set_missing_numbers() {
            assert!(easy_sudoku2.is_part_correct());
        }
        assert!(easy_sudoku2.is_correct());

        // Medium Tests cant be solved this way
        // But every step should be partialy correct
        let mut medium_sudoku =
            Sudoku::from_file("test-sudokus/text/welt-23-11-11-medium-unsolved");

        while !medium_sudoku.is_correct() && medium_sudoku.set_missing_numbers() {
            assert!(medium_sudoku.is_part_correct());
        }
    }

    #[test]
    fn alone() {
        // Easy Test that can be done in a single go
        let mut easy_sudoku =
            Sudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-unsolved");

        while !easy_sudoku.is_correct() {
            easy_sudoku.set_missing_numbers();
            easy_sudoku.set_alone_number();
            assert!(easy_sudoku.is_part_correct());
        }

        assert!(easy_sudoku.is_correct());

        let mut easy_sudoku2 = Sudoku::from_file("test-sudokus/text/welt-23-11-11-easy-unsolved");

        while !easy_sudoku2.is_correct() && easy_sudoku2.set_missing_numbers() {
            easy_sudoku2.set_alone_number();
            assert!(easy_sudoku2.is_part_correct());
        }
        assert!(easy_sudoku2.is_correct());

        // Medium Tests can finaly be solved
        let mut medium_sudoku =
            Sudoku::from_file("test-sudokus/text/welt-23-11-11-medium-unsolved");

        while !medium_sudoku.is_correct() {
            while !medium_sudoku.is_correct() && medium_sudoku.set_missing_numbers() {
                assert!(medium_sudoku.is_part_correct());
            }
            if !medium_sudoku.set_alone_number() {
                break;
            }

            assert!(medium_sudoku.is_part_correct());
        }
        assert!(medium_sudoku.is_correct());

        // Hard cant be solved but they should be correct after every step
        let mut hard_sudoku = Sudoku::from_file("test-sudokus/text/welt-23-11-11-hard-unsolved");

        while !hard_sudoku.is_correct() {
            while !hard_sudoku.is_correct() && hard_sudoku.set_missing_numbers() {
                assert!(hard_sudoku.is_part_correct());
            }
            if !hard_sudoku.set_alone_number() {
                break;
            }

            assert!(hard_sudoku.is_part_correct());
        }
    }

    #[test]
    fn count_moves() {
        for path in Sudoku::UNSOLVED_SUDOKU_FILES {
            let mut sudoku = Sudoku::from_file(path);

            let unset = sudoku.count_unset();

            sudoku.solve();

            assert_eq!(unset, sudoku.count_moves());
        }
    }
}
