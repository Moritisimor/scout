use owo_colors::OwoColorize;
use sysinfo::NetworkData;

pub fn format_interface_data(name: &String, data: &NetworkData) -> String {
    format!(
        "{}: {:.2} {} ↑ / {:.2} {} ↓",
        name.yellow(),
        (data.transmitted() as f64 / 1000000.).green(),
        "MB".purple(),
        (data.received() as f64 / 1000000.).green(),
        "MB".purple()
    )
}
