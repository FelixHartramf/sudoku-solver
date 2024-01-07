use std::fmt;
use std::fs;


/// FastSudoku is an early copy of sudoku.
/// Before implementing ohter sudoku variants the only goal was speed.
/// Due to the code of sudoku getting more complicated I just copied the
/// code to still have the original and fast sudoku.
#[derive(Debug, Clone)]
pub struct FastSudoku {
    pub field: [[i32; 9]; 9],
}

impl PartialEq for FastSudoku {
    fn eq(&self, other: &Self) -> bool {
        self.field == other.field
    }
}

impl fmt::Display for FastSudoku {
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

        Ok(())
    }
}

impl FastSudoku {
    /// Relative Path of solveable Sudokus.
    /// The following format is used:
    /// {website}-{date of retival}-{level}-{solved/unsolved}
    ///
    /// For data structure see Sudoku::from_file
    pub const UNSOLVED_SUDOKU_FILES: [&'static str; 8] = [
        "test-sudokus/text/tagesspiegel-23-11-11-easy-unsolved",
        "test-sudokus/text/welt-23-11-11-medium-unsolved",
        "test-sudokus/text/welt-23-11-11-hard-unsolved",
        "test-sudokus/text/welt-23-11-11-easy-unsolved",
        "test-sudokus/text/tagesspiegel-23-11-11-easy-unsolved",
        "test-sudokus/text/sudoku-com-12-11-12-master-unsolved",
        "test-sudokus/text/sudoku-com-12-11-12-2-master-unsolved",
        "test-sudokus/text/wikipedia-17-given-unsolved",
    ];

    /// Returns an empty Sudoku
    /// ```
    /// use sudoku::sudokus::FastSudoku;
    ///
    /// let sudoku = FastSudoku::empty();
    /// ```
    pub fn empty() -> Self {
        Self { field: [[0; 9]; 9] }
    }

    /// Returns an Sudoku from File
    /// ```
    /// use sudoku::sudokus::FastSudoku;
    ///
    /// let sudoku = FastSudoku::from_file("test-sudokus/text/sudoku-com-12-11-12-master-solved");
    /// ```
    pub fn from_file(file_path: &str) -> Self {
        let mut field = FastSudoku::empty();
        if let Ok(data) = fs::read_to_string(file_path) {
            let mut count = 0;
            for c in data.chars() {
                if count == 81 {
                    break;
                }
                if c >= '0' && c <= '9' {
                    field.add_number(
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

        return field;
    }    

    /// Returns true, if the field is set. A single Number is writen.
    #[inline]
    pub fn is_set(&self, row: usize, collum: usize) -> bool {
        debug_assert!(row < 9 && collum < 9);
        return i32::count_ones(self.field[row][collum]) == 1;
    }

    /// Counts the number of cells in the sudoku that are not set
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
    pub fn clear(&mut self, row: usize, collum: usize) {
        debug_assert!(row < 9 && collum < 9);
        self.field[row][collum] = 0;
    }

    /// Adds the number of cell
    pub fn add_number(&mut self, row: usize, collum: usize, value: i32) {
        if value == 0 {
            return;
        }
        debug_assert!(value >= 1 && value <= 9);
        debug_assert!(row < 9 && collum < 9);
        self.field[row][collum] |= 0b1 << (value - 1);
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

        let numbers_to_check = !self.get_not_possible_numbers_raw(row_to_check, collum_to_check);

        for num in 0..9 {
            if (0b1 << num) & numbers_to_check == 0 {
                continue;
            }
            self.field[row_to_check][collum_to_check] = 0b1 << num;

            while !self.is_correct() && self.is_solveable() && self.single_solve() {}

            if self.is_correct() {
                return true;
            }
            self.field = field;
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
    /// use sudoku::sudokus::FastSudoku;
    ///
    /// let mut sudoku = FastSudoku::from_file("test-sudokus/text/sudoku-com-12-11-12-master-unsolved");
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

    /// Returns the number of a cell
    /// If no or multible numbers are set it returns 0
    pub fn get_number(&self, row: usize, collum: usize) -> i32 {
        debug_assert!(row < 9 && collum < 9);

        if self.field[row][collum].count_ones() != 1 {
            return 0;
        }

        (i32::BITS - self.field[row][collum].leading_zeros() )as i32
    }

    /// Returns true if all numbers are set
    /// It doesnt have to be solved correctly
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
    #[inline]
    pub fn is_correct(&self) -> bool {
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
    pub fn set_missing_numbers(&mut self) -> bool {
        let mut number_set = false;
        for row in 0..9 {
            for collum in 0..9 {
                if self.is_set(row, collum) {
                    continue;
                }

                if self.set_missing_number(row, collum) {
                    number_set = true;
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
        return set;
    }

    /// Sets the first alone number
    /// A alone number is a number that only appears once
    /// in a row, collum or 3x3 square in the possible numbers
    ///
    /// The function returns true, if it was able to set a
    /// alone number
    /// ```
    /// use sudoku::sudokus::FastSudoku;
    ///
    /// let mut sudoku = FastSudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-few-missing");
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
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod sudoku_test {
    use crate::sudokus::FastSudoku;

    #[test]
    fn correct() {
        // Unsolved
        let mut sudoku =
            FastSudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-unsolved");

        assert!(!sudoku.is_solved());
        assert!(sudoku.is_part_correct());

        // Solved
        sudoku = FastSudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-solved");
        assert!(sudoku.is_solved());
        assert!(sudoku.is_part_correct());
        assert!(sudoku.is_correct());

        // Wrong solve
        sudoku = FastSudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-wrong");
        assert!(sudoku.is_solved());
        assert!(!sudoku.is_part_correct());
        assert!(!sudoku.is_correct());
    }

    #[test]
    fn missing() {
        let solved_sudoku =
            FastSudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-solved");

        // Easy Test that can be done in a single go
        let mut few_sudoku =
            FastSudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-few-missing");
        few_sudoku.set_missing_numbers();
        assert!(few_sudoku.is_correct());
        assert_eq!(few_sudoku, solved_sudoku);

        let mut some_sudoku =
            FastSudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-some-missing");
        while !some_sudoku.is_correct() && some_sudoku.set_missing_numbers() {
            assert!(some_sudoku.is_part_correct());
        }
        assert!(some_sudoku.is_correct());
        assert_eq!(some_sudoku, solved_sudoku);

        // Easy Test that can be done in a single go
        let mut easy_sudoku =
            FastSudoku::from_file("test-sudokus/text/tagesspiegel-23-11-11-easy-unsolved");

        while !easy_sudoku.is_correct() {
            easy_sudoku.set_missing_numbers();
            assert!(easy_sudoku.is_part_correct());
        }

        assert!(easy_sudoku.is_correct());
        assert_eq!(easy_sudoku, solved_sudoku);

        // Easy Test that can be done in a single go
        let mut easy_sudoku2 =
            FastSudoku::from_file("test-sudokus/text/welt-23-11-11-easy-unsolved");

        while !easy_sudoku2.is_correct() && easy_sudoku2.set_missing_numbers() {
            assert!(easy_sudoku2.is_part_correct());
        }
        assert!(easy_sudoku2.is_correct());

        // Medium Tests cant be solved this way
        // But every step should be partialy correct
        let mut medium_sudoku =
            FastSudoku::from_file("test-sudokus/text/welt-23-11-11-medium-unsolved");

        while !medium_sudoku.is_correct() && medium_sudoku.set_missing_numbers() {
            assert!(medium_sudoku.is_part_correct());
        }
    }

    #[test]
    fn alone() {
        // Some sudokus can be solved with just this approach
        let solveable_path = [
            "test-sudokus/text/tagesspiegel-23-11-11-easy-unsolved",
            "test-sudokus/text/welt-23-11-11-easy-unsolved",
            "test-sudokus/text/welt-23-11-11-medium-unsolved",
        ];
        for file in solveable_path {
            let mut sudoku = FastSudoku::from_file(file);

            while !sudoku.is_correct() {
                sudoku.set_missing_numbers();
                sudoku.set_alone_number();
                assert!(sudoku.is_part_correct());
            }

            assert!(sudoku.is_correct());
        }


        // Hard cant be solved but they should be correct after every step
        let mut hard_sudoku =
            FastSudoku::from_file("test-sudokus/text/welt-23-11-11-hard-unsolved");

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
}
