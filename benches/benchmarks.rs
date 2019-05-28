use std::process::Command;
use criterion::{ Criterion, criterion_group, criterion_main };


fn small_csv_owned() {
    Command::new("target/release/rust_intro")
        .args(&["resources/summary0.csv", "resources/txns0.csv", "--impl", "owned"])
        .output()
        .expect("Process exited non-zero");
}

fn med_csv_owned() {
    Command::new("target/release/rust_intro")
        .args(&["resources/summary1000.csv", "resources/txns1000.csv", "--impl", "owned"])
        .output()
        .expect("Process exited non-zero");
}

fn large_csv_owned() {
    Command::new("target/release/rust_intro")
        .args(&["resources/summary100000.csv", "resources/txns100000.csv", "--impl", "owned"])
        .output()
        .expect("Process exited non-zero");
}

fn small_csv_refs() {
    Command::new("target/release/rust_intro")
        .args(&["resources/summary0.csv", "resources/txns0.csv", "--impl", "refs"])
        .output()
        .expect("Process exited non-zero");
}

fn py_small_csv_owned() {
    Command::new("python3")
        .args(&["resources/python_impl.py", "resources/summary0.csv", "resources/txns0.csv", "--impl", "owned"])
        .output()
        .expect("Process exited non-zero");
}

fn py_med_csv_owned() {
    Command::new("python3")
        .args(&["resources/python_impl.py", "resources/summary1000.csv", "resources/txns1000.csv", "--impl", "owned"])
        .output()
        .expect("Process exited non-zero");
}

fn py_large_csv_owned() {
    Command::new("python3")
        .args(&["resources/python_impl.py", "resources/summary100000.csv", "resources/txns100000.csv", "--impl", "owned"])
        .output()
        .expect("Process exited non-zero");
}


fn bench_small_csv_owned(c: &mut Criterion) {
    c.bench_function("Rust::small_csv::owned", |b| b.iter(|| small_csv_owned()));
}

fn bench_med_csv_owned(c: &mut Criterion) {
    c.bench_function("Rust::med_csv::owned", |b| b.iter(|| med_csv_owned()));
}

fn bench_large_csv_owned(c: &mut Criterion) {
    c.bench_function("Rust::large_csv::owned", |b| b.iter(|| large_csv_owned()));
}

fn bench_small_csv_refs(c: &mut Criterion) {
    c.bench_function("Rust::small_csv::refs", |b| b.iter(|| small_csv_refs()));
}

fn bench_py_small_csv_owned(c: &mut Criterion) {
    c.bench_function("Python::small_csv", |b| b.iter(|| py_small_csv_owned()));
}

fn bench_py_med_csv_owned(c: &mut Criterion) {
    c.bench_function("Python::med_csv", |b| b.iter(|| py_med_csv_owned()));
}

fn bench_py_large_csv_owned(c: &mut Criterion) {
    c.bench_function("Python::small_csv", |b| b.iter(|| py_large_csv_owned()));
}


criterion_group!(rust, bench_small_csv_owned, bench_med_csv_owned, bench_large_csv_owned);
criterion_group!(python, bench_py_small_csv_owned, bench_py_med_csv_owned, bench_py_large_csv_owned);
criterion_main!(rust, python);
