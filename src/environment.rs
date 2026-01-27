use owo_colors::OwoColorize;

pub fn format_os_info() -> String {
    format!(
        "{}: {}\n{}: {}\n{}: {}",
        "OS".yellow(),
        sysinfo::System::name().unwrap_or("Unknown".into()).green(),
        "Architecture".yellow(),
        sysinfo::System::cpu_arch().green(),
        "Kernel".yellow(),
        sysinfo::System::kernel_long_version().green()
    )
}