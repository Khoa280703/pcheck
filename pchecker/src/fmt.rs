// Output formatting module

use std::time::Instant;

const SEPARATOR: &str = "============================================================";

pub fn print_header(version: &str) {
    println!("{}", SEPARATOR);
    println!("🤖 PCHECKER {} - Hardware Info Tool", version);
    println!("{}", SEPARATOR);
    println!();
}

pub fn print_section(icon: &str, label: &str, value: &str) {
    // Label padded to 12 chars, then 5 spaces separator
    println!("{} {:<12}{}", icon, label, value);
}

pub fn print_footer(start_time: Instant) {
    let elapsed = start_time.elapsed();
    let time_str = if elapsed.as_secs() == 0 {
        format!("{:.2}s", elapsed.as_secs_f64())
    } else {
        format!("{}s", elapsed.as_secs())
    };

    println!();
    println!("{}", SEPARATOR);
    println!("Done in {}", time_str);
    println!("{}", SEPARATOR);
}
