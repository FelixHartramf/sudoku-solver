use std::env;
use std::process::ExitCode;

use sudoku::sudokus::Sudoku;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let [_, path] = args.as_slice() else {
        eprintln!("Usage: sudoku <file>");
        return ExitCode::from(2);
    };

    let mut sudoku = match Sudoku::from_file(path) {
        Ok(sudoku) => sudoku,
        Err(error) => {
            eprintln!("Could not read the sudoku `{path}`: {error}");
            return ExitCode::from(2);
        }
    };

    let solutions = sudoku.count_solutions(2);
    if !sudoku.solve() {
        eprintln!("The sudoku has no solution:\n");
        print!("{sudoku}");
        return ExitCode::FAILURE;
    }

    println!("Solved the Sudoku:\n");
    println!("{sudoku}");
    if solutions > 1 {
        println!("Note: The sudoku has more than one solution, this is one of them.\n");
    }
    println!("Moves: {:#?}", sudoku.moves());
    ExitCode::SUCCESS
}
