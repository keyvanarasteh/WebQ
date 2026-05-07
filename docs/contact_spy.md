# Contact Spy Module

## Overview
The `Contact Spy` module performs targeted OSINT (Open-Source Intelligence) spidering. By leveraging the `contact-spy` capabilities of `web-analyzer`, it crawls a target domain extracting emails, phone numbers, and social media footprints.

## Implementation Status
**Status:** ✅ Complete

## Backend Integration
The `scan_contacts` command invokes `crawl_contacts` from `web-analyzer`.
- Performs Breadth-First Search (BFS) spidering limited by a configurable `max_pages` parameter.
- Applies complex Regex heuristics to accurately harvest valid Emails, LinkedIn profiles, Twitter handles, and generic phone number formats from the raw HTML.
- Handles asynchronous HTTP requests safely while obeying baseline crawler rules.

## Frontend UI Components
Located in `/src/lib/components/recon/contact-spy/`:
*   **`CrawlingConsole.svelte`**: A visual "hacker-style" terminal output displaying the live spidering process and URLs visited.
*   **`EmailHarvester.svelte`**: Extracts and lists all validated email addresses found during the crawl.
*   **`SocialOsintBox.svelte`**: Categorizes and links to extracted social media profiles (Twitter, LinkedIn, Facebook, etc.).
*   **`ContactMasonry.svelte`**: The overarching layout component organizing the harvested data into a clean, glassmorphism dashboard.

## Missing Sub-Features
While the core module is integrated, `web-analyzer` returns significant additional data that is currently missing from the WebQ UI. See the [Reconnaissance Extended UI](./ideas/reconnaissance_extended_ui.md) request for details.

