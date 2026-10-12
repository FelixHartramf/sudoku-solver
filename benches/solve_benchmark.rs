use criterion::{criterion_group, criterion_main, Criterion};
use sudoku::sample_files;
use sudoku::sudokus::{FastSudoku, Strategies, Sudoku};

fn bench_fastsudoku(c: &mut Criterion) {
    let sudokus: Vec<_> = sample_files::CLASSIC_UNSOLVED
        .iter()
        .map(|path| FastSudoku::from_file(path).unwrap())
        .collect();

    c.bench_function("Fastsudoku", |b| {
        b.iter(|| {
            for sudoku in &sudokus {
                sudoku.clone().solve();
            }
        })
    });
}

fn bench_sudoku(c: &mut Criterion) {
    let sudokus: Vec<_> = sample_files::unsolved()
        .map(|path| Sudoku::from_file(path).unwrap())
        .collect();

    c.bench_function("Sudoku", |b| {
        b.iter(|| {
            for sudoku in &sudokus {
                sudoku.clone().solve();
            }
        })
    });
}

fn bench_single(c: &mut Criterion) {
    let mut group = c.benchmark_group("Single");
    for path in sample_files::unsolved() {
        let sudoku = Sudoku::from_file(path).unwrap();
        let name = path.rsplit('/').next().unwrap_or(path);
        group.bench_function(name, |b| b.iter(|| sudoku.clone().solve()));
    }
    group.finish();
}

/// Compares the strategy combinations on all sudokus.
///
/// Combinations without "Set missing" and "Set alone" are left out, they are
/// nearly pure guessing and need seconds for the hard sudokus.
fn bench_strategies(c: &mut Criterion) {
    let sudokus: Vec<_> = sample_files::unsolved()
        .map(|path| Sudoku::from_file(path).unwrap())
        .collect();

    let mut group = c.benchmark_group("Strategies");
    for strategies in Strategies::all_combinations().filter(|s| s.set_missing || s.set_alone) {
        let sudokus: Vec<_> = sudokus
            .iter()
            .cloned()
            .map(|mut sudoku| {
                sudoku.set_strategies(strategies);
                sudoku
            })
            .collect();

        group.bench_function(strategies.to_string(), |b| {
            b.iter(|| {
                for sudoku in &sudokus {
                    sudoku.clone().solve();
                }
            })
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_fastsudoku,
    bench_sudoku,
    bench_single,
    bench_strategies
);
criterion_main!(benches);
