# Engine Signatures & Algorithms Encyclopedia

This document exposes the raw, hardcoded signatures, dictionaries, and algorithmic heuristics used by the `web-analyzer` Rust backend. Currently, this data is obscured from the WebQ UI. Understanding these engine-level constants is critical for auditing the scanner's true capabilities.

## 1. Advanced Content Scanner (`advanced_content_scanner.rs`)

### 1.1 Secret Signatures (`SECRET_PATTERNS`)
The engine uses 24 precise regexes to detect secrets. Some notable ones:
*   **AWS Access Key**: `\bAKIA[0-9A-Z]{16}\b` (Medium severity)
*   **AWS Secret Key**: `\b[0-9a-zA-Z/+]{40}\b` (High severity)
*   **Google API Key**: `\bAIza[0-9A-Za-z\-_]{35}\b`
*   **Stripe API Key**: `\b(?:sk|pk)_(live|test)_[0-9a-zA-Z]{24,34}\b`
*   **GitHub Token**: `\b(?:github|gh)(?:_pat)?_[0-9a-zA-Z]{36,40}\b`
*   **Slack Webhook**: `https://hooks\.slack\.com/services/T[a-zA-Z0-9_]+/B[a-zA-Z0-9_]+/[a-zA-Z0-9_]+`
*   **MongoDB Connection String**: `mongodb(?:\+srv)?://[^/\s]+:[^/\s]+@[^/\s]+`
*   **Generic Passwords**: `(?i)(?:password|passwd|pwd)[\s=:]+["'`]([^"'`\s]{8,64})["'`]`

### 1.2 JavaScript Vulnerability Signatures (`JS_VULN_CATEGORIES`)
Scans inline and external JS files for 13 classes of DOM vulnerabilities:
*   **DOM XSS**: `document.write`, `.innerHTML`, `.outerHTML`, `eval`
*   **Open Redirect**: `location.href`, `location.replace`, `location.assign`
*   **CORS Misconfig**: `Access-Control-Allow-Origin: *` or `null`
*   **Prototype Pollution**: `__proto__`, `prototype[`
*   **Command Injection**: `exec(`, `spawn(`
*   **Insecure Crypto**: `createHash`, `crypto.subtle` with `MD5`/`SHA1`, or `Math.random()`

### 1.3 SSRF Parameters (`SSRF_PARAMS`)
Matches input names against a 64-item list of potentially vulnerable parameters (e.g., `url`, `uri`, `redirect`, `dest`, `callback`, `webhook`, `host`, `api_endpoint`).

---

## 2. Web Technologies (`web_technologies.rs`)

The fingerprinting engine relies on multiple byte arrays.
*   **Web Servers (10)**: Nginx, Apache, IIS, Cloudflare, LiteSpeed, Caddy, Traefik, Envoy, Gunicorn, uWSGI.
*   **JS Libraries (12)**: jQuery, Lodash, Moment.js, D3.js, Chart.js, Three.js, GSAP, Axios, Swiper, Bootstrap JS, Popper, Font Awesome.
*   **CSS Frameworks (8)**: Bootstrap, Tailwind CSS, Bulma, Foundation, Semantic UI, Materialize, UIKit, Pure CSS.
*   **CMS (11)**: WordPress, Drupal, Joomla, Magento, Shopify, Wix, Squarespace, Ghost, Webflow, TYPO3, Concrete5.
*   **E-Commerce (9)**: Shopify, WooCommerce, Magento, PrestaShop, BigCommerce, OpenCart, Stripe, PayPal, Square.
*   **Analytics (8)**: Google Analytics, Google Tag Manager, Facebook Pixel, Hotjar, Mixpanel, Segment, Adobe Analytics, Yandex Metrica.
*   **WAF (8)**: Cloudflare, AWS WAF, Incapsula, Akamai, Sucuri, ModSecurity, F5 BIG-IP, Barracuda.
*   **WordPress Plugins (10)**: Yoast, Akismet, Jetpack, WooCommerce, Contact Form 7, Elementor, Wordfence, WP Super Cache, All in One SEO, Google Analytics.

---

## 3. Subdomain Takeover (`subdomain_takeover.rs`)

The takeover scanner checks for "Dangling CNAMEs" by mapping CNAME resolutions to specific error pages for **36 exact SaaS providers**:
*   **Major Providers**: AWS S3 (`NoSuchBucket`), AWS CloudFront (`Bad request`), GitHub Pages (`404: Not Found`), Heroku (`No such app`), Vercel, Netlify.
*   **Cloud/DevOps**: Azure App Service, Azure TrafficManager, Pantheon, Fly.io, Surge.sh, Digital Ocean, AWS Elastic Beanstalk.
*   **CMS/Blog**: Shopify, WordPress.com, Ghost.io, Tumblr, Squarespace, Webflow.
*   **Support/Helpdesk**: Zendesk (`Help Center Closed`), Desk, Helpjuice, HelpScout, UserVoice.
*   **Marketing/Analytics**: Campaign Monitor, Unbounce, Pingdom, UptimeRobot.

---

## 4. API Security Scanner (`api_security_scanner.rs`)

The API scanner uses complex algorithms that aren't visible as static arrays:
*   **Time-based Blind SQLi Detection**: Injects `SLEEP` and `WAITFOR` payloads. It actively monitors request duration; if the `start.elapsed().as_secs_f64()` exceeds **4.8 seconds**, it flags the parameter as critically vulnerable to Time-Based Blind SQLi.
*   **Error-based SQLi**: Checks the response body against regexes for MySQL (`You have an error in your SQL syntax`), PostgreSQL, Oracle (`ORA-[0-9]{5}`), and SQLite.
*   **API Framework Scoring System**: To identify if a path is an API without OpenAPI documentation, it uses a points system. It assigns scores for JSON presence, `x-api-key` or `x-rate-limit` headers, and known framework server headers (e.g., Express, Flask, FastAPI). If the score is >= 4, it proceeds to fuzz the endpoint.
