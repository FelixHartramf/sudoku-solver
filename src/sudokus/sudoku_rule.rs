use std::fmt::Debug;
use std::sync::Arc;

use super::{EvenOddSudoku, Field, Mask, ParseError, XSudoku};

/// An additional rule on top of the classic sudoku rules.
///
/// The solving strategies only use `excluded`, so a new variant just has to
/// say which digits are not allowed in a cell.
pub trait SudokuRule: Debug + Send + Sync {
    /// Returns the digits that this rule forbids in the empty cell at `row`, `column`
    fn excluded(&self, field: &Field, row: usize, column: usize) -> Mask;

    /// Returns true if the set cells of `field` don't break this rule
    fn complies(&self, field: &Field) -> bool;
}

/// The rules a [`GenericSudoku`](super::GenericSudoku) is solved with.
pub trait RuleSet: Clone + Debug + Default {
    /// Adds the rule of a file section, e.g. `x-sudoku`
    fn add_section(&mut self, name: &str, body: &str) -> Result<(), ParseError>;

    /// Returns the digits that the rules forbid in the empty cell at `row`, `column`
    fn excluded(&self, field: &Field, row: usize, column: usize) -> Mask;

    /// Returns true if the set cells of `field` don't break any rule
    fn complies(&self, field: &Field) -> bool;
}

/// Only the classic rules. Everything compiles away, which makes it the fastest option.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoRules;

impl RuleSet for NoRules {
    fn add_section(&mut self, name: &str, _body: &str) -> Result<(), ParseError> {
        Err(ParseError::UnsupportedRule(name.to_string()))
    }

    #[inline]
    fn excluded(&self, _field: &Field, _row: usize, _column: usize) -> Mask {
        0
    }

    #[inline]
    fn complies(&self, _field: &Field) -> bool {
        true
    }
}

/// Any number of sudoku variants that are chosen at runtime
#[derive(Debug, Clone, Default)]
pub struct VariantRules {
    rules: Vec<Arc<dyn SudokuRule>>,
}

impl VariantRules {
    /// Adds a rule
    pub fn push(&mut self, rule: impl SudokuRule + 'static) {
        self.rules.push(Arc::new(rule));
    }

    /// Returns the number of rules
    pub fn len(&self) -> usize {
        self.rules.len()
    }

    /// Returns true if there are no rules, i.e. it is a classic sudoku
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

impl RuleSet for VariantRules {
    fn add_section(&mut self, name: &str, body: &str) -> Result<(), ParseError> {
        match name {
            XSudoku::IDENTIFIER => self.push(XSudoku::parse(body)?),
            EvenOddSudoku::IDENTIFIER => self.push(EvenOddSudoku::parse(body)?),
            _ => return Err(ParseError::UnknownSection(name.to_string())),
        }
        Ok(())
    }

    #[inline]
    fn excluded(&self, field: &Field, row: usize, column: usize) -> Mask {
        self.rules.iter().fold(0, |excluded, rule| {
            excluded | rule.excluded(field, row, column)
        })
    }

    fn complies(&self, field: &Field) -> bool {
        self.rules.iter().all(|rule| rule.complies(field))
    }
}
