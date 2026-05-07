# Feature Request: Configurable Nmap Scanning Profiles

## Overview
The `nmap_zero_day.rs` module beautifully integrates Nmap output with the NVD API and Exploit-DB searches. However, the command arguments are hardcoded directly into the `tokio::process::Command::new("nmap")` execution block:
`-sV -Pn -A -T5 --top-ports 1000 -oG - <ip>`.

This hardcoding prevents analysts from using the engine for stealthy operations or exhaustive sweeps.

## Missing Engine Controls & Visualization
1. **Nmap Profile Selector**: The UI needs a dropdown offering different scan intensities:
   *   **Fast / Loud (Default)**: `-T5 --top-ports 1000`
   *   **Stealth**: `-T2 -f -sS`
   *   **Exhaustive**: `-p- -T4`
   *   **Vuln Scripts**: `--script vuln`
2. **Custom Args Input**: A text box allowing power users to append specific Nmap flags before the command is dispatched to the backend.
3. **CPE Extraction View**: The backend data structure has a `cpe` (Common Platform Enumeration) vector in `PortInfo` which is currently empty because grepable output doesn't supply it. Changing the backend to use XML output (`-oX`) and parsing it would allow the UI to show exact CPE strings for Exploit-DB matching.

## Implementation Request
1. Refactor the `run_nmap_scan` function in `web-analyzer` to accept an `NmapConfig` struct containing args.
2. Build an "Advanced Options" drawer in the WebQ Security Assessment UI to expose these configurable parameters.
