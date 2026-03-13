/// UI (compile-test) suite for `pipeline_macro`.
///
/// Pass tests  → must compile without error.
/// Fail tests  → must emit the expected compile-error saved in `<name>.stderr`.
///
/// Run with: `cargo test -p pipeline_macro`
#[test]
fn ui_pass() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/pass/*.rs");
}

#[test]
fn ui_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
}
