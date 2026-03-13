// Test: pipeline! with only in-place steps (no direct step) works fine.

use pipeline_macro::pipeline;
use std::path::Path;

#[derive(Debug)]
struct E(String);

fn step(_i: &Path, _o: &Path) -> Result<(), E> {
    Ok(())
}

fn run() -> Result<(), E> {
    let out = std::path::PathBuf::from("results/out.parquet");
    pipeline!(step((&out, &out)), step((&out, &out)))?;
    Ok(())
}

fn main() {
    let _ = run;
}
