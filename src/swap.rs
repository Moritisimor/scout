use owo_colors::OwoColorize;
use sysinfo::System;

pub fn format_data(s: &System) -> String {
    format!(
        "{}: {:.2} {}",
        "Swap".yellow(),
        (s.total_swap() as f64 / 1000000000.).green(),
        "GB".purple()
    )
}