use clap::{Parser, Subcommand};
use std::process::Command;
use colored::*;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "adb")]
#[command(about = "Fast Rust ADB CLI wrapper", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List connected devices
    Devices,
    /// Get device info (fast)
    Info {
        #[arg(short, long)]
        device: Option<String>,
    },
    /// List installed packages
    Packages {
        #[arg(short, long)]
        user_only: bool,
    },
    /// Install APK
    Install {
        apk: String,
        #[arg(short, long)]
        replace: bool,
    },
    /// Uninstall package
    Uninstall {
        package: String,
    },
    /// Get app permissions
    Perms {
        package: String,
    },
    /// Execute shell command
    Shell {
        #[arg(trailing_var_arg = true)]
        cmd: Vec<String>,
    },
    /// Get screen size & DPI
    Display,
    /// Monkey test (random taps)
    Monkey {
        package: String,
        #[arg(default_value = "100")]
        events: u32,
    },
    /// List files on device
    Ls {
        path: String,
    },
}

fn adb(args: &[&str]) -> Result<String> {
    let output = Command::new("adb")
        .args(args)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Devices => {
            let out = adb(&["devices"])?;
            println!("{}", out);
        }
        Commands::Info { device } => {
            let mut cmd = vec!["shell", "getprop"];
            let d = device.unwrap_or_default();
            if !d.is_empty() {
                cmd = vec!["-s", &d, "shell", "getprop"];
            }
            
            let model = adb(&["shell", "getprop", "ro.product.model"])?;
            let android = adb(&["shell", "getprop", "ro.build.version.release"])?;
            let cpu = adb(&["shell", "getprop", "ro.product.cpu.abi"])?;
            
            println!("{}", "Device Info".bold());
            println!("  Model:   {}", model.blue());
            println!("  Android: {}", android.green());
            println!("  CPU:     {}", cpu.cyan());
        }
        Commands::Packages { user_only } => {
            let flag = if user_only { "-3" } else { "" };
            let out = adb(&["shell", "pm", "list", "packages", flag])?;
            let count = out.lines().count();
            println!("{} packages found:", count);
            for pkg in out.lines().take(20) {
                println!("  {}", pkg);
            }
            if count > 20 {
                println!("  ... and {} more", count - 20);
            }
        }
        Commands::Install { apk, replace } => {
            let mut cmd = vec!["install"];
            if replace {
                cmd.push("-r");
            }
            cmd.push(&apk);
            let out = adb(&cmd)?;
            if out.contains("Success") {
                println!("{}", "✓ Installed".green());
            } else {
                println!("{}", "✗ Failed".red());
            }
        }
        Commands::Uninstall { package } => {
            let out = adb(&["uninstall", &package])?;
            if out.contains("Success") {
                println!("{}", "✓ Uninstalled".green());
            } else {
                println!("{}", "✗ Failed".red());
            }
        }
        Commands::Perms { package } => {
            let out = adb(&["shell", "dumpsys", "package", &package])?;
            println!("Granted permissions:");
            for line in out.lines() {
                if line.contains("granted=true") {
                    println!("  {}", line.trim().green());
                }
            }
        }
        Commands::Display => {
            let size = adb(&["shell", "wm", "size"])?;
            let dpi = adb(&["shell", "wm", "density"])?;
            println!("Display: {}", size.cyan());
            println!("DPI:     {}", dpi.cyan());
        }
        Commands::Shell { cmd } => {
            if cmd.is_empty() {
                eprintln!("No command specified");
                return Ok(());
            }
            let out = adb(&cmd.iter().map(|s| s.as_str()).collect::<Vec<_>>())?;
            println!("{}", out);
        }
        Commands::Monkey { package, events } => {
            println!("Running {} events on {}...", events.bold(), package.yellow());
            let out = adb(&["shell", "monkey", "-p", &package, &events.to_string()])?;
            println!("{}", out);
        }
        Commands::Ls { path } => {
            let out = adb(&["shell", "ls", "-lah", &path])?;
            println!("{}", out);
        }
    }
    Ok(())
}
