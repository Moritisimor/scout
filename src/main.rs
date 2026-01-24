use owo_colors::OwoColorize;
use std::{
    io::{self, Write},
    thread, time,
};

fn make_error(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, msg)
}

fn main() -> io::Result<()> {
    let mut s = sysinfo::System::new();
    let mut n = sysinfo::Networks::new_with_refreshed_list();
    s.refresh_all();
    println!(
        "{}\n{}",
        "Getting data...".magenta(),
        "Ctrl + C to exit".green()
    );

    loop {
        thread::sleep(time::Duration::from_secs(1));
        println!("\x1b[H\x1b[2J\x1b[3J{}", "[ System ]".bold().blue());
        println!(
            "{}: {}\n{}: {}",
            "OS".yellow(),
            sysinfo::System::name().unwrap_or("Unknown".into()).green(),
            "Architecture".yellow(),
            sysinfo::System::cpu_arch().green()
        );

        println!("{}", "\n[ Memory ]".bold().blue());
        println!(
            "{}: {:.2} {} / {:.2} {}",
            "RAM".yellow(),
            (s.free_memory() as f64 / 1000000000.).green(),
            "GB".purple(),
            (s.total_memory() as f64 / 1000000000.).green(),
            "GB".purple()
        );

        println!(
            "{}: {:.2} {}",
            "Swap".yellow(),
            (s.total_swap() as f64 / 1000000000.).green(),
            "GB".purple()
        );

        println!("{}", "\n[ Net ]".bold().blue());
        for (name, data) in &n {
            println!(
                "{}: {:.2} {} ↑ / {:.2} {} ↓",
                name.yellow(),
                (data.transmitted() as f64 / 1000000.).green(),
                "MB".purple(),
                (data.received() as f64 / 1000000.).green(),
                "MB".purple()
            )
        }

        println!("\n{}", "[ CPU ]".bold().blue());
        println!(
            "{}: {}",
            "Model".yellow(),
            s.cpus()
                .get(0)
                .ok_or(make_error("Could not read CPU model"))?
                .brand()
                .green()
        );
        
        println!(
            "{}: {} {}",
            "Frequency".yellow(),
            (s.cpus()
                .get(0)
                .ok_or(make_error("Could not read CPU frequency"))?
                .frequency() as f64
                / 1000.)
                .green(),
            "Ghz".purple()
        );

        println!(
            "{}: {:.2} {}",
            "Total Usage".yellow(),
            s.global_cpu_usage().green(),
            "%".purple()
        );

        print!("{}:", "Usage per core".yellow());
        let mut i = 0;
        for cpu in s.cpus() {
            if i % 2 == 0 {
                println!("")
            } else {
                print!("\t")
            }

            print!(
                "{}: {:.2} {}",
                i.magenta(),
                cpu.cpu_usage().green(),
                "%".purple()
            );
            i += 1;
        }

        s.refresh_cpu_usage();
        s.refresh_cpu_frequency();
        s.refresh_memory();
        n.refresh(true);
        io::stdout().flush()?
    }
}
