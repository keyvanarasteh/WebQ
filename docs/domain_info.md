# Domain Info Module

## Overview
The `Domain Info` module provides exhaustive domain reconnaissance. It integrates the `web-analyzer`'s `domain-info` feature into WebQ, enabling users to perform deep dives into a target domain's infrastructure, SSL configuration, open ports, WHOIS registry data, and HTTP header security posture.

## Implementation Status
**Status:** ✅ Complete

All aspects of the `domain-info` crate are successfully integrated into the Tauri backend and have a dedicated, high-fidelity Svelte 5 frontend component architecture.

## Backend Integration
The Tauri backend exposes the following IPC commands wrapped around the `web-analyzer`'s asynchronous core:

*   `scan_domain_info`: Triggers the overarching domain scan.
*   `scan_ip_resolution`: Resolves IPv4/IPv6 addresses and performs reverse DNS lookups.
*   `scan_whois`: Retrieves domain registry and registrar details.
*   `scan_ssl`: Validates SSL certificate chains, expiration, and issuer data.
*   `scan_ports`: Utilizes Rust's concurrent sockets to scan common ports (e.g., 21, 22, 80, 443, 3306).
*   `scan_http_status`: Captures server response times, active server daemons, and HTTP response codes.
*   `scan_security_headers`: Analyzes CORS, CSP, HSTS, and X-Frame-Options headers.

## Frontend UI Components
The frontend is implemented in `/src/routes/intelligence/domain-info/` and `/src/lib/components/intelligence/domain-info/` using Svelte 5 Runes.

*   **`DomainOverview.svelte`**: Central dashboard for IP, HTTP, and WHOIS summaries.
*   **`SslStatus.svelte`**: Visualizes the certificate chain and expiration metrics.
*   **`SecurityDetails.svelte`**: Displays a security matrix grading the server's HTTP security headers.
*   **`PortSecurityMatrix.svelte`**: Dynamically maps resolved IPs to open ports and displays potential vulnerability footprints.

## Data Flow
1.  User enters target domain in the UI.
2.  `+page.svelte` invokes the decoupled Tauri commands in parallel.
3.  Tauri asynchronously executes the Rust routines without blocking the main thread.
4.  Data is fed back to the Svelte components which reactively update using `$state` and `$derived` variables.
