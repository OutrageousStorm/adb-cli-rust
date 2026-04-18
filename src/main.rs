use std::process::Command;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "devices" => cmd_devices(),
        "info" => cmd_info(),
        "install" => cmd_install(&args),
        "uninstall" => cmd_uninstall(&args),
        "push" => cmd_push(&args),
        "pull" => cmd_pull(&args),
        "shell" => cmd_shell(&args),
        "forward" => cmd_forward(&args),
        "reboot" => cmd_reboot(),
        "--help" | "-h" | "help" => print_help(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
        }
    }
}

fn adb(args: &[&str]) -> String {
    let output = Command::new("adb")
        .args(args)
        .output()
        .expect("Failed to execute adb");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn cmd_devices() {
    println!("📱 Connected devices:");
    let output = adb(&["devices", "-l"]);
    for line in output.lines().skip(1) {
        if !line.is_empty() {
            println!("  {}", line);
        }
    }
}

fn cmd_info() {
    println!("📊 Device info:");
    let props = [
        ("ro.product.model", "Model"),
        ("ro.build.version.release", "Android"),
        ("ro.build.version.sdk", "API"),
        ("ro.product.cpu.abi", "Architecture"),
    ];
    for (key, label) in props {
        let val = adb(&["shell", "getprop", key]);
        println!("  {}: {}", label, val.trim());
    }
}

fn cmd_install(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: adb-cli install <file.apk>");
        return;
    }
    println!("📲 Installing {}...", args[2]);
    let output = adb(&["install", "-r", &args[2]]);
    println!("{}", output);
}

fn cmd_uninstall(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: adb-cli uninstall <package.name>");
        return;
    }
    println!("🗑️  Uninstalling {}...", args[2]);
    let output = adb(&["uninstall", &args[2]]);
    println!("{}", output);
}

fn cmd_push(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Usage: adb-cli push <local> <remote>");
        return;
    }
    println!("📤 Pushing {} → {}", args[2], args[3]);
    adb(&["push", &args[2], &args[3]]);
}

fn cmd_pull(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Usage: adb-cli pull <remote> <local>");
        return;
    }
    println!("📥 Pulling {} → {}", args[2], args[3]);
    adb(&["pull", &args[2], &args[3]]);
}

fn cmd_shell(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: adb-cli shell <command>");
        return;
    }
    let cmd = &args[2..].join(" ");
    let output = adb(&["shell", cmd]);
    println!("{}", output);
}

fn cmd_forward(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Usage: adb-cli forward <local_port> <remote_port>");
        return;
    }
    println!("🔀 Forwarding tcp:{} → tcp:{}", args[2], args[3]);
    adb(&["forward", &format!("tcp:{}", args[2]), &format!("tcp:{}", args[3])]);
}

fn cmd_reboot() {
    println!("🔄 Rebooting device...");
    adb(&["reboot"]);
}

fn print_help() {
    println!("\n🤖 adb-cli — Simple ADB command wrapper in Rust\n");
    println!("USAGE:");
    println!("  adb-cli <COMMAND> [OPTIONS]\n");
    println!("COMMANDS:");
    println!("  devices              List all connected devices");
    println!("  info                 Show device properties");
    println!("  install <file.apk>   Install APK on device");
    println!("  uninstall <pkg>      Uninstall package");
    println!("  push <local> <remote> Push file to device");
    println!("  pull <remote> <local> Pull file from device");
    println!("  shell <cmd>          Execute shell command");
    println!("  forward <local> <remote> Forward port");
    println!("  reboot               Reboot device");
    println!("  help                 Show this help\n");
}
