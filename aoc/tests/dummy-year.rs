use serial_test::serial;
use std::process::Command;

fn run_with(args: &[&str]) -> String {
    let output = Command::new("cargo")
        .args(["run", "--"])
        .args(args)
        .current_dir("../dummy-year")
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap()
}

#[test]
#[serial]
fn day1() {
    insta::assert_snapshot!(run_with(&["-d", "1"]), @r###"
    Day 1 - part 1: 232
    Day 1 - part 1 — str_slice: 232
    Day 1 - part 2: 1783
    Day 1 - part 2 — result: 1783
    Day 1 - part 2 — result_string: 1783
    "###);
}

#[test]
#[serial]
fn day1_main() {
    insta::assert_snapshot!(run_with(&["-d", "1", "-m"]), @r###"
    Day 1 - part 1: 232
    Day 1 - part 2: 1783
    "###);
}

#[test]
#[serial]
fn day2() {
    insta::assert_snapshot!(run_with(&["-d", "2"]), @r###"
    Day 2 - part 1: 1606483
    Day 2 - part 2: 20x3x11
                    15x27x5
    Day 2 - part 2 — no_eol: 20x3x11
                             15x27x5

    "###);
}

#[test]
#[serial]
fn all_days() {
    insta::assert_snapshot!(run_with(&["-a"]), @r###"
    Day 1 - part 1: 232
    Day 1 - part 1 — str_slice: 232
    Day 1 - part 2: 1783
    Day 1 - part 2 — result: 1783
    Day 1 - part 2 — result_string: 1783
    Day 2 - part 1: 1606483
    Day 2 - part 2: 20x3x11
                    15x27x5
    Day 2 - part 2 — no_eol: 20x3x11
                             15x27x5
    Day 3 - part 1: 8134

    "###);
}

#[test]
#[serial]
fn all_days_main() {
    insta::assert_snapshot!(run_with(&["-a", "-m"]), @r###"
    Day 1 - part 1: 232
    Day 1 - part 2: 1783
    Day 2 - part 1: 1606483
    Day 2 - part 2: 20x3x11
                    15x27x5
    Day 3 - part 1: 8134

    "###);
}
