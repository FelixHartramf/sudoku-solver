use criterion::{criterion_group, criterion_main, Criterion};
use sudoku::Sudoku;


fn criterion_benchmark(c: &mut Criterion) {

    for sudoku_path in Sudoku::UNSOLVED_SUDOKU_FILES {
        let sudoku_clean = Sudoku::from_file(sudoku_path);

        
        c.bench_function("Bench Solve", |b| b.iter(|| 
        {
            let mut sudoku = sudoku_clean.clone();
            sudoku.solve();
        }
        ));
    }

    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

