use owo_colors::OwoColorize;
use sysinfo::Disk;

pub fn format_disk_data(d: &Disk) -> String {
    format!(
        "{} {:.2} {} / {:.2} {}",
        d.name().display().yellow(),
        ((d.total_space() - d.available_space()) as f64 / 100000000.).green(),
        "GB".purple(),
        (d.total_space() as f64 / 1000000000.).green(),
        "GB".purple()
    )
}
