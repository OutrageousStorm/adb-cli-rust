# ⚡ adb-cli-rust

Ultra-fast Rust CLI for ADB — instant device info, package listing, app control.

## Build
```bash
cargo build --release
./target/release/adb --help
```

## Commands
- `adb devices` — list connected devices
- `adb info [--device <serial>]` — model, Android version, CPU
- `adb packages [--user-only]` — list installed packages
- `adb install <apk> [-r]` — install APK (replace with -r)
- `adb uninstall <package>` — remove app
- `adb perms <package>` — list granted permissions
- `adb display` — screen size and DPI
- `adb monkey <package> [events]` — run monkey test
- `adb ls <path>` — list files on device
- `adb shell <cmd...>` — run arbitrary shell command

## Examples
```bash
adb devices
adb info
adb packages --user-only
adb install app.apk -r
adb perms com.facebook.katana
adb shell pm list packages | wc -l
```

Faster than stock `adb` for common operations.
