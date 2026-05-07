<p align="center">
  <h1 align="center">🔍 WebQ</h1>
  <p align="center">
    <strong>Enterprise Web Intelligence & Cybersecurity Interface</strong>
  </p>
  <p align="center">
    High-performance Svelte 5 & Tauri UI for the <code>web-analyzer</code> reconnaissance engine
  </p>
</p>

<p align="center">
  <a href="https://github.com/keyvanarasteh/WebQ/releases"><img src="https://img.shields.io/github/v/release/keyvanarasteh/WebQ?style=for-the-badge&logo=github&logoColor=white&color=orange" alt="Release"></a>
  <a href="https://github.com/keyvanarasteh/WebQ/actions"><img src="https://img.shields.io/github/actions/workflow/status/keyvanarasteh/WebQ/release.yml?style=for-the-badge&logo=github&label=CI" alt="CI"></a>
</p>

<p align="center">
  <a href="#"><img src="https://img.shields.io/badge/frontend-svelte%205-ff3e00?style=flat-square&logo=svelte" alt="Svelte 5"></a>
  <a href="#"><img src="https://img.shields.io/badge/backend-tauri%202.0-ffc131?style=flat-square&logo=tauri" alt="Tauri"></a>
  <a href="#"><img src="https://img.shields.io/badge/engine-web--analyzer-black?style=flat-square&logo=rust" alt="WebAnalyzer"></a>
  <a href="#"><img src="https://img.shields.io/badge/styling-tailwindcss%20v4-38bdf8?style=flat-square&logo=tailwindcss" alt="TailwindCSS"></a>
  <a href="#"><img src="https://img.shields.io/badge/platform-desktop%20%26%20mobile-lightgrey?style=flat-square&logo=linux" alt="Cross-Platform"></a>
</p>

---

## ✨ Features

- **Monolithic Intelligence Dashboard** — Centralized UI utilizing advanced Glassmorphism design tokens for high-density OSINT data tracking.
- **Deep Reconnaissance Arrays** — Visually maps concurrent HTTP/HTTPS infrastructure profiling, subdomain takeover checking, and cloud CDN bypasses.
- **Dynamic Security Matrices** — Interactive Wiki dashboards covering 109 WHOIS Native root registries, 200+ port definitions, and 15+ modern Security Header specifications.
- **Seamless Tauri IPC Flow** — Connects directly to the `web-analyzer` crates.io engine, broadcasting real-time concurrent JSON streams and shell footprints via MPSC channels without UI blocking.
- **Granular Security Gradings** — Automatically assesses and grades Domain Posture from `A+` to `F` factoring in WAF configurations, CORS vulnerabilities, and Nmap CVE mappings.
- **Automated Deployments** — Fully synchronized Github Action CI/CD targeting `.deb`, `.AppImage`, `.exe` (NSIS), and `Apple Silicon .dmg` alongside bleeding-edge Cannonical Snap Store availability.

---

## 📦 Reconnaissance Modules

WebQ bridges the entire suite of `web-analyzer` features into an interactive cross-platform UI.

### 🔎 Intelligence Gathering
| Module | Core Functionality |
|--------|----------------|
| **Domain Insight** | Renders WHOIS (TCP), SSL chain checks, local DNS mapping, and port detection grids. |
| **SEO Auditing** | Scans 13 categories, tracking schema markers and visualizes technical gaps via checklists. |
| **Tech Fingerprinting** | Identifies 11 CMS platforms, frameworks, CDN footprints, and WpUser enumerations via Wappalyzer-like matrices. |
| **Bulk Domain Validation** | Parallel processing for bulk asset arrays testing DNS, HTTP, and TLS statuses with unified success graphs. |

### 🕵️ Defensive SecOps
| Module | Core Functionality |
|--------|----------------|
| **Subdomain Live Probing** | Combines `subfinder` with concurrent async HTTP tests to visually filter live/dead target subdomains. |
| **Target Contact Spidering** | Implements BFS crawling to extract organization Emails, Social signatures, and Vcards matching specified regex bounds. |
| **Advanced Secret Scanner** | Sweeps exposed `/config`, `/env`, and `/v1` logic testing 24 hardcoded API secret leaks with pulsing CVSS alerts. |

### 🛡️ Vulnerability Analysis
| Module | Core Functionality |
|--------|----------------|
| **Takeover Analysis** | Maps active subdomains against a 36-service vulnerable CNAME registry with explicit mitigation tips. |
| **Cloudflare Unmasking** | Scrapes history logs against reverse DNS databases to filter private IPs and verify origin connectivity. |
| **Nmap Zero-Day Correlation** | Pipes `nmap` XML footprints natively into NVD databases rendering critical CVE threat data via interactive accordions. |

---

## 🤝 OpenSource Requests

Help us build the next generation of cybersecurity tools! We are actively looking for community contributions. Check out our open requests below:

| Request | Description | Details |
|---------|-------------|---------|
| **React2Shell Honeypot** | Implement UI components for the React2Shell Honeypot engine | [Read Request](./REQUEST.md) |
| **React2Shell UI Profiling** | Build Attacker Profiling and Signature Matrix UI for React2Shell | [Read Request](./docs/ideas/react2shell_ui.md) |
| **Intelligence Gathering Extended UI** | Expose missing SEO, CMS, SSL, and DNS telemetry in the UI | [Read Request](./docs/ideas/intelligence_gathering_extended_ui.md) |
| **Reconnaissance Extended UI** | Surface data provenance and code context windows for scanners | [Read Request](./docs/ideas/reconnaissance_extended_ui.md) |
| **Security Assessment Extended UI** | Add Exploit-DB linkage and granular header/cookie analysis | [Read Request](./docs/ideas/security_assessment_extended_ui.md) |

---

## 🚀 Quick Start (Development)

### Prerequisites

```bash
# Frontend
npm install -g bun

# Desktop Dependencies
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev

# Analysis Subprocesses
sudo apt install nmap dnsutils openssl whois
go install github.com/projectdiscovery/subfinder/v2/cmd/subfinder@latest
```

### Build Pipeline

```bash
# Configure submodules and SvelteKit parameters
bun install

# Launch Tauri Development Hot-Reload 
bun run tauri dev
```

---

## 🏗️ Release Pipeline

WebQ includes sophisticated native compilation pipelines for deployment.

```bash
git commit -m "chore: release WebQ vX.Y.Z"
git tag vX.Y.Z
git push origin main --tags
```
_Deploying an annotated `v*` tag dynamically triggers the `.github/workflows/release.yml` and `.github/workflows/snap.yml` cloud compilation matrices, auto-releasing the UI builds to Github and Cannonical._

---

<p align="center">
  <sub>UI crafted with ✨ Obsidian Dashboards — İstinye University</sub>
</p>