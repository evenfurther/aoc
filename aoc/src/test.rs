use eyre::{Context, bail};
use std::path::Path;
use std::process::Command;

const ENV_VAR: &str = "RECORD_RESULTS";

fn equal_content<P: AsRef<Path>>(actual: &str, expected: P, show_diff: bool) -> eyre::Result<bool> {
    let expected_content = std::fs::read_to_string(expected.as_ref()).context(format!(
        "cannot read {}",
        expected.as_ref().to_string_lossy()
    ))?;
    if actual == expected_content {
        Ok(true)
    } else {
        if show_diff {
            let temp = mktemp::Temp::new_file()?;
            std::fs::write(&temp, actual)?;
            let diff = Command::new("diff")
                .arg("-u")
                .arg(expected.as_ref())
                .arg(temp.as_path())
                .output()?;
            if !diff.stderr.is_empty() {
                bail!(String::from_utf8(diff.stderr)?);
            }
            println!(
                "Actual does not meet expected:\n{}",
                String::from_utf8(diff.stdout)?
            );
            println!("\nRe-run with {ENV_VAR}=1 to update reference files");
        }
        Ok(false)
    }
}

pub fn check_results<F: Fn(), P: AsRef<Path>>(
    register: F,
    expected: P,
    main_only: bool,
) -> eyre::Result<bool> {
    let actual = super::run::run_tests(register, None, None, false, main_only)?;
    let update = std::env::var(ENV_VAR).is_ok();
    if update {
        if !matches!(equal_content(&actual, &expected, false), Ok(true)) {
            std::fs::write(expected, actual)?;
        }
        Ok(true)
    } else {
        equal_content(&actual, &expected, !update)
    }
}
