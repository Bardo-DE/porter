//! The crate's integration tests: one executable, one module per topic.

mod matching;
mod presets;
mod shipped_files;

#[test]
fn every_module_is_declared() {
    test_kit::guard::every_module_is_declared(
        env!("CARGO_MANIFEST_DIR"),
        "tests/it",
        include_str!("main.rs"),
    );
}
