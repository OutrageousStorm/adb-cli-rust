use std::process::Command;
use std::env;

fn adb(args: &[&str]) -> String {
    let output = Command::new("adb")
        .args(args)
        .output()
        .expect("Failed to run adb");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "devices" => {
            let output = adb(&["devices"]);
            println!("📱 Connected devices:");
            for line in output.lines().skip(1) {
                if !line.is_empty() {
                    println!("  {}", line);
                }
            }
        }
        "info" => {
            println!("Device Info:");
            let model = adb(&["shell", "getprop", "ro.product.model"]);
            let android = adb(&["shell", "getprop", "ro.build.version.release"]);
            let serial = adb(&["get-serialno"]);
            println!("  Model: {}", model);
            println!("  Android: {}", android);
            println!("  Serial: {}", serial);
        }
        "packages" => {
            let filter = args.get(2).map(|s| s.as_str()).unwrap_or("");
            let output = adb(&["shell", "pm", "list", "packages"]);
            println!("Installed packages:");
            for line in output.lines() {
                let pkg = line.replace("package:", "");
                if filter.is_empty() || pkg.contains(filter) {
                    println!("  {}", pkg);
                }
            }
        }
        "reboot" => {
            adb(&["reboot"]);
            println!("Rebooting...");
        }
        "shell" => {
            if args.len() < 3 {
                eprintln!("Usage: adb-cli shell <command>");
                return;
            }
            let cmd = &args[2..].join(" ");
            let output = adb(&["shell", cmd]);
            println!("{}", output);
        }
        _ => print_help(),
    }
}

fn print_help() {
    println!("
adb-cli-rust v1.0 - Fast ADB CLI wrapper

Commands:
  devices         List connected devices
  info            Show device info (model, Android version, serial)
  packages [filter]  List installed packages (optional filter)
  reboot          Reboot device
  shell <cmd>     Run shell command

Examples:
  adb-cli devices
  adb-cli info
  adb-cli packages com.facebook
  adb-cli shell getprop ro.build.fingerprint
");
}
