# Sudoku Solver

## What it does
It solves Sudokus (fast?). All sudokus I use for testing are solved in less than 500µs on my machine.


## Features
- fast Sudoku solving


## Usage 
To solve a Sudoku run:
```
./sudoku [file]
```

For the dataformat of the file, see the examples in: `./test-sudokus/text/`

### Build from source
It's Rust. Just run: `cargo build` or `cargo build --release` in the directory 

## Concepts

### Data Structure
The field is a standard 2D-Array of integers. Set numbers and possible numbers (used in all solving strategies) are set in the same array. A simple Bitshift is used for storage.

```
0b1: 1 is set
0b10: 2 is set
0b100: 3 is set
...
0b100_000_000: 9 is set


0b101: 1 and 3 are possible numbers
0b111_111_111: All numbers are possible
```

The reason is that nearly all calculations can be done with simple Bit-Operations.

### Sudoku Rules
To support other sudoku variants I implemented a trait SudokuRule. With this it is possible to easily implement some sudoku variants with minimal change in the underlying solving strategies.

See `sudoku_rules.rs` for more details.


### Solving Strategie
Currently only 3 Strategies are implemented:
-  Set missing: Sets the numbers that are possible. If it is a single number, a new cell is set

- Set alone: If a cell of row 1 is the only one where number 4 is possible, than this cell can be set to 4.

- Brutforce: Set a randome unset cell to a possible number. If it than can be solved, it is sovled. Otherwise try the next candidate.

Those 3 Strategies are used from top to bottom. If "Set missing" doesn't set a number, "Set alone" is used. So Brutforce is only used if it is neccassary.

Those 3 are enogh to solve all sudokus fast enough. See ToDo-Section for ideas to make it better and maby faster.


## Purpose
This is a explorative project. I used it to learn about Rust and some of the features of the build process. That is the reason why some features are excessivly used, others not at all and a few probably wrong. So don't take it as an example for good Rust code.

## ToDo
- [ ] Better Docs
- [ ] More Sudokus for testing and benchmarking
- [ ] More solving strategies for solving (e.g. twins, dripples, X-Wing)
- [x] ~~Multithreading~~ After I quick test, I don't think that it is worth it. The overhead is to mouch and the parallism to short. It is better/cleaner/easier to just use `RUSTFLAGS="-C target-cpu=native"` for the speed up.
- [ ] Support for other Sudoku variant 
    - [x] X Sudoku
    - [ ] Killer Sudoku
    - [ ] Hyper Sudoku
    - [x] Even-Odd Sudoku
    - [ ] Comparison Sudoku
- [ ] Player Moves aren't counted as aspected

## Help me get better
I still need to learn a lot about Rust, good documentation, naming, programm structure and more. So if you have any suggestions please open an issue.