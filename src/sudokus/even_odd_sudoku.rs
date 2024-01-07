use super::sudoku_rule::SudokuRule;

#[derive(Debug, Clone)]
pub struct EvenOddSudoku {
    even_fields: [[bool; 9]; 9],
}

impl SudokuRule for EvenOddSudoku {
    fn str_identifier() -> &'static str {
        "even-odd-sudoku"
    }

    fn from_str(_input: &str) -> Box<Self> {
        let mut even_odd = Self::empty();
        let mut count = 0;
        for c in _input.chars() {
            if count == 81 {
                break;
            }
            if c == '0' || c == '1' {
                even_odd.even_fields[count % 9][count / 9] = c.to_digit(10).unwrap_or(0) == 0;
                count += 1;
            }
        }

        // ToDo: Return Result and not just panic if their is an error while parsing
        if even_odd.is_valid(){
            panic!();
        }
        Box::new(even_odd)
    }

    fn complys(&self, field: &[[i32; 9]; 9]) -> bool {
        for row in 0..9 {
            for column in 0..9 {
                if field[row][column].count_ones() != 1 {
                    continue;
                }

                if field[row][column].trailing_zeros() % 2 == 0 && self.even_fields[row][column] {
                    return false;
                }

                if field[row][column].trailing_zeros() % 2 != 0 && !self.even_fields[row][column] {
                    return false;
                }
            }
        }

        true
    }

    fn get_not_possible_numbers_raw(
        &self,
        _field: &[[i32; 9]; 9],
        row: usize,
        collum: usize,
    ) -> i32 {
        if self.even_fields[row][collum] {
            return 0b101_010_101;
        }

        0b010_101_010
    }
}

impl EvenOddSudoku {
    fn empty() -> Self {
        Self {
            even_fields: [[false; 9]; 9],
        }
    }

    fn is_valid(&self) -> bool {
        // Their are 36 even fields in a classic sudoku
        self.even_fields
            .map(|r| {
                r.map(|e| {
                    if e {
                        return 0;
                    }
                    1
                })
                .as_slice().iter().sum::<i32>()
            })
            .as_slice().iter().sum::<i32>() == 36
    }
}

#[cfg(test)]
mod x_sudoku_test {
    use super::EvenOddSudoku;
    use crate::sudokus::sudoku_rule::SudokuRule;

    #[test]
    fn comply() {
        // ToDo: Write tests
    }
}
