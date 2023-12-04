# Helper crate for Advent Of Code

This is an helper crate to help you solve [https://adventofcode.com/](Advent Of Code) problems in Rust. It contains utilities to run various repetitive tasks, such as input parsing, benchmarking, or testing.

## Installation

The installation involves a few steps that need to be done once.

### Add dependencies

Add the following dependencies to your Rust project's `Cargo.toml`:

```toml
[dependencies]
aoc = { git = "https://github.com/evenfurther/aoc" }

[build-dependencies]
aoc-build = { git = "https://github.com/evenfurther/aoc" }
```

### Create a build script

In the same directory as your `Cargo.toml` file, create a `build.rs` file containing:

```rust
fn main() {
    aoc_build::build().expect("Build error");
}
```

### Create the main program

Create `src/main.rs`, which takes care of running all solutions, or a selection of solutions, depending on the command line arguments:

```rust
fn main() -> eyre::Result<()> {
    aoc::run(aoc2023::register::register_runners)
}
```

### Create the main library file

Create `src/lib.rs`, which will include `register.rs` built by the build script:

```rust
#[macro_use]
extern crate aoc;

pub mod register {
    include!(concat!(env!("OUT_DIR"), "/register.rs"));
}
```

## Solving a day

Let's implement the solution for day 1 of the current year.

### Retrieve your input

Retrieve your input, and place it into `input/day1.txt`. This is where the runner will look for it.

### Add a new module

At the end of `src/lib.rs`, add a new module `day1`:

```rust
pub mod day1;
```

Create a new `src/day1.rs` file, in which you will place your solution.

### Write the code for part 1 of day 1

Add the solution for part 1 of day 1, using the `#aoc` attribute to indicate that you are doing so. Behind the scenes, it will register the corresponding runner:

```rust
#[aoc(day1, part1)]
fn part1(input: &str) -> usize {
   input.len()
}
```

In our case, it assumes that the solution to the first problem is to give the length of the input. You can now run it:

```bash
$ cargo run
Day 1 - part 1: 42
```

That's it. By default, it runs the solution for the current day. If you are working on the day 1 solution at a later time, you can specify the day to run:

```bash
$ cargo run -- --day 1
Day 1 - part 1: 42
```

The `--` is necessary to distinguish between arguments to `cargo run` and arguments to your program.

Of course, you might want to add `--release` if your solution takes time to compute:

```bash
$ cargo run --release -- --day 1
Day 1 - part 1: 42
```

Note that `--release` must appear before `--`.

## Bells and whistles

### Testing

Once you have implemented your solution, you can add the expected outcome to a `expected.txt` file. This file will then be compared to the real execution when running `cargo test` (or `cargo test --release` if your algorithms take time).

When you add a new solution, the `expected.txt` file will be out-of-date. You can update it during `cargo test` by setting the `RECORD_RESULTS` environment variable to 1:

```bash
$ cargo test
running 1 test
test check_expected ... FAILED

failures:

---- check_expected stdout ----
Actual does not meet expected:
--- expected.txt	2023-12-04 11:32:54.051878291 +0100
+++ /tmp/5b5ff539bb01470198c092bad5f444ab	2023-12-04 11:32:56.410914647 +0100
@@ -0,0 +1,10 @@
+Day 1 - part 1: 232

$ RECORD_RESULTS=1 cargo test
running 1 test
test check_expected ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test
running 1 test
test check_expected ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

You can then commit `expected.txt` to your version control system, and run the tests in your continuous integration framework if you use one.

### Output

In our example, `part1()` returns a `usize`. You can return any type implementing the [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) trait. You can even return a [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) as long as the error variant implements the [`Error`](https://doc.rust-lang.org/std/error/trait.Error.html) trait:

Let us assume that we have added the [`eyre`](https://crates.io/crates/eyre) crate to our project:

```rust
#[aoc(day1, part1)]
fn part1(input: &str) -> eyre::Result<u32> {
    Ok(input.parse()? * 2)   // Return twice the number contained in the input
}
```

The runner will automatically extract the result if everything goes well, or print the error otherwise.

### Input file

By default, programs for day N will receive input retrieved from `input/dayN.txt`. You can choose another input using the `--input` command line argument.

For example, if you have stored the example given in the problem text into `input/sample1.txt`, you can run:

```bash
$ cargo run -- --input input/sample1.txt
Day 1 - Part 1: 17
```

### Input format

In our example, our solver receives the input as a `&str`, that is a single string containing the content of the input file `input/day1.txt`. However, this crate is able to split the input into lines automatically if the signature of your solver requires it:

```rust
#[aoc(day1, part1)]
fn part1(input: &[&str]) -> usize {
    input.len()   // Returns the number of lines
}
```

Also, you can ask for any type implementing the [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) trait. For example, let's assume we have an input file with two integers separated by "/" on every line:

```rust
struct Game {
    left: u32,
    right: u32
}

impl FromStr for Game {
    type Error = eyre::Report;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let Some((l, r)) = line.split_once('/')
          else { eyre::bail!("unable to parse {}", line) };
        Ok(Game {
            left: l.parse()?,
            right: r.parse()?,
        })
    }
}

#[aoc(day1, part1)]
fn part1(games: &[Game]) -> u32 {
    // Return the sum of product of left and right for each game
    games.iter().map(|g| g.left * g.right).sum()
}
```

You don't have to take care of parsing the various `Game` structures found on every line, this framework will parse them for you.

In addition, inputs can be given as `String`, `Vec<_>` instead of slices, and so on. You can even mark them as mutable. For example, if you intend to work on the input strings and mangle them, you can do something like:

```rust
#[aoc(day1, part1)]
fn part1(mut input: Vec<String>) -> usize {
    // Add a dummy last line, as directed in the instructions
    input.push(String::from("Hello, world"));
    // Add a 'x' at the end of every line
    input.iter_mut().for_each(|l| l.push('x'));
    // Do something
    input.into_iter().map(|s| s.len()).sum()
}
```

You get the idea.

### Benchmarks

You can get (very) basic timing information by using the `--timing` flag on the command line:

```bash
$ cargo run --release -- --timing --day 1
Day 1 - part 1 (23.96 µs): 42
```

### Alternatives

You might want to implement alternative ways of implementing a part, as shown in [`dummy-year/src/day1.rs`](dummy-year/src/day1.rs):

```rust
#[aoc(day1, part1)]
fn part1(input: &str) -> usize {
    2 * bytecount::count(input.as_bytes(), b'(') - input.trim().len()
}

#[aoc(day1, part1, str_slice)]
fn part1_string_slice(input: &[&str]) -> usize {
    input.iter().copied().map(part1).sum()
}
```

This will show up in runs:

```bash
$ cargo run
Day 1 - part 1: 232
Day 1 - part 1 — str_slice: 232
```

You can use `--main-only` if you do not want to see the alternatives.

### Getting help

You can get help by using `--help` (don't forget the `--`, or you'll get `cargo run`'s help message):

```bash
$ cargo run -- --help
Advent of Code

Usage: dummy-year [OPTIONS]

Options:
  -a, --all            Run all days
  -d, --day <DAY>      Use a specific day
  -p, --part <PART>    Restrict running to one part (1 or 2)
  -t, --timing         Show timing information
  -m, --main-only      Skip running any alternate version
  -i, --input <INPUT>  Use alternate input (file or string)
  -h, --help           Print help
  -V, --version        Print version
```

## Contributing

You are welcome to contribute by sending bug reports and pull requests directed to [the repository](https://github.com/evenfurther/aoc).
