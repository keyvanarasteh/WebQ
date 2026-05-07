# API Security Scanner Module

## Overview
The `API Security Scanner` module focuses on discovering and fuzzing backend API architectures. It probes for exposed GraphQL interfaces, Swagger/OpenAPI documentation, and common REST endpoints.

## Implementation Status
**Status:** ✅ Complete

## Backend Integration
The `scan_api_security` command utilizes `scan_api_endpoints`.
- Performs rapid enumeration over common API paths (`/api/v1`, `/graphql`, `/swagger-ui.html`).
- Tests for lack of rate limiting, missing authentication headers, and verbose error messages.
- Identifies REST vs GraphQL paradigms automatically.

## Frontend UI Components
Located in `/src/lib/components/assessment/api-security/`:
*   **`ApiEndpointsDirectory.svelte`**: Lists all successfully resolved API endpoints and their inferred authentication requirements.
*   **`ApiFuzzerRadar.svelte`**: Visualizes the fuzzing attack surface, highlighting parameters or endpoints that responded anomalously to payload injection.
*   **`ApiVulnLog.svelte`**: A detailed logging component tracking discovered vulnerabilities like BOLA (Broken Object Level Authorization) or Mass Assignment indicators.

## Missing Sub-Features
While the core module is integrated, `web-analyzer` returns significant additional data that is currently missing from the WebQ UI. See the [Security Assessment Extended UI](./ideas/security_assessment_extended_ui.md) request for details.

