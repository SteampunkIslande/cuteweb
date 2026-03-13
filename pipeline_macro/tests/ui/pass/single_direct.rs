// Test: pipeline! with only a single direct step compiles fine.

use pipeline_macro::pipeline;
use std::path::Path;

#[derive(Debug)]
struct E(String);

fn step1(_i: &Path, _o: &Path) -> Result<(), E> {
    Ok(())
}

fn run() -> Result<(), E> {
    let i = std::path::PathBuf::from("a.parquet");
    let o = std::path::PathBuf::from("b.parquet");
    pipeline!(step1(&i, &o))?;
    Ok(())
}

fn main() {
    let _ = run;
}
