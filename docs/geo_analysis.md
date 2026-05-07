# Geo Analysis Module

## Overview
The `Geo Analysis` module focuses on analyzing the geographical and AI-bot related footprints of a target infrastructure. It evaluates routing geography, data sovereignty compliance, and how the domain interacts with modern AI scraping bots (LLMs, MCPs).

## Implementation Status
**Status:** ✅ Complete

## Backend Integration
The `scan_geo_analysis` command invokes `analyze_geo` from the `web-analyzer` crate.
- Resolves the geographical location of the server's IP address.
- Investigates `robots.txt` and `llms.txt` for specific AI bot directives (e.g., blocking OpenAI, Anthropic, or CommonCrawl).
- Detects the presence of Model Context Protocol (MCP) server endpoints or web manifestations.

## Frontend UI Components
Located in `/src/lib/components/assessment/geo-analysis/`:
*   **`ApiBotDirectives.svelte`**: Visualizes the allowed/disallowed scraping permissions for major AI crawlers, extracting intelligence from `robots.txt`.
*   **`LLMsTxtCard.svelte`**: Specifically parses and displays the contents of the newly standardized `/llms.txt` if present.
*   **`WebMcpInspector.svelte`**: Detects and analyzes exposed Model Context Protocol (MCP) servers, highlighting potential data leakage via AI agents.

## Missing Sub-Features
While the core module is integrated, `web-analyzer` returns significant additional data that is currently missing from the WebQ UI. See the [Security Assessment Extended UI](./ideas/security_assessment_extended_ui.md) request for details.

