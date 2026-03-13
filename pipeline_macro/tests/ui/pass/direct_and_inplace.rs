// Test: pipeline! compiles correctly with direct + in-place steps.

use pipeline_macro::pipeline;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct PipelineError(String);

fn step1(_input: &Path, _output: &Path) -> Result<(), PipelineError> {
    Ok(())
}
fn step2(_input: &Path, _output: &Path) -> Result<(), PipelineError> {
    Ok(())
}
fn step3(_input: &Path, _output: &Path, _a: u64) -> Result<(), PipelineError> {
    Ok(())
}
fn step4(_input: &Path, _output: &Path, _a: &str, _b: i32) -> Result<(), PipelineError> {
    Ok(())
}
fn step5() -> Result<(), PipelineError> {
    Ok(())
}

fn run() -> Result<(), PipelineError> {
    let pipeline_input = PathBuf::from("raw/RUN1.parquet");
    let pipeline_output = PathBuf::from("results/RUN1.parquet");

    pipeline!(
        step1(&pipeline_input, &pipeline_output),
        step2((&pipeline_output, &pipeline_output)),
        step3((&pipeline_output, &pipeline_output), 42),
        step4((&pipeline_output, &pipeline_output), "hello world", 15),
        step5()
    )?;

    Ok(())
}

fn main() {
    let _ = run;
}
