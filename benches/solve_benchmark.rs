use criterion::{criterion_group, criterion_main, Criterion};
use sudoku::sudokus::{FastSudoku, Sudoku};

fn bench_fastsudoku(c: &mut Criterion) {
    let mut sudokus = vec![];
    for sudoku_path in FastSudoku::UNSOLVED_SUDOKU_FILES {
        sudokus.push(FastSudoku::from_file(sudoku_path));
    }

    c.bench_function("Fastsudoku", |b| {
        b.iter(|| {
            for sudoku_clean in sudokus.iter() {
                let mut sudoku = sudoku_clean.clone();
                sudoku.solve();
            }
        })
    });
}

fn bench_sudoku(c: &mut Criterion) {
    let mut sudokus = vec![];
    for sudoku_path in Sudoku::UNSOLVED_SUDOKU_FILES {
        sudokus.push(Sudoku::from_file(sudoku_path));
    }

    c.bench_function("Sudoku", |b| {
        b.iter(|| {
            for sudoku in sudokus.iter_mut() {
                let field = sudoku.field;
                sudoku.solve();
                sudoku.field = field;
            }
        })
    });
}

criterion_group!(benches, bench_fastsudoku, bench_sudoku);
criterion_main!(benches);
