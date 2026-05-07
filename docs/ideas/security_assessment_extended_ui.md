# Extended UI: Security Assessment Modules

## Overview
The Security Assessment modules (Security Analysis, Subdomain Takeover, Cloudflare Bypass, Nmap Zero-Day, API Security, Geo Analysis) in `web-analyzer` are highly sophisticated. However, WebQ's frontend is only displaying high-level summaries and missing critical vulnerability metadata.

## Missing Data Visualizations

### 1. Nmap Zero-Day (`nmap_zero_day.rs`)
*   **Exploit-DB Linkage**: The `VulnerabilityInfo` struct maps CVEs directly to Exploit-DB IDs and Metasploit module paths. The WebQ UI only shows the CVE ID and CVSS score. We need actionable exploit links.
*   **Raw Script Output**: Nmap NSE (Nmap Scripting Engine) raw outputs are parsed by the backend but dropped by the UI.

### 2. Security Analysis (`security_analysis.rs`)
*   **Granular Header Analysis**: `HeaderAnalysis` returns detailed flags on `X-Frame-Options`, `X-Content-Type-Options`, and `Referrer-Policy`. The UI currently aggregates these into a single "Headers" grade. We need a strict pass/fail toggle list for every individual security header.
*   **Cookie Security Result**: `CookieSecurityResult` lists exact cookies missing `HttpOnly` or `Secure` flags. The UI only provides a binary secure/insecure status.

### 3. Subdomain Takeover (`subdomain_takeover.rs`)
*   **Remediation Steps**: The `TakeoverVulnerability` struct returns specific, provider-dependent remediation steps (e.g., "Remove S3 bucket reference" vs "Unlink GitHub Pages custom domain"). The UI only shows the vulnerability name.

### 4. Geo Analysis (`geo_analysis.rs`)
*   **ASN Details**: IP addresses are mapped to their specific Autonomous System Numbers (ASN) and ISPs. This telemetry is currently missing from the Geo dashboard.

## Implementation Request
Build modal overlays or expandable accordion tables in the `/src/lib/components/assessment/` UI to allow analysts to drill down from the high-level grades into the raw, granular vulnerability intelligence provided by the backend.
