# Cloudflare Bypass Module

## Overview
The `Cloudflare Bypass` module attempts to reveal the true origin IP address of a target domain hiding behind a Web Application Firewall (WAF) or Reverse Proxy like Cloudflare.

## Implementation Status
**Status:** ✅ Complete

## Backend Integration
The `scan_cloudflare_bypass` command invokes `find_real_ip`.
- Queries historical DNS databases (e.g., SecurityTrails, DNSDumpster).
- Performs SSL certificate parsing (Censys, crt.sh) to find IPs presenting the target's certificate.
- Scans generic subdomains (ftp, mail, direct) that often bypass the proxy.

## Frontend UI Components
Located in `/src/lib/components/assessment/cloudflare-bypass/`:
*   **`BypassGrid.svelte`**: A dynamic table that lists potential origin IPs, the methodology used to find them (e.g., "Historical DNS", "SSL Cert Map"), and their current active status.
*   **`BypassGridGuide.svelte`**: An interactive guide explaining how WAF bypass techniques work.
