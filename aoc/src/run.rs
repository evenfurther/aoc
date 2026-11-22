use chrono::{Datelike, Duration, FixedOffset, Utc};
use clap::Parser;
use std::fmt::Write;

#[derive(Parser)]
#[clap(version, author)]
/// Advent of Code
struct Opts {
    #[clap(short, long)]
    /// Run all days
    all: bool,

    #[clap(short, long)]
    /// Use a specific day
    day: Option<usize>,

    #[clap(short, long)]
    /// Restrict running to one part (1 or 2)
    part: Option<usize>,

    #[clap(short, long)]
    /// Show timing information
    timing: bool,

    #[clap(short, long)]
    /// Skip running any alternate version
    main_only: bool,

    #[clap(short, long)]
    /// Use alternate input (file or string)
    input: Option<String>,
}

#[allow(clippy::cast_precision_loss)]
fn pretty_duration(duration: Duration) -> String {
    if duration < Duration::microseconds(1) {
        format!("{} ns", duration.num_nanoseconds().unwrap())
    } else if duration < Duration::milliseconds(1) {
        format!(
            "{:.2} µs",
            duration.num_nanoseconds().unwrap() as f32 / 1000.0
        )
    } else if duration < Duration::seconds(1) {
        format!(
            "{:.2} ms",
            duration.num_microseconds().unwrap() as f32 / 1000.0
        )
    } else {
        format!("{:.2} s", duration.num_milliseconds() as f32 / 1000.0)
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn run_tests<F>(
    register: F,
    single_day: Option<usize>,
    single_part: Option<usize>,
    main_only: bool,
    timings: bool,
) -> eyre::Result<(String, usize)>
where
    F: Fn(),
{
    register();
    let mut results = String::new();
    let mut failures = 0;
    let mut runners = super::runners::RUNNERS.lock().unwrap();
    let keys = runners.keys().copied().collect::<Vec<_>>();
    for (day, part) in keys {
        if single_part.is_some_and(|p| p != part) {
            continue;
        }
        if single_day.is_none_or(|d| d == day) {
            for (version, runner) in runners.remove(&(day, part)).unwrap() {
                if main_only && version.is_some() {
                    continue;
                }
                let results_start = results.chars().count();
                write!(&mut results, "Day {day} - part {part}")?;
                if let Some(version) = version {
                    write!(&mut results, " — {version}")?;
                }
                let before = chrono::Utc::now();
                let outcome = call_runner(runner);
                let after = chrono::Utc::now();
                let result = match outcome {
                    Ok(result) => {
                        if timings {
                            write!(&mut results, " ({})", pretty_duration(after - before))?;
                        }
                        result
                    }
                    Err(e) => {
                        failures += 1;
                        format!("ERROR: {e}")
                    }
                };
                write!(&mut results, ": ")?;
                let sep = format!("\n{}", " ".repeat(results.chars().count() - results_start));
                write!(&mut results, "{}", result.trim().replace('\n', &sep))?;
                writeln!(&mut results)?;
            }
        }
    }
    Ok((results, failures))
}

/// Run a solution, turning both errors and panics into a printable message
/// so that one failing day does not prevent the others from running.
fn call_runner(runner: Box<super::runners::Runner>) -> Result<String, String> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(runner)) {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(e)) => Err(format!("{e:#}")),
        Err(payload) => Err(payload
            .downcast_ref::<&str>()
            .map(|s| (*s).to_owned())
            .or_else(|| payload.downcast_ref::<String>().cloned())
            .map_or_else(|| String::from("panic"), |m| format!("panic: {m}"))),
    }
}

/// Puzzles are released at midnight EST (UTC-5), so the "current" puzzle day
/// must be computed in that timezone rather than in UTC or local time.
fn puzzle_day(now: chrono::DateTime<Utc>) -> usize {
    let est = FixedOffset::west_opt(5 * 3600).unwrap();
    now.with_timezone(&est).day() as usize
}

pub fn run<F>(register: F) -> eyre::Result<()>
where
    F: Fn(),
{
    color_eyre::install()?;
    let opts = Opts::parse();
    if opts.day.is_some() && opts.all {
        eyre::bail!("--all and --day are not compatible");
    }
    if opts.input.is_some() && opts.all {
        eyre::bail!("--all and --input are not compatible");
    }
    if opts.part.is_some_and(|p| !(1..=2).contains(&p)) {
        eyre::bail!("--part accepts argument must be 1 or 2");
    }
    if let Some(input) = opts.input {
        super::input::OVERRIDE_INPUT.set(input).unwrap();
    }
    let current_day = opts.day.unwrap_or_else(|| puzzle_day(Utc::now()));
    let (results, failures) = run_tests(
        register,
        (!opts.all).then_some(current_day),
        opts.part,
        opts.main_only,
        opts.timing,
    )?;
    println!("{results}");
    if failures > 0 {
        eyre::bail!("{failures} solution(s) failed");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::puzzle_day;
    use chrono::{TimeZone, Utc};

    #[test]
    fn failures_do_not_stop_other_days() {
        fn register() {
            crate::register_runner(20, 1, None, || -> eyre::Result<u32> { Ok(1) });
            crate::register_runner(20, 2, None, || -> eyre::Result<u32> { eyre::bail!("boom") });
            crate::register_runner(21, 1, None, || -> eyre::Result<u32> { panic!("kaboom") });
            crate::register_runner(21, 2, None, || -> eyre::Result<u32> { Ok(4) });
        }
        let (results, failures) = super::run_tests(register, None, None, false, false).unwrap();
        assert_eq!(failures, 2);
        assert_eq!(
            results,
            "Day 20 - part 1: 1\nDay 20 - part 2: ERROR: boom\n\
             Day 21 - part 1: ERROR: panic: kaboom\nDay 21 - part 2: 4\n"
        );
    }

    #[test]
    fn puzzle_day_follows_est() {
        // 04:59 UTC on Dec 5th is still Dec 4th in EST.
        let t = Utc.with_ymd_and_hms(2026, 12, 5, 4, 59, 0).unwrap();
        assert_eq!(puzzle_day(t), 4);
        // 05:00 UTC on Dec 5th is midnight EST: day 5 is out.
        let t = Utc.with_ymd_and_hms(2026, 12, 5, 5, 0, 0).unwrap();
        assert_eq!(puzzle_day(t), 5);
    }
}
