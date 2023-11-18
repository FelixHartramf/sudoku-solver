use std::env;

use sudoku::Sudoku;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please Provide a single file of a Sudoku to solve");
        return;
    }
    
    let mut sudoku = Sudoku::from_file(&args[1]);

    if sudoku.solve() {
        println!("Solved the Sudoku: ");
    }

    println!("{sudoku}");
}
