use chrono::{Datelike, Duration};
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
) -> eyre::Result<String>
where
    F: Fn(),
{
    register();
    let mut results = String::new();
    let mut runners = super::runners::RUNNERS.lock().unwrap();
    let keys = runners.keys().copied().collect::<Vec<_>>();
    for (day, part) in keys {
        if single_part.is_some_and(|p| p != part) {
            continue;
        }
        if single_day.map_or(true, |d| d == day) {
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
                let result = runner()?;
                let after = chrono::Utc::now();
                if timings {
                    write!(&mut results, " ({})", pretty_duration(after - before))?;
                }
                write!(&mut results, ": ")?;
                let sep = format!("\n{}", " ".repeat(results.chars().count() - results_start));
                write!(&mut results, "{}", result.trim().replace('\n', &sep))?;
                writeln!(&mut results)?;
            }
        }
    }
    Ok(results)
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
    unsafe {
        super::input::OVERRIDE_INPUT = opts.input;
    }
    let current_day = opts.day.unwrap_or(chrono::Utc::now().day() as usize);
    println!(
        "{}",
        run_tests(
            register,
            (!opts.all).then_some(current_day),
            opts.part,
            opts.main_only,
            opts.timing
        )?
    );
    Ok(())
}
