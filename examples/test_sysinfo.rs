// Test sysinfo capabilities for temperature and frequency
use sysinfo::{System, Components, RefreshKind, CpuRefreshKind};

fn main() {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything())
    );
    sys.refresh_cpu_all();

    println!("=== CPU Frequency ===");
    for (i, cpu) in sys.cpus().iter().enumerate() {
        println!("CPU {}: {} MHz ({:.2} GHz)", i, cpu.frequency(), cpu.frequency() as f64 / 1000.0);
    }

    println!("\n=== Components (Temperature) ===");
    let mut components = Components::new_with_refreshed_list();
    println!("Number of components: {}", components.len());

    for (i, comp) in components.iter().enumerate() {
        let temp = comp.temperature();
        let max = comp.max();
        println!("Component {}: {} - Temp: {:?}°C (max: {:?}°C)",
            i, comp.label(), temp, max);
    }
}
