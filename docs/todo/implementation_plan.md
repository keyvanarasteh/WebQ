# Implementation Plan: Domain Intelligence 0→100% Coverage

## Goal
Bring all 3 Domain Intelligence sub-modules ([domain-info](file:///home/drvoid/ISU/WebQ/src/routes/intelligence/domain-info), [domain-dns](file:///home/drvoid/ISU/WebQ/src/routes/intelligence/domain-dns), `seo-analysis`) from their current skeleton/broken state to full 100% rendering coverage of every field returned by the `web-analyzer` crate.

## User Review Required

> [!IMPORTANT]
> The SEO Analysis page currently uses **wrong property keys** ([basic](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#298-357)/[technical](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#537-586)/[social](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#589-621) instead of [basic_seo](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#298-357)/`technical_seo`/`social_media`), making all 3 sub-components render empty data. This is a silent breakage — no errors, just blank UI.

> [!WARNING]
> The `web-analyzer` crate has a bug where `ssl.days_until_expiry` is **never computed** (field exists in struct but [check_ssl()](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#457-557) never calculates it). We'll fix this in the crate directly.

---

## Proposed Changes

### Phase 1: TypeScript Type Definitions

#### [NEW] [types.ts](file:///home/drvoid/ISU/WebQ/src/lib/types/intelligence.ts)

Create strongly-typed interfaces matching every crate struct:

- [DomainInfoResult](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#74-90) — maps to `web-analyzer::domain_info::DomainInfoResult`
- [WhoisInfo](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#92-103), [SslInfo](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#105-120), [DnsInfo](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#122-131), [SecurityInfo](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#133-139)
- [DomainDnsResult](file:///home/drvoid/Qix/web-analyzer/src/domain_dns.rs#19-25), [DnsRecords](file:///home/drvoid/Qix/web-analyzer/src/domain_dns.rs#8-17)
- [SeoAnalysisResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#39-55) — all 13 sub-structs ([BasicSeoResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#71-81), [TitleAnalysis](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#57-62), [MetaDescAnalysis](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#64-69), [ContentAnalysisResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#96-105), [HeadingInfo](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#83-87), [KeywordInfo](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#89-94), [TechnicalSeoResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#107-116), [SocialMediaResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#118-122), [PerformanceResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#124-133), [MobileAccessibilityResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#143-149), [AltAttributeResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#135-141), [SchemaMarkupResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#151-157), [LinkAnalysisResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#159-165), [ImageSeoResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#167-174), [PageSpeedResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#176-183), [SeoScoreResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#185-191))

This eliminates all `any` types across the intelligence pages (fixes FE-04 violations).

---

### Phase 2: Domain Info — DomainOverview Rewrite

#### [MODIFY] [DomainOverview.svelte](file:///home/drvoid/ISU/WebQ/src/lib/components/intelligence/domain-info/DomainOverview.svelte)

**Currently:** Shows only `whois.registrar` in a single card.

**Will show (full metric grid):**
| Row | Fields |
|---|---|
| Network | `ipv4`, `all_ipv4` list, `ipv6` list, [reverse_dns](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#236-249) |
| Server | `web_server`, [http_status](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#629-656), `response_time_ms` |
| WHOIS | `registrar`, `registrant`, `creation_date`, `expiry_date`, `last_updated`, `privacy_protection` |
| DNS | `whois.name_servers`, `whois.domain_status` |

Design: Neon metric-grid cards grouped into collapsible sections. Each value is `font-mono` with copy-on-click.

---

### Phase 3: Domain Info — SSL Card Rewrite

#### [MODIFY] [SslStatus.svelte](file:///home/drvoid/ISU/WebQ/src/lib/components/intelligence/domain-info/SslStatus.svelte)

**Currently:** Only shows `days_until_expiry` gauge (always `?`).

**Will show:**
- Status badge (`Valid` / `Error` / `HTTPS not available`)
- Issued To (CN)
- Issuer (CA)
- Protocol Version (TLS version)
- Expiry Date (raw string)
- Days Until Expiry (circular gauge — after crate fix)
- SANs (alternative names list)

---

### Phase 4: Domain Info — Security Details Card

#### [NEW] [SecurityDetails.svelte](file:///home/drvoid/ISU/WebQ/src/lib/components/intelligence/domain-info/SecurityDetails.svelte)

Renders the [security](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#659-691) object:
- HTTPS Available badge
- HTTP→HTTPS Redirect check
- Security Headers checklist (HSTS, X-Frame-Options, X-Content-Type-Options, X-XSS-Protection, CSP) with present/missing indicators
- Headers coverage count

#### [MODIFY] [+page.svelte](file:///home/drvoid/ISU/WebQ/src/routes/intelligence/domain-info/+page.svelte)

- Import `SecurityDetails` component
- Replace `any` types with [DomainInfoResult](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#74-90)
- Add error toast via Svelte-Sonner

---

### Phase 5: Domain DNS — Minor Enhancements

#### [MODIFY] [+page.svelte](file:///home/drvoid/ISU/WebQ/src/routes/intelligence/domain-dns/+page.svelte)

- Add `timestamp` and `response_time_ms` display in header
- Replace `any` with [DomainDnsResult](file:///home/drvoid/Qix/web-analyzer/src/domain_dns.rs#19-25) type

---

### Phase 6: SEO Analysis — Complete Rewrite

This is the biggest phase. Current page is **non-functional** due to wrong property keys.

#### [MODIFY] [+page.svelte](file:///home/drvoid/ISU/WebQ/src/routes/intelligence/seo-analysis/+page.svelte)

- Fix property key mappings: [basic](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#298-357) → [basic_seo](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#298-357), [technical](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#537-586) → `technical_seo`, [social](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#589-621) → `social_media`
- Add new component imports for the 10 missing sections
- Replace `any` with [SeoAnalysisResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#39-55)
- Restructure layout to accommodate all 13 sections in a responsive masonry/grid

#### [MODIFY] [BasicSeoOverview.svelte](file:///home/drvoid/ISU/WebQ/src/lib/components/intelligence/seo-analysis/BasicSeoOverview.svelte)

Fix type to match crate's [BasicSeoResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#71-81):
- `title` → `title.text`, `title.length`, `title.status` (was flat `title: string`)
- `description` → `meta_description.text`, `.length`, `.status` (was flat `description: string`)
- Add: `canonical_url`, `meta_robots`, `viewport`, `language`, [charset](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#382-404)
- Remove: `h1_count`, `heading_structure` (these are in `content_analysis`)

#### [MODIFY] [TechnicalSeoCard.svelte](file:///home/drvoid/ISU/WebQ/src/lib/components/intelligence/seo-analysis/TechnicalSeoCard.svelte)

Fix type to match crate's [TechnicalSeoResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#107-116):
- Remove wrong fields: `is_ssl`, `has_sitemap`, `has_robots_txt`
- Add correct fields: `page_size_bytes`, [http_status](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#629-656), `redirects`, `internal_links`, `external_links`, `structured_data_count`, `has_breadcrumbs`

#### [MODIFY] [SocialMediaCard.svelte](file:///home/drvoid/ISU/WebQ/src/lib/components/intelligence/seo-analysis/SocialMediaCard.svelte)

Fix type to match crate's [SocialMediaResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#118-122):
- Change from flat `og_title, og_description, og_image, twitter_card` to `open_graph: Record<string, string>` and `twitter_cards: Record<string, string>` HashMaps
- Iterate over HashMap entries dynamically

#### [NEW] 10 new SEO sub-components:

| Component | Crate Section | Key Displays |
|---|---|---|
| `ContentAnalysis.svelte` | `content_analysis` | Heading hierarchy, word count, text:html ratio, keyword density |
| `AnalyticsTracker.svelte` | [analytics](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#624-657) | GA4, GTM, Facebook Pixel, etc. detection grid |
| `PerformanceCard.svelte` | [performance](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#660-695) | Load time, content size, compression, cache-control |
| `MobileAccessibility.svelte` | `mobile_accessibility` | Viewport, mobile-friendly, alt coverage, ARIA labels |
| `SeoResourcesCheck.svelte` | [seo_resources](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#733-745) | robots.txt, sitemap.xml, humans.txt, ads.txt presence |
| `SchemaMarkup.svelte` | `schema_markup` | JSON-LD count/types, microdata |
| `LinkAnalysis.svelte` | `link_analysis` | Total/internal/external/nofollow link counts |
| `ImageSeoCard.svelte` | `image_seo` | Image count, lazy-load, alt/title coverage, score |
| `PageSpeedFactors.svelte` | `page_speed_factors` | CSS/JS files, inline styles/scripts, compression |
| `SeoScoreGauge.svelte` | [seo_score](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#902-982) | Score/max, percentage, grade (A-F) |

---

### Phase 7: Crate Bug Fix — `days_until_expiry`

#### [MODIFY] [domain_info.rs](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs)

In [check_ssl()](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#457-557), after extracting `expiry_date` string, parse it with `chrono` and compute `days_until_expiry = (expiry_date - now).num_days()`.

---

### Phase 8: i18n Messages

#### [MODIFY] [en.json](file:///home/drvoid/ISU/WebQ/messages/en.json)
#### [MODIFY] [tr.json](file:///home/drvoid/ISU/WebQ/messages/tr.json)

Add new message keys for all new labels, section titles, status badges, and descriptions across all new components.

---

## Verification Plan

### Build Verification
1. `cd /home/drvoid/Qix/web-analyzer && cargo build --all-features` — verify crate compiles after `days_until_expiry` fix
2. `cd /home/drvoid/ISU/WebQ/src-tauri && cargo build` — verify Tauri backend compiles with updated crate
3. `cd /home/drvoid/ISU/WebQ && npm run check` — verify Svelte/TypeScript has no compilation errors

### Manual UI Verification
1. Run `cd /home/drvoid/ISU/WebQ && npm run tauri dev`
2. Navigate to **Intelligence → Domain Info**, enter `qline.tech`, click Scan
   - Verify: DomainOverview shows all WHOIS fields (registrar, dates, registrant, nameservers)
   - Verify: Network section shows IPv4/IPv6, reverse DNS
   - Verify: Server section shows web_server, HTTP status, response time
   - Verify: SslStatus shows issuer, protocol, expiry, days gauge, SANs
   - Verify: SecurityDetails shows HTTPS check, redirect, security headers
   - Verify: PortSecurityMatrix shows ports + score (unchanged)
3. Navigate to **Intelligence → Domain DNS**, enter `qline.tech`, click Scan
   - Verify: Timestamp and response time shown in header
4. Navigate to **Intelligence → SEO Analysis**, enter `qline.tech`, click Scan
   - Verify: **Data actually appears** (was blank before)
   - Verify: All 13 sections render with real data
   - Verify: SEO Score gauge shows grade
