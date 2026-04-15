# Implementation Plan: WebQ Security Posture Progress Integration

This plan outlines the systematic modifications required to retrofit the `security_analysis` module to emit chunk-based stream updates rather than blocking execution until completion.

## 1. Context & Rationale
Currently, Phase 4 tools like the Security Analyzer parse all Headers, WAF components, SSL parameters, and CORS configurations procedurally. The end-user is left staring at a blank loading screen. We must propagate the `ScanProgress` MPSC channel through the backend API.

## 2. Proposed Backend Changes (`web-analyzer`)
- **File**: `src/security_analysis.rs`
- **Actions**:
  - Augment `analyze_security(..)` to receive an optional Sender (`Option<Sender<ScanProgress>>`).
  - Isolate blocks spanning: WAF Fingerprinting, Context Security Policies (CSP), Cookies verification, and Cross-Origin Resource checking.
  - Implement dynamic `.send().await` operations relaying the granular module step context.

## 3. Tauri Middleware Layer (`WebQ`)
- **File**: `src-tauri/src/lib.rs` -> macro block calling `scan_security_analysis`.
- **Actions**:
  - Spawn `tokio::sync::mpsc::channel`.
  - Handle asynchronous cross-channel events mapped directly to Svelte App Emitter `scan-progress`.

## 4. Subsystem Wiring (Svelte 5 UI)
- **File**: `src/routes/assessment/security-posture/+page.svelte`
- **Actions**:
  - Destructure existing API invocation arrays and implement `onMount` or internal `listen` hooks to register state mutations globally.
  - Attach `<ScanTerminal />` to the DOM logic, triggering dynamic visibility based strictly upon `appState.isScanning`.
