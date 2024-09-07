use clap::{Parser, Subcommand};
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use nvml_wrapper::Nvml;
use std::process::exit;
use sysinfo::{CpuRefreshKind, RefreshKind};
use systemstat::{Memory, Platform};

#[derive(Subcommand, Debug)]
enum Command {
    Memory,
    Mounts {
        #[arg(required = true)]
        paths: Vec<String>,
    },
    NvidiaGpu,
    Cpu,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn mount_usage(sys: &systemstat::System, mount_path: &str) -> f32 {
    let mount = sys.mount_at(mount_path).unwrap_or_else(|e| {
        eprintln!("ERROR={e}: Failed to get mount information at {mount_path}");
        exit(1);
    });

    (1.0 - (mount.free.as_u64() as f32 / mount.total.as_u64() as f32)) * 100.0
}

fn memory_usage(mem: &Memory) -> f32 {
    (1.0 - (mem.free.as_u64() as f32 / mem.total.as_u64() as f32)) * 100.0
}

fn bytes_to_gib(bytes: u64) -> f32 {
    (bytes as f32) / (1024.0 * 1024.0 * 1024.0)
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Memory => {
            let sys = systemstat::System::new();
            let mem = sys.memory().unwrap();

            println!(
                "MEM: {:.1}%\n{:.1}/{:.1} GiB",
                memory_usage(&mem),
                bytes_to_gib(mem.total.as_u64() - mem.free.as_u64()),
                bytes_to_gib(mem.total.as_u64()),
            );
        }
        Command::Mounts { paths } => {
            let sys = systemstat::System::new();

            for path in paths {
                println!("{path}={:.1}%", mount_usage(&sys, &path));
            }
        }
        Command::NvidiaGpu => {
            let nvml = Nvml::init().unwrap_or_else(|e| {
                eprintln!("ERROR={e}: NVML failed to initialize");
                exit(1);
            });
            let device = nvml.device_by_index(0).unwrap_or_else(|e| {
                eprintln!("ERROR={e}: Failed to retrieve GPU at index 0");
                exit(1);
            });
            let mem_info = device.memory_info().unwrap();

            println!(
                "GPU: {:.1}% {}° {:.0}W\n{:.0}% {:.1}/{:.1} GiB",
                device.utilization_rates().unwrap().gpu,
                device.temperature(TemperatureSensor::Gpu).unwrap(),
                device.power_usage().unwrap() as f32 / 1000.0,
                mem_info.used as f32 / mem_info.total as f32 * 100.0,
                bytes_to_gib(mem_info.used),
                bytes_to_gib(mem_info.total),
            );
        }
        Command::Cpu => {
            let mut sys = sysinfo::System::new_with_specifics(
                RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
            );
            std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
            sys.refresh_cpu_usage();

            let sys2 = systemstat::System::new();

            let load = sys2.load_average().unwrap_or_else(|e| {
                eprintln!("ERROR={e}: Failed to retrieve load averages");
                exit(1);
            });

            println!(
                "CPU: {:.0}% {:.0}°\n{:.1} {:.1} {:.1}",
                sys.global_cpu_usage(),
                sys2.cpu_temp().unwrap_or_else(|e| {
                    eprintln!("ERROR={e}: Failed to retrieve CPU temperatures");
                    exit(1);
                }),
                load.one,
                load.five,
                load.fifteen,
            );
        }
    }
}
