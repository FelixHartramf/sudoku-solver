/// Counts how the cells of a sudoku were set.
///
/// Only the moves that lead to the solution are counted in `missing_moves`,
/// `alone_moves` and `bruteforce_moves`. Moves that were undone while
/// backtracking are counted in the `failed_*` fields.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Moves {
    /// Cells set because only a single digit was possible ("Set missing")
    pub missing_moves: u32,
    /// Cells set because a digit was only possible in one cell of a row, column or square ("Set alone")
    pub alone_moves: u32,
    /// Cells set by guessing
    pub bruteforce_moves: u32,
    /// Cells set with `set_number`
    pub player_moves: u32,
    /// Guesses that turned out wrong
    pub bruteforce_failed_tries: u32,
    /// "Set missing" moves that were undone
    pub failed_missing_moves: u32,
    /// "Set alone" moves that were undone
    pub failed_alone_moves: u32,
    /// Candidates removed by "Naked pairs"
    pub pair_eliminations: u32,
    /// Candidates removed by "X-Wing"
    pub x_wing_eliminations: u32,
}

impl Moves {
    /// No moves at all
    pub const NONE: Moves = Moves {
        missing_moves: 0,
        alone_moves: 0,
        bruteforce_moves: 0,
        player_moves: 0,
        bruteforce_failed_tries: 0,
        failed_missing_moves: 0,
        failed_alone_moves: 0,
        pair_eliminations: 0,
        x_wing_eliminations: 0,
    };

    /// Returns the number of moves that set a cell of the current field
    pub fn total(&self) -> u32 {
        self.missing_moves + self.alone_moves + self.bruteforce_moves + self.player_moves
    }

    /// Returns the moves from before a guess (`self`) after the guess failed.
    /// The moves done in the failed `attempt` are booked as failed.
    pub(crate) fn rolled_back(&self, attempt: &Moves) -> Moves {
        Moves {
            failed_missing_moves: attempt.failed_missing_moves + attempt.missing_moves
                - self.missing_moves,
            failed_alone_moves: attempt.failed_alone_moves + attempt.alone_moves - self.alone_moves,
            bruteforce_failed_tries: attempt.bruteforce_failed_tries + 1,
            ..*self
        }
    }
}
