# Extended UI: Intelligence Gathering Modules

## Overview
While WebQ has integrated the core functionality of the `web-analyzer` Intelligence Gathering modules (Domain Info, DNS, SEO Analysis, Web Technologies, Bulk Validator), a deep dive into the `web-analyzer` source code reveals that a massive amount of detailed telemetry is discarded or simply not visualized by the current frontend components.

## Missing Data Visualizations

### 1. SEO Analysis (`seo_analysis.rs`)
The backend calculates a 13-category SEO audit, but the UI is missing visualizations for:
*   `MobileAccessibilityResult`: Viewport scaling, touch-target sizing metrics.
*   `SchemaMarkupResult`: Count and breakdown of specific structured data elements (JSON-LD, Microdata).
*   `LinkAnalysisResult`: Granular counts of internal vs external links, nofollow attributes, and broken link detection.
*   `ImageSeoResult` & `AltAttributeResult`: Ratio of images missing alt tags, lazy loading states.
*   `PageSpeedResult`: Backend-inferred load time metrics (`load_time_secs`).
*   `SocialMediaResult`: Extracted Open Graph (`og:title`, `og:image`) and Twitter Cards metadata.

### 2. Domain Info (`domain_info.rs`)
*   **SSL Certificates**: The `SslInfo` struct returns deep certificate chain details (Issuer, Subject, Serial numbers, SHA256 Fingerprints, Validity bounds) that are not currently displayed in the basic grid.
*   **Raw Port Responses**: Port scan arrays are returned, but banner grabbing responses are ignored.

### 3. Web Technologies (`web_technologies.rs`)
*   The backend detects 16 different categories of tech stacks (Analytics, CDNs, Caching, Databases, Frameworks, JavaScript libraries). WebQ currently only visualizes CMS (WordPress) and basic frameworks. The UI needs expanding to cover all 16 categories using a categorised radar chart or dense grid.

## Implementation Request
Build dedicated Svelte 5 components (e.g., `SchemaMarkupCard.svelte`, `SslChainViewer.svelte`, `TechCategoryRadar.svelte`) to surface these ignored data structures, providing a truly enterprise-grade intelligence dashboard.
