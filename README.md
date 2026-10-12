# Sudoku Solver

## What it does
It solves Sudokus fast. Most sudokus I use for testing are solved in less than 10µs on my machine, the hardest one ("World's hardest sudoku" by Arto Inkala) in about 0.5ms (`cargo bench`).


## Features
- fast Sudoku solving
- Sudoku variants: X-Sudoku and Even-Odd-Sudoku
- checks if a sudoku has a unique solution
- counts which strategy set how many cells
- the solving strategies can be chosen


## Usage 
To solve a Sudoku run:
```
./sudoku [--strategies <list>] <file>
```

`--strategies` chooses the solving strategies as comma separated list of `missing`, `alone`, `pairs`, `x-wing`, `all` and `none` (see [Solving Strategies](#solving-strategies)). The default is `missing,alone`.

The exit code is `0` if the sudoku was solved, `1` if it has no solution and `2` if the file could not be read.

### File format
For examples see `./test-sudokus/text/`. A file consists of sections that end with `;`. Each section starts with its name, followed by `:` and its content. Whitespace is ignored.

```
field:
0 0 2  0 0 0  0 0 0
5 6 0  0 0 0  7 0 0
...;

x-sudoku;

even-odd-sudoku:
1 0 0  0 1 1  1 0 1
...;
```

- `field`: the 81 cells in reading order, `0` (or `.`) is an empty cell
- `x-sudoku`: both diagonals contain every digit once
- `even-odd-sudoku`: `0` marks an even cell, `1` an odd cell

### Library
```rust
use sudoku::sudokus::{Strategies, Sudoku};

let mut sudoku = Sudoku::from_file("test-sudokus/text/welt-23-11-11-hard-unsolved")?;
assert!(sudoku.has_unique_solution());
sudoku.set_strategies(Strategies::ALL); // optional, the default is Strategies::FASTEST
sudoku.solve();
println!("{sudoku}");
```

### Build from source
It's Rust. Just run: `cargo build` or `cargo build --release` in the directory.
Before pushing run `cargo fmt`, `cargo clippy --all-targets` and `cargo test`, the CI checks them.

## Concepts

### Data Structure
Each cell is a bitmask (`u16`). A set cell has exactly one bit set, an empty cell is `0`.

```
0b1: 1
0b10: 2
0b100: 3
...
0b100_000_000: 9

0b101: 1 and 3 (used for candidates)
0b111_111_111: All numbers
```

Besides the cells the board stores the digits used in every row, column and 3x3 square. They are updated whenever a cell is set, so the candidates of a cell are just:

```rust
ALL_DIGITS & !(rows[row] | columns[column] | squares[square] | rules.excluded(...))
```

The whole board is `Copy`. Saving the state before a guess is a single memcpy.

### Sudoku Rules
To support sudoku variants there is the trait `SudokuRule`. A rule only has to say which digits it forbids in a cell (`excluded`) and whether a field breaks it (`complies`). The solving strategies work with every rule without any change.

The sudoku is generic over its `RuleSet`:
- `Sudoku = GenericSudoku<VariantRules>`: the rules are chosen at runtime, e.g. from a file
- `FastSudoku = GenericSudoku<NoRules>`: only classic rules, all rule checks are optimized away

Both share the same code. See `src/sudokus/sudoku_rule.rs` for more details.


### Solving Strategies
Currently 5 strategies are implemented:
- Set missing (naked single): If only a single number is possible in a cell, it is set.

- Set alone (hidden single): If a cell of row 1 is the only one where number 4 is possible, then this cell can be set to 4. The same works for columns and squares.

- Naked pairs: If two cells of a row, column or square have the same two candidates (e.g. 3 and 7), these two digits have to be in these cells. So 3 and 7 can be removed from the candidates of all other cells of the unit.

- X-Wing: If in two rows a digit is only possible in the same two columns, it has to be in the corners of this rectangle. So it can be removed from all other cells of these two columns. The same works with rows and columns swapped.

- Bruteforce (backtracking): Take the empty cell with the fewest candidates and try them one after another. If one leads to a contradiction, the state before the guess is restored and the next candidate is tried.

"Set missing" is used until it doesn't find anything, then "Set alone", then "Naked pairs" and "X-Wing". Every success starts again with "Set missing". Bruteforce is only used if all of them are stuck.

"Set missing" and "Set alone" also detect contradictions (a cell without candidates, a digit without a cell), so a wrong guess is noticed early.

"Naked pairs" and "X-Wing" don't set cells, they only remove candidates. The removed candidates are stored per cell in the board (`eliminated`).

#### Choosing the strategies
Bruteforce is always used, so every combination of the other 4 strategies solves every sudoku. They only change the speed. The tests check all 16 combinations.

The `Strategies` group of `cargo bench` compares them on all test sudokus (time for all 13 together):

| Strategies | Time |
|---|---|
| missing, alone | 665 – 700 µs |
| missing, alone, pairs | 660 – 700 µs |
| missing, alone, pairs, x-wing | 774 µs |
| missing, alone, x-wing | 865 µs |
| alone | 999 µs |
| alone, pairs | 1297 µs |
| alone, x-wing | 1388 µs |
| alone, pairs, x-wing | 1476 µs |
| missing | 1752 µs |
| missing, pairs | 2380 µs |
| missing, pairs, x-wing | 3021 µs |
| missing, x-wing | 5145 µs |
| pairs / x-wing / pairs, x-wing / none | 0.4 – 8 s (nearly pure guessing) |

`missing,alone` and `missing,alone,pairs` are equal within the noise of several runs. The sum is dominated by the hardest sudoku, where "Naked pairs" helps (497µs → 385µs). On the other sudokus it is slower or the same (e.g. AI Escargot 19µs → 60µs, the 17-clue sudoku 5µs → 9µs), because a guess is cheaper than searching for pairs. So the default (`Strategies::FASTEST`) is `missing,alone`.

See ToDo-Section for ideas to make it better and maybe faster.


## Purpose
This is an explorative project. I used it to learn about Rust and some of the features of the build process. That is the reason why some features are excessively used, others not at all and a few probably wrong. So don't take it as an example for good Rust code.

## ToDo
- [x] Better Docs
- [ ] More Sudokus for testing and benchmarking (the choice of the default strategies depends on them)
- [ ] More solving strategies for solving
    - [x] Naked pairs
    - [x] X-Wing
    - [ ] Hidden pairs, pointing pairs, Swordfish
- [x] ~~Multithreading~~ After a quick test, I don't think that it is worth it. The overhead is too much and the parallelism too short. It is better/cleaner/easier to just use `RUSTFLAGS="-C target-cpu=native"` for the speed up.
- [ ] Support for other Sudoku variants
    - [x] X Sudoku
    - [ ] Killer Sudoku
    - [ ] Hyper Sudoku
    - [x] Even-Odd Sudoku
    - [ ] Comparison Sudoku
- [x] Player Moves are counted

## Help me get better
I still need to learn a lot about Rust, good documentation, naming, program structure and more. So if you have any suggestions please open an issue.
