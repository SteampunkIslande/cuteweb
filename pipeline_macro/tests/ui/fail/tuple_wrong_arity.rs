// Test: a tuple with 3 elements must produce a compile error.

use pipeline_macro::pipeline;
use std::path::Path;

#[derive(Debug)]
struct E(String);

fn step(_i: &Path, _o: &Path) -> Result<(), E> {
    Ok(())
}

fn run() -> Result<(), E> {
    let a = std::path::PathBuf::from("a.parquet");
    pipeline!(step((&a, &a, &a)))?;
    Ok(())
}

fn main() {
    let _ = run;
}
