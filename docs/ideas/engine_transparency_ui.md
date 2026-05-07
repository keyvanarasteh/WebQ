# Feature Request: Engine Transparency UI

## Overview
WebQ is powered by the `web-analyzer` Rust crate, which has an incredibly rich, hardcoded dictionary of signatures and heuristic algorithms. Currently, WebQ acts as a black box—the user hits "Scan", and results appear. 

We need to expose the **Engine Ruleset** to the user, not just for transparency, but to allow security analysts to toggle specific rules on or off, thereby customizing their scan profiles.

## Missing Engine Controls
The UI is currently missing the ability to view or configure the internal data dictionaries:

1. **Subdomain Takeover Provider Matrix**: The engine supports exactly 36 SaaS providers (AWS, Heroku, GitHub Pages, Pantheon, Fly.io, etc.). The UI needs a matrix showing these supported providers and their exact CNAME/Error signatures.
2. **Secret Regex Toggles**: The engine checks 24 distinct regex patterns for API keys (Stripe, Slack, AWS, Google, SSH keys). Analysts should be able to disable regexes they don't care about to reduce false positives.
3. **JS Vulnerability Sinks**: The engine checks 13 categories of DOM vulnerabilities (e.g., `document.write` vs `eval`). Users should be able to see the exact regexes being executed against the JS.
4. **Tech Radar Categories**: The engine maps 16 distinct categories (Analytics, WAFs, E-Commerce, CSS frameworks). The UI only visualizes CMS and frontend/backend.
5. **Algorithmic Thresholds**: The API Scanner uses a strict `4.8s` latency threshold to confirm Time-Based Blind SQLi. Power users might want to adjust this threshold to account for naturally slow targets.

## Implementation Request
1. Create a new `Engine Settings` or `Signature Ruleset` tab in the WebQ settings module.
2. Build accordion lists that read from the `docs/signatures_and_algorithms.md` encyclopedia (or preferably, fetch these via a new Tauri command directly from the backend vectors).
3. Allow users to toggle individual rules on or off (e.g., "Disable checking for WordPress plugins").
