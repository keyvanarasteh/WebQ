# Web Technologies Module

## Overview
The `Web Technologies` module utilizes the `web-analyzer` crate's Wappalyzer-like fingerprinting capabilities. It actively scans target endpoints to detect underlying Content Management Systems (CMS), JavaScript frameworks, server languages, and specifically performs deep introspection on WordPress installations.

## Implementation Status
**Status:** ✅ Complete

The backend implementation successfully exposes the `scan_web_technologies` command and feeds the data to a robust set of Svelte UI components under the `/recon/web-technologies` route.

## Backend Integration
The `scan_web_technologies` command triggers the `detect_web_technologies` function from `web-analyzer`.
- Performs HTTP Header introspection (e.g., `X-Powered-By`, `Server`, `X-Generator`).
- Scans HTML source code for known meta tags, framework specific IDs (`__next`, `Nuxt`), and script signatures.
- Executes targeted WordPress enumerations (wp-json user exposure, theme/plugin detection paths).

## Frontend UI Components
The module utilizes the following UI components located in `/src/lib/components/recon/web-technologies/`:

*   **`TechStackGrid.svelte`**: A visually dense grid that categorizes detected technologies (e.g., Frontend Frameworks, Web Servers, Programming Languages, CDNs) using identifiable Lucide icons.
*   **`WordPressScanner.svelte`**: A dedicated component that renders active WordPress vulnerabilities, exposed users, active themes, and plugins if the target is identified as a WP instance.
*   **`SecurityHeadersList.svelte`**: Parses the HTTP response headers gathered during the fingerprinting process to detect missing baseline security controls (HSTS, X-Content-Type-Options).

## Data Flow
1.  A target URL is inputted in the `/recon/web-technologies` view.
2.  Tauri fires the `scan_web_technologies` command, instructing the backend to parse the HTTP responses.
3.  The results are matched against regex signatures and returned as a serialized `WebTechResult`.
4.  The Svelte frontend loops through the detected stacks, rendering categorized badges and conditionally mounting the `WordPressScanner` if a WP payload is detected.
