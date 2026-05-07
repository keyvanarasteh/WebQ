# React2Shell Scanner & Honeypot Module

## Overview
The `React2Shell` module is a dual-purpose engine designed to evaluate and mitigate the CVE-2025-55182 vulnerability affecting React Server Components (RSC) and Next.js platforms. It acts as both a vulnerability scanner and an active honeypot that logs, profiles, and simulates realistic responses to attackers.

## Implementation Status
**Status:** ⚠️ Incomplete (Backend Complete, Frontend Partial)

## Backend Integration
The Tauri backend integrates several commands from `web-analyzer`'s `react` and `honeypot` modules:
- `scan_react2shell`, `scan_react_source_leak`, `scan_react_rce_command`, `scan_react_rce_full`: Probes target domains for vulnerability markers.
- `start_honeypot`, `stop_honeypot`, `get_honeypot_status`, `get_top_attackers`, `test_payload_locally`: Manages the honeypot engine, evaluates incoming requests against a matrix of 40+ signatures, and calculates attacker risk scores.

## Frontend UI Components
The module has basic dashboard scaffolding in `/src/routes/react/` but lacks detailed analytical views.

### Current Implementation
*   **`Dashboard Overview`** (`/react/honeypot/+page.svelte`): Initial integration of honeypot controls.

### Missing Implementation (Needs Work)
*   **Attacker Profiling**: Missing UI to display detailed `AttackerProfile` metrics.
*   **Signature Matrix Documentation**: Missing the interface to document and educate defenders on the 40+ regex signatures.

> [!NOTE]
> A feature request has been opened to track the completion of this module. See the [React2Shell UI Feature Request](./ideas/react2shell_ui.md).
