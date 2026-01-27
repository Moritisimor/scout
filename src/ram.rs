use owo_colors::OwoColorize;
use sysinfo::System;

pub fn format_data(s: &System) -> String {
    format!(
        "{}: {:.2} {} / {:.2} {}",
        "RAM".yellow(),
        (s.free_memory() as f64 / 1000000000.).green(),
        "GB".purple(),
        (s.total_memory() as f64 / 1000000000.).green(),
        "GB".purple()
    )
}