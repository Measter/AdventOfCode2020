use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use color_eyre::eyre::{eyre, Context, Result};

pub mod nom;

type PartFunction<Input, Output> = dyn Fn(Input) -> Result<Output>;

pub fn run<Input, Output>(
    name: &str,
    input: Input,
    parts: &[&PartFunction<Input, Output>],
) -> Result<()>
where
    Output: Display,
    Input: Copy,
{
    println!("{}\n", name);

    for (part, i) in parts.iter().zip(1..) {
        println!("-- Part {} --", i);

        let part_result = part(input).with_context(|| eyre!("Error running Part {}", i))?;

        println!("Result: {}", part_result);

        // Run a few times to get an estimate of how long it takes.
        let mut min_run = Duration::from_secs(u64::MAX);

        for _ in 0..5 {
            let now = Instant::now();
            let _ = part(input);
            let time = now.elapsed();

            if time < min_run {
                min_run = time;
            }
        }

        let total_runs = (10.0 / min_run.as_secs_f64())
            .ceil()
            .max(10.0)
            .min(u32::MAX as f64) as u32; // I doubt we'll actually *do* 4.2 billion runs...

        let mut total_time = Duration::default();
        let mut min_run = Duration::from_secs(u64::MAX);
        let mut max_run = Duration::default();

        for _ in 0..total_runs {
            let start = Instant::now();
            let _ = part(input); // We'll just discard the result as we handled errors above.
            let elapsed = start.elapsed();

            total_time += start.elapsed();
            if elapsed < min_run {
                min_run = elapsed;
            }

            if elapsed > max_run {
                max_run = elapsed;
            }
        }

        let mean_run = total_time / total_runs;

        let min_prec = if min_run.as_nanos() < 1000 { 0 } else { 3 };
        let mean_prec = if mean_run.as_nanos() < 1000 { 0 } else { 3 };
        let max_prec = if max_run.as_nanos() < 1000 { 0 } else { 3 };

        println!(
            "Times for {} runs: [{:.min_prec$?} .. {:.mean_prec$?} .. {:.max_prec$?}]",
            human_format::Formatter::new().format(total_runs as f64),
            min_run,
            mean_run,
            max_run,
            min_prec = min_prec,
            mean_prec = mean_prec,
            max_prec = max_prec
        );

        println!();
    }

    Ok(())
}
