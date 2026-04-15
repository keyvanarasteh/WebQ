# Domain Intelligence — Full Coverage Task List

## Phase 1: TypeScript Type Definitions
- [ ] Create `src/lib/types/intelligence.ts` with all interfaces matching web-analyzer crate structs
  - [ ] [DomainInfoResult](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#74-90), [WhoisInfo](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#92-103), [SslInfo](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#105-120), [DnsInfo](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#122-131), [SecurityInfo](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#133-139)
  - [ ] [DomainDnsResult](file:///home/drvoid/Qix/web-analyzer/src/domain_dns.rs#19-25), [DnsRecords](file:///home/drvoid/Qix/web-analyzer/src/domain_dns.rs#8-17)
  - [ ] All 16 SEO types ([SeoAnalysisResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#39-55), [BasicSeoResult](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#71-81), [TitleAnalysis](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#57-62), [MetaDescAnalysis](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#64-69), etc.)

## Phase 2: Domain Info — DomainOverview Rewrite
- [ ] Rewrite [DomainOverview.svelte](file:///home/drvoid/ISU/WebQ/src/lib/components/intelligence/domain-info/DomainOverview.svelte) with full metric grid (Network, Server, WHOIS, DNS sections)
- [ ] Display: `ipv4`, `all_ipv4`, `ipv6`, [reverse_dns](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#236-249), `web_server`, [http_status](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#629-656), `response_time_ms`
- [ ] Display: `registrar`, `registrant`, `creation_date`, `expiry_date`, `last_updated`, `privacy_protection`, `name_servers`, `domain_status`

## Phase 3: Domain Info — SSL Card Rewrite
- [ ] Rewrite [SslStatus.svelte](file:///home/drvoid/ISU/WebQ/src/lib/components/intelligence/domain-info/SslStatus.svelte) to show all 7 SSL fields
- [ ] Display: [status](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#629-656) badge, `issued_to`, `issuer`, `protocol_version`, `expiry_date`, `days_until_expiry` gauge, `alternative_names` list

## Phase 4: Domain Info — Security Details + Page Update
- [ ] Create `SecurityDetails.svelte` (HTTPS available, redirect, security headers checklist)
- [ ] Update [domain-info/+page.svelte](file:///home/drvoid/ISU/WebQ/src/routes/intelligence/domain-info/+page.svelte): import `SecurityDetails`, replace `any` with typed [DomainInfoResult](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#74-90), add error toast

## Phase 5: Domain DNS — Minor Enhancements
- [ ] Update [domain-dns/+page.svelte](file:///home/drvoid/ISU/WebQ/src/routes/intelligence/domain-dns/+page.svelte): show `timestamp` and `response_time_ms` in header, replace `any` type

## Phase 6: SEO Analysis — Complete Rewrite
- [ ] Fix property keys in [seo-analysis/+page.svelte](file:///home/drvoid/ISU/WebQ/src/routes/intelligence/seo-analysis/+page.svelte) ([basic](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#298-357) → [basic_seo](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#298-357), etc.)
- [ ] Rewrite [BasicSeoOverview.svelte](file:///home/drvoid/ISU/WebQ/src/lib/components/intelligence/seo-analysis/BasicSeoOverview.svelte) to match crate's nested [TitleAnalysis](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#57-62)/[MetaDescAnalysis](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#64-69) structs + add `canonical_url`, `meta_robots`, `viewport`, `language`, [charset](file:///home/drvoid/Qix/web-analyzer/src/seo_analysis.rs#382-404)
- [ ] Rewrite [TechnicalSeoCard.svelte](file:///home/drvoid/ISU/WebQ/src/lib/components/intelligence/seo-analysis/TechnicalSeoCard.svelte) with correct fields (`page_size_bytes`, [http_status](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#629-656), etc.)
- [ ] Rewrite [SocialMediaCard.svelte](file:///home/drvoid/ISU/WebQ/src/lib/components/intelligence/seo-analysis/SocialMediaCard.svelte) for HashMap-based `open_graph`/`twitter_cards`
- [ ] Create `ContentAnalysis.svelte` — headings, word count, keyword density
- [ ] Create `AnalyticsTracker.svelte` — tracking tool detection grid
- [ ] Create `PerformanceCard.svelte` — load time, compression, cache
- [ ] Create `MobileAccessibility.svelte` — viewport, alt coverage, ARIA
- [ ] Create `SeoResourcesCheck.svelte` — robots.txt, sitemap.xml, etc.
- [ ] Create `SchemaMarkup.svelte` — JSON-LD types, microdata
- [ ] Create `LinkAnalysis.svelte` — total/internal/external/nofollow links
- [ ] Create `ImageSeoCard.svelte` — image count, lazy-load, alt/title, score
- [ ] Create `PageSpeedFactors.svelte` — CSS/JS files, inline styles/scripts
- [ ] Create `SeoScoreGauge.svelte` — score, percentage, grade
- [ ] Update [seo-analysis/+page.svelte](file:///home/drvoid/ISU/WebQ/src/routes/intelligence/seo-analysis/+page.svelte) with all 13 section imports + responsive grid layout

## Phase 7: Crate Bug Fix
- [ ] Fix [web-analyzer/src/domain_info.rs](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs) [check_ssl()](file:///home/drvoid/Qix/web-analyzer/src/domain_info.rs#457-557) to compute `days_until_expiry` from parsed `expiry_date`

## Phase 8: i18n & Polish
- [ ] Add new message keys to [messages/en.json](file:///home/drvoid/ISU/WebQ/messages/en.json) for all new section titles and labels
- [ ] Add corresponding keys to [messages/tr.json](file:///home/drvoid/ISU/WebQ/messages/tr.json)
- [ ] Update [TODO.md](file:///home/drvoid/ISU/WebQ/TODO.md) progress statistics

## Verification
- [ ] `cargo build --all-features` in web-analyzer (crate fix)
- [ ] `cargo build` in WebQ/src-tauri (backend)
- [ ] `npm run check` in WebQ (frontend type-check)
- [ ] Commit and push to feature branch
- [ ] Telegram progress report
