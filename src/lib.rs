use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use color_eyre::eyre::{eyre, Context, Result};

pub mod nom;

type PartFunction<Input, Output> = dyn Fn(&Input) -> Result<Output>;

pub fn run<Input, Output>(input: Input, parts: &[&PartFunction<Input, Output>]) -> Result<()>
where
    Output: Display,
{
    for (part, i) in parts.iter().zip(1..) {
        println!("-- Part {} --", i);

        let now = Instant::now();
        let part_result = part(&input).with_context(|| eyre!("Error running Part {}", i))?;
        let time = now.elapsed();

        println!("Result: {}", part_result);

        let total_runs = (1.0 / time.as_secs_f64())
            .ceil()
            .max(10.0)
            .min(u32::MAX as f64) as u32; // I doubt we'll actually *do* 4.2 billion runs...

        let mut total_time = Duration::default();
        let mut min_run = Duration::from_secs(u64::MAX);
        let mut max_run = Duration::default();

        for _ in 0..total_runs {
            let start = Instant::now();
            let _ = part(&input); // We'll just discard the result as we handled errors above.
            let elapsed = start.elapsed();

            total_time += start.elapsed();
            if elapsed < min_run {
                min_run = elapsed;
            }

            if elapsed > max_run {
                max_run = elapsed;
            }
        }

        println!(
            "Time for {} runs: [{:?} .. {:?} .. {:?}]",
            human_format::Formatter::new().format(total_runs as f64),
            min_run,
            total_time / total_runs,
            max_run
        );

        println!();
    }

    Ok(())
}
