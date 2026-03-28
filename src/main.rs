mod disk;
mod environment;
mod helpers;
mod net;
mod processor;
mod ram;
mod swap;

use owo_colors::OwoColorize;
use std::{
    io::{self},
    thread, time,
};

fn main() -> io::Result<()> {
    let mut s = sysinfo::System::new();
    let mut n = sysinfo::Networks::new_with_refreshed_list();
    let mut d = sysinfo::Disks::new();
    helpers::refresh(&mut s, &mut n, &mut d);

    println!(
        "{}\n{}",
        "Getting data...".magenta(),
        "Ctrl + C to exit".green()
    );

    loop {
        let mut buf = String::new();
        thread::sleep(time::Duration::from_secs(1));
        helpers::refresh(&mut s, &mut n, &mut d);

        let ram_date = ram::format_data(&s);
        let swap_data = swap::format_data(&s);

        let cpu_model = processor::format_model_data(&s)?;
        let cpu_frequency = processor::format_frequency(&s)?;
        let cpu_global_usage = processor::format_global_usage(&s);
        
        buf += &format!("{}\n", environment::format_os_info());

        buf += &format!("{}\n", "\n[ Memory ]".bold().blue());
        buf += &format!("{}\n", ram_date).as_str();
        buf += &format!("{}\n", swap_data).as_str();

        buf += &format!("{}\n", "\n[ Net ]".bold().blue());
        for (name, data) in &n {
            buf += &format!("{}\n", net::format_interface_data(name, data))
        }

        buf += &format!("{}\n", "\n[ Disk ]".bold().blue());
        for disk in &d {
            buf += &format!("{}\n", disk::format_disk_data(disk))
        }

        buf += &format!("\n{}\n", "[ CPU ]".bold().blue());
        buf += &format!("{}\n", cpu_model);
        buf += &format!("{}\n", cpu_frequency);
        buf += &format!("{}\n", cpu_global_usage);

        buf += &format!("{}:", "Usage per core".yellow());
        let mut i = 0;
        for cpu in s.cpus() {
            if i % 4 == 0 {
                buf += "\n"
            } else {
                buf += "\t"
            }

            buf += &format!("{}", processor::format_core_usage(i, cpu));
            i += 1;
        }

        helpers::clear();
        println!("{buf}")
    }
}
