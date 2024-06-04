use std::env;

use rswind::create_app;
use rswind_cli::{
    io::{get_files, FileInput},
    run::RunParallel,
};
use rswind_extractor::Extractable;
use criterion::{criterion_group, criterion_main, Criterion};
use dotenv::dotenv;
use rayon::prelude::*;
use rustc_hash::FxHashSet as HashSet;

pub fn criterion_benchmark(c: &mut Criterion) {
    dotenv().ok();

    let input_path = env::var("BENCH_INPUT_PATH").expect("BENCH_INPUT_PATH is not set");

    c.bench_function("Parse Multi With Read", |b| {
        b.iter(|| {
            let mut app = create_app();
            let files = get_files(&input_path);

            app.run_parallel(files);
        });
    });

    c.bench_function("Parse Multi Without Read", |b| {
        let files = get_files(&input_path)
            .into_par_iter()
            .map(|f| FileInput::from_file(&f))
            .collect::<Vec<_>>();

        b.iter(|| {
            let mut app = create_app();

            app.run_parallel_with(files.par_iter().map(|f| f.extract()).reduce(
                HashSet::default,
                |mut acc, x| {
                    acc.extend(x);
                    acc
                },
            ))
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);