use std::time::Duration;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

fn main() {
    let mut sys = System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
    std::thread::sleep(Duration::from_millis(1000));
    sys.refresh_cpu_usage();

    print!("{}%", sys.global_cpu_usage());
}
