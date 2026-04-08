# WebQ Projesi Görev Takip Çizelgesi
📈 **Progress Statistics:** [0] done, [0] ongoing, [0] implemented, [0] skipped. Toplam Görev: ~35

## 1. Proje Hazırlık ve Altyapı (Phase 1: Foundation)
- [x] AGENTS.md ve GEMINI.md global AI kuralları tanımlandı.
- [x] Git depositörü oluşturuldu, remote ayarları yapıldı ve `.gitignore` eklendi.
- [ ] Tauri v2 + Svelte 5 iskeleti kurulacak (Vite).
- [ ] Tailwind CSS v4, Shadcn/Bits UI ve ikon kütüphaneleri eklenecek.
- [ ] `web-analyzer` crate'i Cargo.toml'a tüm özellikleri aktif (`all-features`) şekilde eklenecek.
- [ ] Tauri IPC (Inter-Process Communication) altyapısı ve `AppError` -> `String` serializasyonları yazılacak.
- [ ] Svelte Runes ile global State Management (`$state`, Svelte sınıfları) kurgulanacak.

## 2. Intelligence Gathering Modülleri (Phase 2: Basic Level)
*Bu aşamadaki özellikler dışa bağımlılığı az olan, asenkron `reqwest` ve standart TCP isteklerine dayanan en temel istihbarat modülleridir.*

- [ ] **Domain DNS (`domain_dns`)**: DNS kayıtları (A, AAAA, MX, NS, SOA, TXT, CNAME) için arka plan Tauri komutu (command) ve frontend bileşeni (Data table) yazılacak.
- [ ] **Domain Info (`domain_info`)**: WHOIS sorguları, port taraması ve SSL sertifika çözümlemesi sisteme entegre edilecek.
- [ ] **Domain Validator (`domain_validator`)**: Toplu domain doğrulaması (DNS + HTTP + SSL) için Bulk Check arayüzü ve paralel `tokio` kanalları kurulacak.
- [ ] **SEO Analysis (`seo_analysis`)**: 13 kategorilik SEO puanlama motoru, şema işaretleri ve izleyici loglaması için skor göstergeleri (Gauge charts) tasarlanacak.
- [ ] **Web Technologies (`web_technologies`)**: 16 kategoride teknoloji (CMS, WAF, Analytics vs.) parmak izi tespiti için UI tag'leri (Badge components) eklenecek.

## 3. Reconnaissance Modülleri (Phase 3: Intermediate Level)
*Hafif seviyede disk I/O ve dış sistem entegrasyonları (subfinder vb.) gerektiren keşif araçları.*

- [ ] **Subdomain Discovery (`subdomain_discovery`)**: `subfinder` entegrasyonu, multi-part TLD parse işlemi ve UI'da ağaç (Tree) veya liste formatında render işlemi yapılacak.
- [ ] **Contact Spy (`contact_spy`)**: E-posta, telefon ve 15 farklı sosyal medya platformundan veri kazıma (BFS crawl) modülü entegre edilip veri tablo görünümü hazırlanacak.
- [ ] **Advanced Content Scanner (`advanced_content_scanner`)**: 24 gizli secret paterni, zafiyet scriptleri ve hassas dosya okuma taramaları log basacak şelale (Waterfall) log arayüzüyle eşlenecek.

## 4. Security Assessment Modülleri (Phase 4: Advanced Level)
*Kompleks asenkron akışlar, CVE sorguları ve aktif güvenlik testleri barındıran mimariler.*

- [ ] **Security Analysis (`security_analysis`)**: Güvenlik notlandırması (Grade A+ ile F), WAF, CORS ve session cookie analizleri için Dashboard performans kartları tasarlanacak.
- [ ] **Geo Analysis (`geo_analysis`)**: `llms.txt`, WebMCP ve AI bot scraper logları çözümlenip sisteme dökülecek.
- [ ] **Cloudflare Bypass (`cloudflare_bypass`)**: Origin IP çözülmesi (Tarihçe taraması, TCP kontrolü), IP eşleştirme animasyonları ve backend logic'leri bağlanacak.
- [ ] **Subdomain Takeover (`subdomain_takeover`)**: 36-servislik dev vulnerability veritabanı tarayıcısı UI'a dahil edilecek. Etkilenen sistemler vurgulu (Highlight) renkle raporlanacak.
- [ ] **Nmap Zero Day (`nmap_zero_day`)**: `nmap` süreçlerinin Tauri üzerinden tetiklenmesi, NVD CVE / Exploit-DB aramalarının eşlenmesi ve CVSS kritiklik grafiklerinin çizdirilmesi.
- [ ] **API Security Scanner (`api_security_scanner`)**: 9 büyük test süiti (SQLi, XSS, SSRF, Path Traversal vb.) modüller için aktif tarama (Active Scan) menüleri ve Terminal / Log Konsolu oluşturulacak.

## 5. Optimizasyon, Loglama ve Yayınlama (Phase 5: Release)
- [ ] Güvenlik Taramaları esnasında oluşan `Window::emit` (Tauri Event) bazlı ilerleme durumlarının (Progress Bars) Svelte'e bağlanması.
- [ ] Veri çokluğundan kaynaklı DOM çökmelerini engellemek için `{#each}` virtual-list limitleri / pagination (sayfalama) sisteminin eklenmesi.
- [ ] Linux, macOS ve Windows sürümleri için Security Pipeline (Github Actions) ve `.msi / .AppImage / .dmg` derleme profillerinin hazırlanması.
