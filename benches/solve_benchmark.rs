use criterion::{criterion_group, criterion_main, Criterion};
use sudoku::sample_files;
use sudoku::sudokus::{FastSudoku, Sudoku};

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

criterion_group!(benches, bench_fastsudoku, bench_sudoku, bench_single);
criterion_main!(benches);
