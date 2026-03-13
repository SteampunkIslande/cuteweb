// Test: using a tuple with different elements must produce a compile error.

use pipeline_macro::pipeline;
use std::path::Path;

#[derive(Debug)]
struct E(String);

fn step(_i: &Path, _o: &Path) -> Result<(), E> {
    Ok(())
}

fn run() -> Result<(), E> {
    let a = std::path::PathBuf::from("a.parquet");
    let b = std::path::PathBuf::from("b.parquet");
    pipeline!(
        // (a, b) — different identifiers → must fail at macro-expansion time
        step((&a, &b))
    )?;
    Ok(())
}

fn main() {
    let _ = run;
}
