use std::error::Error;
use std::fmt;
use std::io;

/// Error while reading a sudoku
#[derive(Debug)]
pub enum ParseError {
    /// The file could not be read
    Io(io::Error),
    /// There is no `field:` section
    MissingField,
    /// A section appears more than once
    DuplicateSection(String),
    /// A section contains a character that isn't allowed there
    InvalidCharacter { section: String, character: char },
    /// A grid section doesn't contain exactly 81 cells
    WrongCellCount { section: String, found: usize },
    /// The section name isn't known
    UnknownSection(String),
    /// The rule exists, but the sudoku type can't handle it (e.g. a variant in a `FastSudoku`)
    UnsupportedRule(String),
    /// The content of a rule section is invalid
    InvalidRule { rule: String, reason: String },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "could not read the file: {error}"),
            Self::MissingField => write!(f, "the `field` section is missing"),
            Self::DuplicateSection(section) => write!(f, "the section `{section}` appears twice"),
            Self::InvalidCharacter { section, character } => {
                write!(f, "invalid character `{character}` in section `{section}`")
            }
            Self::WrongCellCount { section, found } => {
                write!(f, "section `{section}` needs 81 cells, but has {found}")
            }
            Self::UnknownSection(section) => write!(f, "unknown section `{section}`"),
            Self::UnsupportedRule(rule) => {
                write!(f, "the rule `{rule}` is not supported by this sudoku type")
            }
            Self::InvalidRule { rule, reason } => write!(f, "invalid rule `{rule}`: {reason}"),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// Reads the 81 cells of a grid section in reading order.
///
/// Whitespace is ignored, every other character is converted with `convert`.
pub(crate) fn parse_grid<T: Copy + Default>(
    section: &str,
    body: &str,
    convert: impl Fn(char) -> Option<T>,
) -> Result<[[T; 9]; 9], ParseError> {
    let mut grid = [[T::default(); 9]; 9];
    let mut count = 0;

    for character in body.chars().filter(|c| !c.is_whitespace()) {
        let value = convert(character).ok_or_else(|| ParseError::InvalidCharacter {
            section: section.to_string(),
            character,
        })?;
        if count < 81 {
            grid[count / 9][count % 9] = value;
        }
        count += 1;
    }

    if count != 81 {
        return Err(ParseError::WrongCellCount {
            section: section.to_string(),
            found: count,
        });
    }
    Ok(grid)
}

#[cfg(test)]
mod parse_test {
    use super::{parse_grid, ParseError};

    fn digit(c: char) -> Option<u32> {
        c.to_digit(10)
    }

    #[test]
    fn reading_order() {
        let body: String = (0..81).map(|i| format!("{} ", i % 10)).collect();
        let grid = parse_grid("field", &body, digit).unwrap();

        assert_eq!(grid[0][1], 1);
        assert_eq!(grid[1][0], 9);
        assert_eq!(grid[8][8], 0);
    }

    #[test]
    fn errors() {
        assert!(matches!(
            parse_grid("field", "1 2 3", digit),
            Err(ParseError::WrongCellCount { found: 3, .. })
        ));
        assert!(matches!(
            parse_grid("field", &"0".repeat(82), digit),
            Err(ParseError::WrongCellCount { found: 82, .. })
        ));
        assert!(matches!(
            parse_grid("field", "1 x", digit),
            Err(ParseError::InvalidCharacter { character: 'x', .. })
        ));
    }
}
