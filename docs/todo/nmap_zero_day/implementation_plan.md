# Implementation Plan: Nmap Zero-Day Tracker

## 1. Rationale 
The Nmap port scanner fundamentally lacks ETA timing due to standard command binary outputs. However, emitting standard context ticks upon initialization and parsing stages provides the user with adequate process mapping feedback.

## 2. Rust Backend Hooks
- Implement `progress_tx` channel passing throughout `nmap_zero_day.rs`.
- Identify sequential steps: Bin Path resolution -> Command Execution Wait -> XML Schema parse mapping -> NVD Extractor.
- Output detailed message payloads indicating version matching matrices within the database.

## 3. UI Synchronization
- Drop `<ScanTerminal />` UI abstraction alongside reactive array bindings in `nmap-zero-day/+page.svelte`.
- Maintain synchronization of states via Tauri global listeners.
