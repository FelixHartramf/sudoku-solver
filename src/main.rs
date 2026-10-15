use std::env;
use std::process::ExitCode;

use sudoku::sudokus::{Strategies, Sudoku};

const USAGE: &str = "Usage: sudoku [--strategies <list>] <file>

  --strategies <list>  Comma separated list of: missing, alone, pairs, x-wing, all, none
                       Default: missing,alone (the fastest for the test sudokus)";

/// Reads the arguments and returns the file path and the strategies
fn parse_args(args: &[String]) -> Result<(&str, Strategies), String> {
    match args {
        [path] => Ok((path, Strategies::default())),
        [flag, list, path] | [path, flag, list] if flag == "--strategies" => {
            let strategies = list.parse().map_err(|error| format!("{error}"))?;
            Ok((path, strategies))
        }
        _ => Err(USAGE.to_string()),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let (path, strategies) = match parse_args(&args) {
        Ok(parsed) => parsed,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(2);
        }
    };

    let mut sudoku = match Sudoku::from_file(path) {
        Ok(sudoku) => sudoku,
        Err(error) => {
            eprintln!("Could not read the sudoku `{path}`: {error}");
            return ExitCode::from(2);
        }
    };
    sudoku.set_strategies(strategies);

    let solutions = sudoku.count_solutions(2);
    if !sudoku.solve() {
        eprintln!("The sudoku has no solution:\n");
        print!("{sudoku}");
        return ExitCode::FAILURE;
    }

    println!("Solved the Sudoku with the strategies {strategies}:\n");
    println!("{sudoku}");
    if solutions > 1 {
        println!("Note: The sudoku has more than one solution, this is one of them.\n");
    }
    println!("Moves: {:#?}", sudoku.moves());
    ExitCode::SUCCESS
}
