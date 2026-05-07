# Security Posture Analysis Module

## Overview
The `Security Analysis` module acts as a primary Defensive SecOps dashboard. It executes the `security-analysis` capabilities of `web-analyzer`, offering a deep dive into header security, SSL robustness, WAF detection, and CORS configurations.

## Implementation Status
**Status:** ✅ Complete

## Backend Integration
The `scan_security_posture` command wraps `web_analyzer::security_analysis::analyze_security`.
- Performs WAF detection heuristics (e.g., identifying Cloudflare, AWS WAF, Akamai).
- Analyzes CORS policies for excessive permissiveness (`*`, `null` origin).
- Tests supported HTTP methods (detecting unsafe methods like PUT, TRACE).
- Emits real-time progress via Tauri MPSC channels.

## Frontend UI Components
This module features an extensive set of components located in `/src/lib/components/assessment/security-posture/`:
*   **`OverallSecurityGrade.svelte`**: Calculates and displays an overarching letter grade (A+ to F) based on the combined security findings.
*   **`WafBypassStatus.svelte`**: Highlights detected WAFs and evaluates baseline bypass potential.
*   **`CorsCookieAnalysis.svelte`**: Audits CORS headers and Cookie security flags (HttpOnly, Secure, SameSite).
*   **`HeadersAnalysis.svelte` / `SslAnalysisGrade.svelte`**: Deep inspections of HSTS, CSP, and TLS configurations.
*   **`VulnScanLog.svelte`**: A rolling log of the live scanning execution.
*   **Interactive Guides**: Nearly every component has an accompanying `*Guide.svelte` to teach SecOps principles.
