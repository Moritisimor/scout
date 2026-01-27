use owo_colors::OwoColorize;

pub fn format_os_info() -> String {
    format!(
        "{} {}\n{}",
        sysinfo::System::name()
            .unwrap_or("Unknown".into())
            .bold()
            .blue(),
        sysinfo::System::cpu_arch().bold().blue(),
        sysinfo::System::kernel_long_version().bold().blue()
    )
}
