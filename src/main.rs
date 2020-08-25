use std::cmp::Ordering;
use sysinfo::{LoadAvg, SystemExt};

use colored::*;

fn main() {
    let mut system = sysinfo::System::new();
    system.refresh_cpu();

    let number_of_cpus = system.get_processors().len() as f64;
    let LoadAvg { one, five, fifteen } = system.get_load_average();

    println!(
        "{} {} {}",
        format_load(one, number_of_cpus),
        format_load(five, number_of_cpus),
        format_load(fifteen, number_of_cpus)
    );
}

fn format_load(load: f64, number_of_cpus: f64) -> String {
    let parsed = format!("{:.2}", load);

    match load
        .partial_cmp(&number_of_cpus)
        .expect("Couldn't find a way to compare values.")
    {
        Ordering::Greater => parsed.red().to_string(),
        Ordering::Equal | Ordering::Less => parsed,
    }
}
