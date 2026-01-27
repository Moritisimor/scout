use std::io;

use owo_colors::OwoColorize;
use sysinfo::{Cpu, System};

use crate::helpers;

pub fn format_model_data(s: &System) -> io::Result<String> {
    Ok(format!(
        "{}: {}",
        "Model".yellow(),
        s.cpus()
            .get(0)
            .ok_or(helpers::make_err("Could not read CPU model"))?
            .brand()
            .green()
    ))
}

pub fn format_frequency(s: &System) -> io::Result<String> {
    Ok(format!(
        "{}: {} {}",
        "Frequency".yellow(),
        (s.cpus()
            .get(0)
            .ok_or(helpers::make_err("Could not read CPU frequency"))?
            .frequency() as f64 / 1000.)
            .green(),

        "Ghz".purple()
    ))
}

pub fn format_global_usage(s: &System) -> String {
    format!(
        "{}: {:.2} {}",
        "Total Usage".yellow(),
        s.global_cpu_usage().green(),
        "%".purple()
    )
}

pub fn format_core_usage(i: i32, cpu: &Cpu) -> String {
    format!(
        "{}: {:.2} {}",
        i.magenta(),
        cpu.cpu_usage().green(),
        "%".purple()
    )
}
