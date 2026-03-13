mod disk;
mod environment;
mod helpers;
mod net;
mod processor;
mod ram;
mod swap;

use owo_colors::OwoColorize;
use std::{
    io::{self, Write},
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
        thread::sleep(time::Duration::from_secs(1));
        helpers::refresh(&mut s, &mut n, &mut d);

        let ram_date = ram::format_data(&s);
        let swap_data = swap::format_data(&s);

        let cpu_model = processor::format_model_data(&s)?;
        let cpu_frequency = processor::format_frequency(&s)?;
        let cpu_global_usage = processor::format_global_usage(&s);
        
        helpers::clear();
        println!("{}", environment::format_os_info());

        println!("{}", "\n[ Memory ]".bold().blue());
        println!("{}", ram_date);
        println!("{}", swap_data);

        println!("{}", "\n[ Net ]".bold().blue());
        for (name, data) in &n {
            println!("{}", net::format_interface_data(name, data))
        }

        println!("{}", "\n[ Disk ]".bold().blue());
        for disk in &d {
            println!("{}", disk::format_disk_data(disk))
        }

        println!("\n{}", "[ CPU ]".bold().blue());
        println!("{}", cpu_model);
        println!("{}", cpu_frequency);
        println!("{}", cpu_global_usage);

        print!("{}:", "Usage per core".yellow());
        let mut i = 0;
        for cpu in s.cpus() {
            if i % 4 == 0 {
                println!()
            } else {
                print!("\t")
            }

            print!("{}", processor::format_core_usage(i, cpu));
            i += 1;
        }

        io::stdout().flush()?
    }
}
