use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(name = "adb", about = "Fast ADB CLI", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List connected devices
    Devices,
    /// Get device info
    Info {
        /// Optional device serial
        #[arg(short)]
        device: Option<String>,
    },
    /// List installed packages
    Packages {
        /// Only user apps
        #[arg(short, long)]
        user: bool,
    },
    /// Clear app data
    Clear {
        /// Package name
        package: String,
    },
    /// Pull file from device
    Pull {
        /// Source path on device
        source: String,
        /// Destination on PC
        dest: String,
    },
}

fn run_adb(args: &[&str]) -> String {
    let output = Command::new("adb")
        .args(args)
        .output()
        .expect("Failed to run adb");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn main() {
    let args = Args::parse();
    
    match args.command {
        Commands::Devices => {
            println!("{}", run_adb(&["devices"]));
        }
        Commands::Info { device } => {
            let mut adb_args = vec!["shell", "getprop"];
            if let Some(d) = device {
                println!("Device: {}", d);
            }
            let out = run_adb(&["shell", "getprop"]);
            println!("{}", out);
        }
        Commands::Packages { user } => {
            let flag = if user { "-3" } else { "" };
            let out = run_adb(&["shell", "pm", "list", "packages", flag]);
            println!("{}", out);
        }
        Commands::Clear { package } => {
            let result = run_adb(&["shell", "pm", "clear", &package]);
            println!("Clear result: {}", result);
        }
        Commands::Pull { source, dest } => {
            let result = run_adb(&["pull", &source, &dest]);
            println!("Pull result: {}", result);
        }
    }
}
