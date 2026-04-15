# 🔍 Domain Intelligence — Deep Completeness Audit

> **Verdict: ❌ NOT fully implemented.** All 3 sub-modules have significant gaps between what the `web-analyzer` crate provides and what the WebQ frontend actually renders.

---

## Summary Matrix

| Module | Crate Fields | Fields Rendered | Coverage | Status |
|---|---|---|---|---|
| **Domain Info** | 15 fields | **1 field** | ~7% | 🔴 Skeleton |
| **Domain DNS** | 7 record types + metadata | 7 types + SPF/DMARC | ~90% | 🟢 Mostly Done |
| **SEO Analysis** | 13 result sections | 3 sections (wrong keys) | ~23% | 🔴 Broken + Skeleton |

---

## 1. Domain Info ([/intelligence/domain-info](file:///home/drvoid/ISU/WebQ/src/routes/intelligence/domain-info))

### Crate Output ([DomainInfoResult](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#74-90))
The `web-analyzer` crate returns a rich struct with 15+ fields:

| Field | Type | Rendered? | Notes |
|---|---|---|---|
| [domain](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#220-233) | `String` | ❌ | Not displayed |
| `ipv4` | `Option<String>` | ❌ | Comment placeholder only |
| `ipv6` | `Vec<String>` | ❌ | Not rendered |
| `all_ipv4` | `Vec<String>` | ❌ | Not rendered |
| [reverse_dns](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#236-249) | `Option<String>` | ❌ | Not rendered |
| `whois.registrar` | `String` | ✅ | **Only WHOIS field shown** |
| `whois.creation_date` | `String` | ❌ | Not rendered |
| `whois.expiry_date` | `String` | ❌ | Not rendered |
| `whois.last_updated` | `String` | ❌ | Not rendered |
| `whois.domain_status` | `Vec<String>` | ❌ | Not rendered |
| `whois.registrant` | `String` | ❌ | Not rendered |
| `whois.privacy_protection` | `String` | ❌ | Not rendered |
| `whois.name_servers` | `Vec<String>` | ❌ | Not rendered |
| `ssl.status` | `String` | ❌ | Not rendered |
| `ssl.issued_to` | `Option<String>` | ❌ | Not rendered |
| `ssl.issuer` | `Option<String>` | ❌ | Not rendered |
| `ssl.protocol_version` | `Option<String>` | ❌ | Not rendered |
| `ssl.expiry_date` | `Option<String>` | ❌ | Not rendered |
| `ssl.days_until_expiry` | `Option<i64>` | ✅ | Circular gauge, but **always shows `?`** because crate doesn't compute this value |
| `ssl.alternative_names` | `Vec<String>` | ❌ | Not rendered |
| `dns.nameservers` | `Vec<String>` | ❌ | Not rendered (internal DNS struct) |
| `dns.mx_records` | `Vec<String>` | ❌ | Not rendered |
| `dns.txt_records` | `Vec<String>` | ❌ | Not rendered |
| `dns.spf` | `Option<String>` | ❌ | Not rendered |
| `dns.dmarc` | `Option<String>` | ❌ | Not rendered |
| `open_ports` | `Vec<String>` | ✅ | Port badges with danger colors |
| [http_status](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#629-656) | `Option<String>` | ❌ | Not rendered |
| `web_server` | `Option<String>` | ❌ | Comment placeholder only |
| `response_time_ms` | `Option<f64>` | ❌ | Not rendered |
| `security.*` | [SecurityInfo](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#133-139) | ❌ | Not rendered (HTTPS, headers) |
| [security_score](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#694-727) | `u32` | ✅ | Big number + score |

### Issues in [DomainOverview.svelte](file:///home/drvoid/ISU/WebQ/src/lib/components/intelligence/domain-info/DomainOverview.svelte)
- **Only renders `result.whois.registrar`** — the entire grid comment says `"More metric key-value cards will go here"` but they were never added.
- Uses `any` type for props (violates FE-04 TypeScript strictness rule).

### Issues in [SslStatus.svelte](file:///home/drvoid/ISU/WebQ/src/lib/components/intelligence/domain-info/SslStatus.svelte)
- Only renders `ssl.days_until_expiry` — which is **always `null`** because the crate's SSL check function never computes `days_until_expiry` (it extracts `expiry_date` as a string but doesn't calculate the day difference). So the UI always shows `?`.
- **6 other SSL fields** ([status](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#629-656), `issued_to`, `issuer`, `protocol_version`, `expiry_date`, `alternative_names`) are completely ignored.

### Issues in [PortSecurityMatrix.svelte](file:///home/drvoid/ISU/WebQ/src/lib/components/intelligence/domain-info/PortSecurityMatrix.svelte)
- ✅ Renders [ports](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#598-626) and [security_score](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#694-727) correctly.
- Missing `security.https_available`, `security.https_redirect`, and `security.security_headers` display.

---

## 2. Domain DNS ([/intelligence/domain-dns](file:///home/drvoid/ISU/WebQ/src/routes/intelligence/domain-dns))

### Crate Output ([DomainDnsResult](file:///home/drvoid/Qix/web-analyzer/src/domain_dns.rs#19-25))
| Field | Rendered? | Notes |
|---|---|---|
| `timestamp` | ❌ | Not shown |
| [domain](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#220-233) | ❌ | Not shown as header |
| `records.a` | ✅ | Tab A |
| `records.aaaa` | ✅ | Tab AAAA |
| `records.mx` | ✅ | Tab MX |
| `records.ns` | ✅ | Tab NS |
| `records.soa` | ✅ | Tab SOA |
| `records.txt` | ✅ | Tab TXT |
| `records.cname` | ✅ | Tab CNAME |
| `response_time_ms` | ❌ | Not shown |
| SPF detection (from TXT) | ✅ | [DnsSecurityCheck.svelte](file:///home/drvoid/ISU/WebQ/src/lib/components/intelligence/domain-dns/DnsSecurityCheck.svelte) |
| DMARC detection (from TXT) | ✅ | [DnsSecurityCheck.svelte](file:///home/drvoid/ISU/WebQ/src/lib/components/intelligence/domain-dns/DnsSecurityCheck.svelte) |

### ✅ This module is **mostly complete**
Minor missing items: `timestamp`, [domain](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#220-233), and `response_time_ms` are not rendered but are non-critical metadata.

---

## 3. SEO Analysis (`/intelligence/seo-analysis`)

### 🚨 Critical: Property Name Mismatch

The frontend accesses `scanResult.basic`, `scanResult.technical`, `scanResult.social` but the crate returns:
- [basic_seo](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#298-357) (not [basic](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#298-357))
- `technical_seo` (not [technical](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#537-586))
- `social_media` (not [social](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#589-621))

**This means the SEO page will always show empty/null data** even after a successful scan. All 3 sub-components will render "No data" states.

### Crate Output ([SeoAnalysisResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#39-55) — 13 sections)
| Section | Frontend Component | Rendered? | Notes |
|---|---|---|---|
| [basic_seo](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#298-357) | `BasicSeoOverview` | ⚠️ Wrong key | Accessed as `.basic`, expects flat `{title, description, keywords, h1_count, heading_structure}` but crate returns nested `{title: TitleAnalysis, meta_description: MetaDescAnalysis, ...}` |
| `content_analysis` | — | ❌ | No component exists |
| `technical_seo` | `TechnicalSeoCard` | ⚠️ Wrong key + wrong shape | Accessed as `.technical`, expects `{is_ssl, has_sitemap, has_robots_txt}` but crate returns `{page_size_bytes, http_status, redirects, internal_links, external_links, structured_data_count, has_breadcrumbs}` |
| `social_media` | `SocialMediaCard` | ⚠️ Wrong key + wrong shape | Accessed as `.social`, expects `{og_title, og_description, og_image, twitter_card}` but crate returns `{open_graph: HashMap, twitter_cards: HashMap}` |
| [analytics](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#624-657) | — | ❌ | No component exists |
| [performance](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#660-695) | — | ❌ | No component exists |
| `mobile_accessibility` | — | ❌ | No component exists |
| [seo_resources](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#733-745) | — | ❌ | No component (robots.txt, sitemap.xml, humans.txt, ads.txt checks) |
| `schema_markup` | — | ❌ | No component exists |
| `link_analysis` | — | ❌ | No component exists |
| `image_seo` | — | ❌ | No component exists |
| `page_speed_factors` | — | ❌ | No component exists |
| [seo_score](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#902-982) | — | ❌ | No component (score/grade/percentage all missing) |

---

## 4. Additional Code Quality Issues

| Issue | Files | Severity |
|---|---|---|
| `any` types used everywhere | All pages + components | 🟡 Violates FE-04 |
| No error toast on scan failure | [domain-info/+page.svelte](file:///home/drvoid/ISU/WebQ/src/routes/intelligence/domain-info/+page.svelte) | 🟡 Silent failures |
| No TypeScript interfaces for crate types | All components | 🔴 Fragile, no compile-time safety |
| `ssl.days_until_expiry` never computed | `web-analyzer/domain_info.rs` | 🔴 Crate bug — field exists but is never set |

---

## 5. Crate-Level Bug: `days_until_expiry` Never Computed

In [domain_info.rs](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs), the [SslInfo](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#105-120) struct has `days_until_expiry: Option<i64>`, and [check_ssl()](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#457-557) extracts `expiry_date` as a raw string (e.g., `"Jun 15 12:00:00 2025 GMT"`), but **never parses the date or calculates the day difference**. The field is always `None`, causing the UI gauge to show `?`.
