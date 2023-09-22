use std::error::Error;
use std::process;

use clap::Parser;

use image_dataset_analyzer::{get_dataset_description, DatasetDescription};
use image_dataset_analyzer::config::Args;
use image_dataset_analyzer::benchmark;


fn main() {
    let args = Args::parse();

    let res;
    let duration;
    if args.timeit == true {
        (res, duration) = benchmark::timeit(|| run(args.clone()));
        println!("Execution duration {} seconds", duration.as_secs_f32());
        println!("Speed: {} (images / second)", (res.as_ref().unwrap().size as f32) / (duration.as_secs_f32()));
    } else {
        res = run(args);
    }

    if let Err(e) = res {
        println!("Application error: {e}");
        process::exit(1);
    }
}


fn run(args: Args) -> Result<DatasetDescription, Box<dyn Error>> {
    let root_dir = args.root_dir;
    let trackit = args.trackit;
    let dataset_description = get_dataset_description(root_dir, trackit)?;

    println!("Dataset description {dataset_description}");

    return Ok(dataset_description);
}
