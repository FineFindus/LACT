use daemon::daemon_connection::DaemonConnection;
use structopt::StructOpt;
use colored::*;

#[derive(StructOpt)]
#[structopt(rename_all = "lower")]
enum Opt {
    ///Realtime GPU information
    Stats {
        gpu_id: Option<u32>,
    },
    ///Get GPU list
    Gpus,
    ///General information about the GPU
    Info {
        gpu_id: Option<u32>,
    },
}

fn main() {
    env_logger::init();
    
    let opt = Opt::from_args();

    let d = DaemonConnection::new().unwrap();
    log::trace!("connection established");

    match opt {
        Opt::Gpus => {
            let gpus = d.get_gpus();
            println!("{:?}", gpus);
        },
        Opt::Stats { gpu_id } => {
            let mut gpu_ids: Vec<u32> = Vec::new();

            if let Some(gpu_id) = gpu_id {
                gpu_ids.push(gpu_id);
            }
            else {
                for (gpu_id, _) in d.get_gpus().unwrap() {
                    gpu_ids.push(gpu_id);
                }
            }

            for gpu_id in gpu_ids {
                print_stats(&d, gpu_id);
            }
        },
        Opt::Info { gpu_id } => {
            let mut gpu_ids: Vec<u32> = Vec::new();

            if let Some(gpu_id) = gpu_id {
                gpu_ids.push(gpu_id);
            }
            else {
                for (gpu_id, _) in d.get_gpus().unwrap() {
                    gpu_ids.push(gpu_id);
                }
            }

            for gpu_id in gpu_ids {
                print_info(&d, gpu_id);
            }
        },
    }
}

fn print_info(d: &DaemonConnection, gpu_id: u32) {
    let gpu_info = d.get_gpu_info(gpu_id).unwrap();
    println!("{} {}", "GPU Model:".blue(), gpu_info.card_model.bold());
    println!("{} {}", "GPU Vendor:".blue(), gpu_info.gpu_vendor.bold());
    println!("{} {}", "Driver in use:".blue(), gpu_info.driver.bold());
    println!("{} {}", "VBIOS Version:".blue(), gpu_info.vbios_version.bold());
    println!("{} {}", "VRAM Size:".blue(), gpu_info.vram_size.to_string().bold());
    println!("{} {}", "Link Speed:".blue(), gpu_info.link_speed.bold());
}

fn print_stats(d: &DaemonConnection, gpu_id: u32) {
    let gpu_stats = d.get_gpu_stats(gpu_id).unwrap();
    println!("{} {}/{} MiB", "VRAM Usage:".green(), gpu_stats.mem_used, gpu_stats.mem_total);
    println!("{} {}°C", "Temperature:".green(), gpu_stats.gpu_temp);
    println!("{} {}/{}RPM", "Fan Speed:".green(), gpu_stats.fan_speed, gpu_stats.max_fan_speed);
    println!("{} {}MHz", "GPU Clock:".green(), gpu_stats.gpu_freq);
    println!("{} {}V", "GPU Voltage:".green(), gpu_stats.voltage as f64 / 1000.0);
    println!("{} {}MHz", "VRAM Clock:".green(), gpu_stats.mem_freq);
    println!("{} {}/{}W", "Power Usage:".green(), gpu_stats.power_avg, gpu_stats.power_cap);
}