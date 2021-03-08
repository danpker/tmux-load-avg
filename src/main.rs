use sysinfo::{LoadAvg, RefreshKind, System, SystemExt};

fn main() {
    let mut system: System = SystemExt::new_with_specifics(RefreshKind::new().with_cpu());
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

    match load {
        n if n >= number_of_cpus => format!("#[fg=red]{}", parsed),
        n if n >= number_of_cpus * 0.5 => format!("#[fg=yellow]{}", parsed),
        _ => format!("#[fg=white]{}", parsed),
    }
}
