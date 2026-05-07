# SEO Analysis Module

## Overview
The `SEO Analysis` module brings comprehensive Search Engine Optimization auditing to WebQ via the `web-analyzer` crate. It evaluates technical SEO aspects, content structure, link profiles, and schema markups for a specific URL.

## Implementation Status
**Status:** ✅ Complete

The backend `scan_seo_analysis` command securely pipes the extensive data returned by the analyzer into an extremely rich and categorized Svelte frontend.

## Backend Integration
The `scan_seo_analysis` command invokes the `analyze_advanced_seo` core function inside `web-analyzer` asynchronously.
- Evaluates title, meta descriptions, canonical links, and viewport tags.
- Parses the DOM for header hierarchies (H1-H6) and calculates content-to-html ratios.
- Verifies the existence of critical files like `robots.txt` and `sitemap.xml`.
- Streams progress updates directly to the frontend via Tauri MPSC channels.

## Frontend UI Components
This module has one of the most extensive UI implementations located in `/src/lib/components/intelligence/seo-analysis/`. It uses a multi-faceted dashboard approach:

*   **`SeoScoreGauge.svelte`**: Central visual gauge rating the overall SEO posture.
*   **`BasicSeoOverview.svelte`**: Core meta tags and heading structure.
*   **`ContentAnalysis.svelte` / `LinkAnalysis.svelte`**: Deep dive into content metrics and internal/external linking.
*   **`TechnicalSeoCard.svelte` / `PerformanceCard.svelte`**: Detailed technical diagnostics (HTTPS, gzip, lazy loading, response times).
*   **`SchemaMarkup.svelte` / `SocialMediaCard.svelte`**: OpenGraph, Twitter Cards, and JSON-LD structured data validation.
*   **Interactive Guides**: Every metric has an accompanying `*Guide.svelte` component offering contextual, educational overviews on how to resolve flagged issues.

## Data Flow
1.  A URL is submitted to the `/intelligence/seo-analysis` route.
2.  Tauri fetches and parses the HTML alongside header responses.
3.  The detailed `SeoAnalysisResult` payload is fed reactively to the dozen specialized Svelte components rendering categorized metrics and scores.
