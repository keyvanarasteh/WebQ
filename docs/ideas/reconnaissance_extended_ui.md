# Extended UI: Reconnaissance Modules

## Overview
A deep dive into the `web-analyzer` source code for Reconnaissance modules (Subdomain Discovery, Contact Spy, Advanced Content Scanner) reveals several advanced data points and heuristics that are not currently surfaced in the WebQ frontend.

## Missing Data Visualizations

### 1. Contact Spy (`contact_spy.rs`)
The backend spider extracts deep context alongside raw contacts:
*   **Source Page Context**: The UI shows extracted emails, but `web-analyzer` tracks the exact URL and line number where the email was found. A "Provenance" column is needed in the UI.
*   **Extended Social Networks**: The backend regexes cover smaller/niche networks (GitHub, GitLab, Discord invites, Telegram groups) which aren't categorized distinctly in the `SocialOsintBox` component.

### 2. Subdomain Discovery (`subdomain_discovery.rs`)
*   **Detection Methodology**: The `SubdomainDetail` struct returns whether a subdomain was found via "Bruteforce", "Certificate Transparency", or "Scraping". The UI currently only shows the final resolution IP, omitting the OSINT methodology used to find it.

### 3. Advanced Content Scanner (`advanced_content_scanner.rs`)
*   **Source Code Leak Context**: When API keys or secrets are found, the backend returns the surrounding code snippet (context window) for validation. The WebQ UI currently only shows "Secret Found" without the surrounding text, forcing the analyst to manually verify the leak.

## Implementation Request
Update the Svelte components in `/src/lib/components/recon/` to consume the full data structs. Specifically, add provenance tracking to Contacts and Context Windows to the Content Scanner.
