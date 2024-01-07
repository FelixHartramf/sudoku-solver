pub(crate) trait SudokuRule{

    fn str_identifier() -> &'static str where Self: Sized;

    fn from_str(input: &str) -> Box<Self> where Self: Sized;

    fn complys(&self, field: &[[i32; 9]; 9]) -> bool;

    fn get_not_possible_numbers_raw(&self, field: &[[i32; 9]; 9], row: usize, collum: usize) -> i32;
}