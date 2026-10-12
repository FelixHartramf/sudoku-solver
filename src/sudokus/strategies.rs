use std::fmt;
use std::str::FromStr;

/// The logical strategies the solver uses before it starts guessing.
///
/// Guessing (backtracking) is always used, so every combination solves every
/// sudoku. The strategies only change how fast it is.
///
/// ```
/// use sudoku::sudokus::{Strategies, Sudoku};
///
/// let mut sudoku = Sudoku::from_file("test-sudokus/text/welt-23-11-11-hard-unsolved").unwrap();
/// sudoku.set_strategies(Strategies::ALL);
/// assert!(sudoku.solve());
///
/// let strategies: Strategies = "missing,alone".parse().unwrap();
/// assert!(strategies.set_missing && !strategies.naked_pairs);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Strategies {
    /// "Set missing" (naked single): a cell with a single candidate is set
    pub set_missing: bool,
    /// "Set alone" (hidden single): a digit with a single cell in a unit is set
    pub set_alone: bool,
    /// "Naked pairs": two cells with the same two candidates
    pub naked_pairs: bool,
    /// "X-Wing": a digit in the same two columns of two rows
    pub x_wing: bool,
}

impl Strategies {
    /// All strategies
    pub const ALL: Strategies = Strategies {
        set_missing: true,
        set_alone: true,
        naked_pairs: true,
        x_wing: true,
    };

    /// No strategy, only guessing
    pub const NONE: Strategies = Strategies {
        set_missing: false,
        set_alone: false,
        naked_pairs: false,
        x_wing: false,
    };

    /// The fastest combination for the sudokus in `test-sudokus/`,
    /// measured with the `Strategies` group of `cargo bench`
    pub const FASTEST: Strategies = Strategies {
        set_missing: true,
        set_alone: true,
        naked_pairs: false,
        x_wing: false,
    };

    /// Names used by `Display` and `FromStr`
    const NAMES: [&'static str; 4] = ["missing", "alone", "pairs", "x-wing"];

    fn flags(&self) -> [bool; 4] {
        [
            self.set_missing,
            self.set_alone,
            self.naked_pairs,
            self.x_wing,
        ]
    }

    fn from_flags(flags: [bool; 4]) -> Self {
        let [set_missing, set_alone, naked_pairs, x_wing] = flags;
        Self {
            set_missing,
            set_alone,
            naked_pairs,
            x_wing,
        }
    }

    /// Returns all 16 combinations
    pub fn all_combinations() -> impl Iterator<Item = Strategies> {
        (0..16).map(|bits: u8| Self::from_flags([0, 1, 2, 3].map(|i| bits & (1 << i) != 0)))
    }
}

impl Default for Strategies {
    fn default() -> Self {
        Self::FASTEST
    }
}

impl fmt::Display for Strategies {
    /// Prints the strategies as comma separated list, e.g. `missing,alone`, or `none`
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let names: Vec<_> = Self::NAMES
            .iter()
            .zip(self.flags())
            .filter_map(|(name, used)| used.then_some(*name))
            .collect();

        if names.is_empty() {
            write!(f, "none")
        } else {
            write!(f, "{}", names.join(","))
        }
    }
}

/// The name of a strategy is unknown
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownStrategy(pub String);

impl fmt::Display for UnknownStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "unknown strategy `{}`, possible are: {}, all, none",
            self.0,
            Strategies::NAMES.join(", ")
        )
    }
}

impl std::error::Error for UnknownStrategy {}

impl FromStr for Strategies {
    type Err = UnknownStrategy;

    /// Reads a comma separated list of `missing`, `alone`, `pairs`, `x-wing`, `all` and `none`
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut flags = [false; 4];
        for name in input
            .split(',')
            .map(str::trim)
            .filter(|name| !name.is_empty())
        {
            match name {
                "all" => flags = [true; 4],
                "none" => {}
                _ => {
                    let index = Self::NAMES
                        .iter()
                        .position(|known| *known == name)
                        .ok_or_else(|| UnknownStrategy(name.to_string()))?;
                    flags[index] = true;
                }
            }
        }
        Ok(Self::from_flags(flags))
    }
}

#[cfg(test)]
mod strategies_test {
    use super::{Strategies, UnknownStrategy};

    #[test]
    fn all_combinations() {
        let combinations: Vec<_> = Strategies::all_combinations().collect();
        assert_eq!(combinations.len(), 16);
        assert_eq!(combinations[0], Strategies::NONE);
        assert_eq!(combinations[15], Strategies::ALL);
        for (i, a) in combinations.iter().enumerate() {
            assert!(!combinations[i + 1..].contains(a));
        }
    }

    #[test]
    fn display_and_parse() {
        for strategies in Strategies::all_combinations() {
            assert_eq!(strategies.to_string().parse(), Ok(strategies));
        }
        assert_eq!(Strategies::NONE.to_string(), "none");
        assert_eq!(Strategies::ALL.to_string(), "missing,alone,pairs,x-wing");
        assert_eq!("all".parse(), Ok(Strategies::ALL));
        assert_eq!("".parse(), Ok(Strategies::NONE));
        assert_eq!(
            " pairs , x-wing ".parse(),
            Ok(Strategies {
                naked_pairs: true,
                x_wing: true,
                ..Strategies::NONE
            })
        );
        assert_eq!(
            "missing,swordfish".parse::<Strategies>(),
            Err(UnknownStrategy("swordfish".to_string()))
        );
    }
}
