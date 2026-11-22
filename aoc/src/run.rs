use clap::Parser;
use std::fmt::Write;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

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
    if duration < Duration::from_micros(1) {
        format!("{} ns", duration.as_nanos())
    } else if duration < Duration::from_millis(1) {
        format!("{:.2} µs", duration.as_nanos() as f32 / 1000.0)
    } else if duration < Duration::from_secs(1) {
        format!("{:.2} ms", duration.as_micros() as f32 / 1000.0)
    } else {
        format!("{:.2} s", duration.as_millis() as f32 / 1000.0)
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
                let before = Instant::now();
                let outcome = call_runner(runner);
                let elapsed = before.elapsed();
                let result = match outcome {
                    Ok(result) => {
                        if timings {
                            write!(&mut results, " ({})", pretty_duration(elapsed))?;
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
fn puzzle_day(seconds_since_epoch: u64) -> usize {
    const EST_OFFSET: u64 = 5 * 3600;
    let days = seconds_since_epoch.saturating_sub(EST_OFFSET) / 86_400;
    day_of_month(days)
}

/// Day of month for a number of days since 1970-01-01, using Howard
/// Hinnant's `civil_from_days` algorithm (proleptic Gregorian calendar).
#[allow(clippy::cast_possible_truncation)]
fn day_of_month(days_since_epoch: u64) -> usize {
    let z = days_since_epoch + 719_468;
    let doe = z % 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    (doy - (153 * mp + 2) / 5 + 1) as usize
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
    let current_day = opts.day.unwrap_or_else(|| {
        puzzle_day(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(0, |d| d.as_secs()),
        )
    });
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
    use super::{day_of_month, pretty_duration, puzzle_day};
    use std::time::Duration;

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
        // 2026-12-05T04:59:00Z is still Dec 4th in EST.
        assert_eq!(puzzle_day(1_796_446_740), 4);
        // 2026-12-05T05:00:00Z is midnight EST: day 5 is out.
        assert_eq!(puzzle_day(1_796_446_800), 5);
    }

    #[test]
    fn day_of_month_matches_calendar() {
        assert_eq!(day_of_month(0), 1); // 1970-01-01
        assert_eq!(day_of_month(59), 1); // 1970-03-01
        assert_eq!(day_of_month(11_016), 29); // 2000-02-29
        assert_eq!(day_of_month(20_818), 31); // 2026-12-31
    }

    #[test]
    fn pretty_durations() {
        assert_eq!(pretty_duration(Duration::from_nanos(800)), "800 ns");
        assert_eq!(pretty_duration(Duration::from_nanos(1_500)), "1.50 µs");
        assert_eq!(pretty_duration(Duration::from_micros(2_250)), "2.25 ms");
        assert_eq!(pretty_duration(Duration::from_millis(3_500)), "3.50 s");
    }
}
