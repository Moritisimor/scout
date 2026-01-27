use std::io;

use sysinfo::{Networks, System};

pub fn make_err(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, msg)
}

pub fn clear() {
    println!("\x1b[H\x1b[2J\x1b[3J")
}

pub fn refresh(s: &mut System, n: &mut Networks) {
    s.refresh_cpu_usage();
    s.refresh_cpu_frequency();
    s.refresh_memory();
    n.refresh(true);
}
