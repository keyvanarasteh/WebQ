# Feature Request: Comprehensive DNS Investigation Dashboard

## Overview
The `domain_dns.rs` module utilizes `tokio::join!` to parallelize `dig` queries, instantly resolving A, AAAA, MX, NS, SOA, TXT, and CNAME records simultaneously. 

While the backend gathers this massive breadth of DNS data, the UI currently lacks a dedicated, high-fidelity view to display and interpret this information. DNS is critical for uncovering hidden subdomains, email security posture (SPF/DMARC), and infrastructure providers.

## Missing Engine Controls & Visualization
1. **Multi-Record DNS Table**: A dedicated UI component that cleanly separates A, AAAA, CNAME, MX, and TXT records, rather than dumping them as raw text.
2. **SPF / DMARC Parsing**: The `DomainInfo` struct parses out SPF and DMARC text strings. The UI should display visual indicators (Green check/Red X) regarding the email spoofing posture of the domain.
3. **Name Server (NS) Analysis**: Visualize the NS records to immediately highlight the hosting or CDN provider (e.g., Route53, Cloudflare).

## Implementation Request
1. Create a `DnsInvestigationCard` component in the Svelte frontend.
2. Implement specific parsing in the UI for TXT records to highlight verification keys (Google Site Verification, Atlassian, etc.).
3. Add a button to copy the entire raw zone file (all parsed records) to the clipboard.
