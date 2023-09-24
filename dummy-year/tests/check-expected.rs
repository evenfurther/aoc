#[test]
fn check_expected() {
    assert!(aoc::test::check_results(
        dummy_year::register::register_runners,
        "expected.txt",
        false
    )
    .unwrap());
}
