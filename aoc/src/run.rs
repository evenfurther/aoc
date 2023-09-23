use chrono::{Datelike, Duration};
use clap::Parser;
use itertools::Itertools;

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
    /// Show timing information
    timing: bool,

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

pub fn run<F>(register: F) -> eyre::Result<()>
where
    F: Fn(),
{
    color_eyre::install().unwrap();
    let opts = Opts::parse();
    if opts.day.is_some() && opts.all {
        eyre::bail!("--all and --day are not compatible");
    }
    if opts.input.is_some() && opts.all {
        eyre::bail!("--all and --input are not compatible");
    }
    unsafe {
        super::input::OVERRIDE_INPUT = opts.input;
    }
    let current_day = opts.day.unwrap_or(chrono::Utc::now().day() as usize);
    register();
    let mut runners = super::runners::RUNNERS.lock().unwrap();
    let keys = runners.keys().copied().collect::<Vec<_>>();
    for (day, part) in keys {
        if day == current_day || opts.all {
            for (version, runner) in runners.remove(&(day, part)).unwrap() {
                let before = chrono::Utc::now();
                let result = runner();
                let after = chrono::Utc::now();
                let version = version
                    .clone()
                    .map_or_else(String::new, |v| format!(" — {}", v));
                let elapsed = if opts.timing {
                    format!(" ({})", pretty_duration(after - before))
                } else {
                    String::new()
                };
                let header = format!("Day {day} - part {part}{version}: ");
                let sep = format!("\n{}", " ".repeat(header.len()));
                let result = match result {
                    Ok(e) => e.lines().join(&sep),
                    Err(e) => format!("<error: {e:?}>"),
                };
                println!("{header}{result}{elapsed}");
            }
        }
    }
    Ok(())
}
