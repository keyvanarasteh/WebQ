# MacOS M-Series (Apple Silicon) Build Checklist for WebQ

This document covers the complete workflow for compiling and signing the native macOS release of WebQ (Tauri v2) on an Apple Silicon Mac Mini.

## 1. System Requirements & Prerequisites
- [ ] Install Xcode Command Line Tools: `xcode-select --install`
- [ ] Install Homebrew: `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`
- [ ] Install Rust for Apple Silicon: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- [ ] Install Bun (Package Manager): `curl -fsSL https://bun.sh/install | bash`
- [ ] Check external scanning binaries directly: WebQ expects Nmap to potentially be in `/opt/homebrew/bin/nmap` instead of generic `$PATH` sometimes. Make sure `openssl`, `nmap`, and `dig` exist.

## 2. Project Initialization
- [ ] Clone repository: `git clone <repo_url> && cd WebQ`
- [ ] Install JS dependencies using Bun: `bun install`
- [ ] Verify compilation: `bun run dev` (Ensure backend connects over the internal SQLite interface flawlessly).

## 3. Apple Developer & Security Certificates (Signing)
*Note: Since WebQ performs extensive port scanning and ICMP pings (which sometimes require elevated networking capabilities depending on the scanner implementation), entitlements and codesigning are highly sensitive.*

- [ ] Obtain an **Apple Developer Program** license.
- [ ] Add the Developer ID Application Certificate into your macOS Keychain via Xcode.
- [ ] Configure `tauri.conf.json` -> `bundle.macOS`:
  - Define exact entitlements in `entitlements.mac.plist` allowing network client boundaries to prevent the macOS Application Firewall from blocking internal WebAnalyzer requests.
- [ ] Setup Apple Notarization credentials in env variables (for `notarytool`):
  - `APPLE_ID="..."`
  - `APPLE_PASSWORD="..."` (App-Specific Password)
  - `APPLE_TEAM_ID="..."`

## 4. Compiling & Production Builds
- [ ] Build Target: `aarch64-apple-darwin` for M-series execution efficiency.
  `rustup target add aarch64-apple-darwin`
- [ ] (Optional) Universal Binary for seamless Intel fallback distribution:
  `bun run tauri build --target universal-apple-darwin`
- [ ] Standard Native Build:
  `bun run tauri build`
  
## 5. Artifact Verification
- [ ] The generated artifacts will exist under: `src-tauri/target/release/bundle/macos/` and `src-tauri/target/release/bundle/dmg/`
- [ ] Verify that within the generated App sandbox, the local SQLite database is efficiently placed inside `~/Library/Application Support/WebQ/` or standard config directories.

## 6. Known MacOS Specific Edge Cases
- Network Firewall constraints: MacOS may aggressively pop up "Do you want the application WebQ.app to accept incoming network connections?" The user must click "Allow" to prevent deep network scans from being dropped.
- Dependency paths: MacOS binaries usually sit in `/usr/bin/` or `/opt/homebrew/bin/`. Ensure the system dependency engine handles paths correctly instead of panicking.
