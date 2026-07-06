# Windows OS Build Checklist for WebQ

This document outlines the workflow for compiling and distributing the native Windows (.msi / .nsis) release of WebQ (Tauri v2).

## 1. System Requirements & Prerequisites
- [ ] Install **Visual Studio 2022 C++ Build Tools**: Make sure to check the "Desktop development with C++" workload (required for compiling the Rust backend).
- [ ] Install Rust for Windows targeting `x86_64-pc-windows-msvc`: Download from `https://rustup.rs/`.
- [ ] Install Bun (Package Manager) via PowerShell: `powershell -c "irm bun.sh/install.ps1 | iex"`
- [ ] Install **Npcap / Nmap for Windows**: Native network scanning on Windows requires Npcap hooks into the Windows kernel to bypass typical networking constraints.

## 2. Project Initialization
- [ ] Clone repository: `git clone <repo_url> && cd WebQ`
- [ ] Install JS dependencies using Bun: `bun install`
- [ ] Verify local connectivity with the SQLite WAL architecture before final production build passes: `bun run dev`.

## 3. Tauri Bundle Tools Configuration
- [ ] Download and install the **WiX Toolset v3** or configure to natively use **NSIS** packaging. (NSIS is heavily recommended over MSI for Tauri v2).

## 4. Signing (Authenticode EV)
*Note: Since WebQ is a "Cybersecurity Analysis Tool" engaging in high-velocity TCP/UDP scanning, unsigned binaries will instantly trigger intense **Microsoft Defender SmartScreen** / Anti-Virus exclusions acting as False Positives.*

- [ ] Acquire an EV or Standard Code Signing Certificate (.pfx / .p12).
- [ ] Configure `signtool` executable path or Azure specific remote sign capabilities.
- [ ] Ideally, explicitly handle AppLocker environments by maintaining pristine metadata on the `.exe`.

## 5. Compiling & Production Builds
- [ ] Since WebQ leverages high-concurrency network tasks, compile in fully optimized release mode:
  `bun run tauri build`
  
## 6. Artifact Verification
- [ ] The generated installers will exist under: 
  - `src-tauri/target/release/bundle/msi/`
  - `src-tauri/target/release/bundle/nsis/`

## 7. Known Windows Specific Edge Cases
- **Strict Network Stacks:** Windows Firewall will immediately interrupt internal Zero-day probes and Nmap integrations natively. The setup wizard (NSIS) could proactively run `netsh` rules setting WebQ into the trusted program allowance list on execution, preventing silent application timeouts.
- **SQLite Concurrency:** Ensure the Windows filesystem perfectly tolerates WAL concurrency mechanisms when hundreds of event logs pour through `tracing` to the DB storage, as SMB or Anti-Virus real-time scannings frequently lock `.db-wal` files on Windows leading to "Resource temporarily unavailable" timeouts.
