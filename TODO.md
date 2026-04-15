# WebQ Projesi Görev Takip Çizelgesi
📈 **Progress Statistics:** [57] done, [0] ongoing, [0] implemented, [1] skipped. Toplam Görev: ~290

## Faz 1: Proje Temelleri ve Mimari Hazırlık
- [x] AGENTS.md ve GEMINI.md kuralları tamamlandı.
- [x] Git depositörü, Remote ve `.gitignore` ayarlandı.
- [x] Tauri v2 iskeleti kurulacak (`cargo tauri init`).
- [x] Svelte 5 (Vite) iskeleti kurulacak.
- [x] Tailwind CSS v4 ve typography/forms eklentileri ayarlanacak.
- [ ] Shadcn/Bits UI (veya Luxe UI / Qix UI) kütüphanesi Svelte 5'e uygun entegre edilecek.
- [x] `web-analyzer` bağımlılığı Cargo.toml'a (`all-features` ile) eklenecek.
- [x] `reqwest`, `tokio`, `serde_json` gibi temel Rust bağımlılıkları ayarlanacak.
- [x] Routing mimarisi (SvelteKit veya SPA Router) kurulacak.
- [x] Genel Siber Güvenlik Teması (Obsidian/Neon) Svelte global CSS'ine (app.css) eklenecek.
- [x] AppError (Tauri hata yönetimi) `Result<T, String>` kalıpları yazılacak.
- [x] Global Store (Runes `$state`) class'ları oluşturulacak (`AppState.svelte.ts`).
- [x] Tauri `Window::emit` dinleyicileri (Listener) için ortak Svelte fonksiyonları yazılacak.
- [x] Global Layout (Sidebar, Topbar) iskeleti oluşturulacak.
- [x] Dark Mode / Theme toggle butonu eklenecek.
- [x] Export (PDF, CSV, JSON) veri kaydetme mekanizmasının iskeleti yazılacak.
- [x] Settings (Ayarlar) sayfası ve config.json okuma/yazma servisleri hazırlanacak.

## Faz 2: Intelligence Gathering (İstihbarat)
### 2.1 Domain Info Modülü (`domain_info`) [Mimari ve UX Planı]
- [x] Sayfa: `src/routes/intelligence/domain-info/+page.svelte` (Ana Dashboard)
  - *Input:* Kullanıcıdan alınan hedef `domain` string'i.
  - *Output:* Tauri `invoke` aracılığıyla arka plandan gelen `DomainInfoResult` objesi (`JSON`).
  - *Visualize:* En üstte multi-language arama çubuğu, altında responsive (Mobile: 1 column, Desktop: 2-3 column) CSS Grid veri panelleri. Karanlık/Aydınlık tema ile duyarlı (adaptive) renkler.
- [x] Component: `src/lib/components/intelligence/domain-info/DomainOverview.svelte`
  - *Input:* `whois`, `ipv4`, `web_server`, `response_time_ms`.
  - *Output:* Registrar, Kayıt Tarihi, IP Lokasyonu, Hizmet Veren Sunucu Tipi (Nginx vs).
  - *Visualize:* Verileri "Key-Value" Metric Grid formatında (ikili satırlar halinde) gösteren Neon Glow kenarlıklı Luxe Card tasarımı.
- [x] Component: `src/lib/components/intelligence/domain-info/SslStatus.svelte`
  - *Input:* `ssl` objesi (`status`, `issuer`, `days_until_expiry`, `protocol_version`).
  - *Output:* Sertifika ömrü, yayınlayan otorite (Let's encrypt vb).
  - *Visualize:* Kalan gün sayısını 365 gün üzerinden oranlayan Dairesel (Doughnut) Gauge grafiği. Kalan güne göre yeşil, sarı veya kırmızı progres.
- [x] Component: `src/lib/components/intelligence/domain-info/PortSecurityMatrix.svelte`
  - *Input:* `open_ports`, `security`, `security_score`.
  - *Output:* 100 üzerinden animasyonlu genel güvenlik skoru, eksik HTTPS veya Firewall kuralları yüzünden tespit edilen açık port listesi.
  - *Visualize:* NumberFlow ile artan animasyonlu 0-100 skoru. Açık portlar için Grid: 80/443 yeşil, tehlikeli olanlar (21, 22, 3306) kırmızı yanıp sönen noktalar (ping animation).
- [x] Component (Advanced UX): `Domain Info` modülündeki tüm bileşenler (DomainOverview, SslStatus, PortSecurityMatrix, SecurityDetails) için monolithic scan'den bağımsız, progresif ve eşzamanlı çalışabilen Tauri command (Refresh) altyapısı kuruldu. `scanResult` verisini beklemeden asenkron component hydrate desteği sağlandı.
- [x] Arkaplan: Rust Tauri Command `scan_domain_info` fonksiyonu `web_analyzer::domain_info`'yu asenkron Tokio thread'inde çağıracak. Hata yönetimi Paraglide ile lokalize edilip Svelte-Sonner toast olarak verilecek. (Real-time MPSC terminal arayüzü ile tamamlandı)

### 2.2 Domain DNS Modülü (`domain_dns`) [Mimari ve UX Planı]
- [x] Sayfa: `src/routes/intelligence/domain-dns/+page.svelte` (Tabbed DNS View)
  - *Input:* Domain adı string'i.
  - *Output:* Tauri'den dönen `DomainDnsResult` (içinde `DnsRecords` objesi).
  - *Visualize:* Yatay Tabs menüsü (A, AAAA, MX, NS, SOA, TXT). İçerikler Data Grid şeklinde.
- [x] Component: `src/lib/components/intelligence/domain-dns/DnsRecordsBoard.svelte`
  - *Input:* Seçilen DNS türüne ait satır verileri (örn. `Vec<String>`).
  - *Output:* Düzenli IP Adresleri ve Record değerleri.
  - *Visualize:* Obsidian/Neon theme, scrollable overflow table.
- [x] Component: `src/lib/components/intelligence/domain-dns/DnsSecurityCheck.svelte`
  - *Input:* DMARC ve SPF değerleri.
  - *Output:* Güvenlik zafiyeti göstergeleri (DMARC policy missing vb).
  - *Visualize:* Kırmızı (Risk) / Yeşil (Safe) uyarı rozetleri (Badges).
- [x] Arkaplan: Rust Tauri Command `scan_domain_dns` fonksiyonu entegre edilecek.
- [ ] İhracat: Kayıtları JSON olarak panoya (Clipboard) kopyalama fonksiyonu `Cargo.toml -> tauri-plugin-clipboard-manager`.

### 2.3 SEO Analysis Modülü (`seo_analysis`) [Mimari ve UX Planı]
- [x] Sayfa: `src/routes/intelligence/seo-analysis/+page.svelte`
  - *Input:* Hedef Domain
  - *Output:* Tauri'den dönen `SeoAnalysisResult` objesi.
  - *Visualize:* Üstte `SeoScoreResult` Gauge Chart, altta Masonry Grid şeklinde alt paneller.
- [x] Component: `src/lib/components/intelligence/seo-analysis/BasicSeoOverview.svelte`
  - *Input:* `BasicSeoResult` (TitleAnalysis, MetaDescAnalysis, HeadingInfo).
  - *Output:* SEO Title, Meta Description uzunluğu, kelime yoğunluğu (KeywordInfo).
  - *Visualize:* Missing Meta Description gibi hataları vurgulayan kırmızı "!" checklist.
- [x] Component: `src/lib/components/intelligence/seo-analysis/TechnicalSeoCard.svelte`
  - *Input:* `TechnicalSeoResult` (Robots.txt, Sitemap.xml, Canonical URLs).
  - *Output:* Teknik SEO postürü.
  - *Visualize:* Luxe Card içerisinde True/False bool checkmark ikonları.
- [x] Component: `src/lib/components/intelligence/seo-analysis/SocialMediaCard.svelte`
  - *Input:* `SocialMediaResult` (OpenGraph, Twitter Cards).
  - *Output:* Paylaşıldığında görünecek sosyal medya önizlemesi (Preview Box mock).
- [x] Arkaplan: Rust Tauri Command `scan_seo_analysis` modülü eklenecek.

### 2.4 Web Technologies Modülü (`web_technologies`) [Mimari ve UX Planı]
- [x] Sayfa: `src/routes/recon/web-technologies/+page.svelte`
  - *Input:* URL / Domain
  - *Output:* Tauri `WebTechResult` parse edilecek.
- [x] Component: `src/lib/components/recon/web-technologies/TechStackGrid.svelte`
  - *Input:* Bulunan framework, dil ve sunucu listeleri.
  - *Output:* Nginx, PHP, React gibi teknoloji ağaçları.
  - *Visualize:* Her teknoloji için ikonlu neon badge grid.
- [x] Component: `src/lib/components/recon/web-technologies/WordPressScanner.svelte`
  - *Input:* `WordPressAnalysis` objesi ve `WpUser` listeleri.
  - *Output:* Versiyon tespiti, yüklü eklentiler/temalar ve enumeration user list.
- [x] Component: `src/lib/components/recon/web-technologies/SecurityHeadersList.svelte`
  - *Input:* `SecurityHeaderInfo`.
  - *Output:* Strict-Transport-Security, X-Frame-Options varlığı.
  - *Visualize:* Accordion halinde eksik header'ların CVSS skorlama tahminleri.
- [x] Arkaplan: Rust Tauri Command `scan_web_technologies` çağrısı entegre edilecek.

### 2.5 Domain Validator Modülü (`domain_validator`) [Mimari ve UX Planı]
- [x] Sayfa: `src/routes/recon/domain-validator/+page.svelte`
  - *Input:* Virgülle ayrılmış domain listesi veya txt dosyası yükleme (File Drop Zone).
  - *Output:* Toplu analiz çıktıları (`BulkValidationResult`).
- [x] Component: `src/lib/components/recon/domain-validator/ValidationStatsBar.svelte`
  - *Input:* `ValidationStats` (Total, valid, invalid, skipped).
  - *Output:* Canlı test progress bar.
  - *Visualize:* Percentage hesaplayan yeşil-kırmızı linear bar.
- [x] Component: `src/lib/components/recon/domain-validator/ValidationDataGrid.svelte`
  - *Input:* Her domain'in DNS, HTTP ve SSL validasyon durumu (`ValidationResult`).
  - *Output:* Table/Grid satırları.
  - *Visualize:* Her domain için 3 sütunlu "Checkmark" matrisi. Hatalı domain'ler soluklaşacak (opacity-50).
- [x] Arkaplan: Rust tarafında multi-threading (Tokio) kullanılarak toplu analiz başlatan `validate_bulk_domains` komutu eklenecek. Mümkünse stream (tauri event handler) üzerinden `%` (yüzde) bazlı aktarım sağlanacak.

### 2.6 Localization & Multi-Language
- [x] Paraglide-js EN/TR (Multi-language) entegrasyonu WebTech, Domain Validator ve SEO Analysis modülleri için tamamlanacak.

## Faz 3: Reconnaissance (Keşif ve Zafiyet Tespiti)
### 3.1 Subdomain Discovery Modülü (`subdomain_discovery`) [Mimari ve UX Planı]
- [x] Sayfa: `src/routes/recon/subdomain-discovery/+page.svelte` (Tabbed View: Tree vs Grid)
  - *Input:* Hedef Domain URL'i (Input Validation: geçerli bir FQDN olmalı).
  - *Output:* Tauri `Async` stream üzerinden akan `SubdomainRecord` objesi veya tam dönüş `SubdomainInfoResult` (JSON).
  - *Visualize:* Karanlık uzay/Neon UI ile "LuxeCard" kapsayıcıları. Ekran üstünde Progress/Scan Indicator.
- [x] Component: `src/lib/components/recon/subdomain-discovery/SubdomainTree.svelte`
  - *Input:* TLD parçalanmış, gruplanmış `$derived` hiyerarşik JSON yapısı.
  - *Output:* Svelte 5 `#snippet` recursive döngüsüyle inşa edilmiş daraltılabilir (collapsible) klasör/ağaç listesi.
  - *Visualize:* Her node için Lucide-svelte ikonu, wildcard (*) keşif tespitleri için mor "Glow" efekti. Ağaç dallarında Tailwind sınır çizgileri (border-l).
- [x] Component: `src/lib/components/recon/subdomain-discovery/SubdomainGrid.svelte`
  - *Input:* `$state<SubdomainRecord[]>` ile bağlanan (bound) keşfedilmiş liste.
  - *Output:* Host, IP Adresi, Cloud/CDN Provider bilgisi içeren Data tablosu. Tablo eylemleri: "Ping", "Copy", "Takeover Check".
  - *Visualize:* Tablo satırlarında (row) Bulunma sırasına göre animasyonlu giriş (transition:slide/fade).
- [x] Component: `src/lib/components/recon/guides/SubdomainGuide.svelte`
  - *İçerik:* Dangling Subdomain Takeover riskleri, Cloud DNS kaydı güvenliği ve SecOps eğitim materyali. LuxeUI modal ile açılacak.
- [x] Backend: Rust Tauri Command `scan_subdomains(domain: String) -> Result<SubdomainInfoResult, AppError>`. `tokio::spawn` kullanılarak `web-analyzer` internal sand-box crate'ine istek atılacak. Olası API rate-limit hataları AppError olarak UI'a paslanacak.
- [x] Multi-language: Eğitim modalları dahil tüm statik metinlerin `m.recon_subdomain_*` (Paraglide-js) ile çevrilmesi.

### 3.2 Contact Spy Modülü (`contact_spy`) [Mimari ve UX Planı]
- [x] Sayfa: `src/routes/recon/contact-spy/+page.svelte`
  - *Input:* Hedef URL / Toplu Domain listesi (Regex verifikasyonlu).
  - *Output:* Crate'den dönen `ContactSpyResult` (E-postalar, Telefon Numaraları, Sosyal Profiller, Vcards).
  - *Visualize:* Ekranın solunda OSINT profil kartları (LuxeCard), sağında verileri süzen Masonry Grid tasarımı.
- [x] Component: `src/lib/components/recon/contact-spy/EmailHarvester.svelte`
  - *Input:* Regex ile kazınmış `.edu`, `.gov`, `.com` vb. e-posta hash array.
  - *Output:* Sektörel rol sınıflandırması ("Role-based" örn: admin, info / "Personal" örn: isim.soyisim).
  - *Visualize:* Rol tabanlı e-postalar için kırmızı güvenlik ikaz rozeti (Hedefli Phishing riski), kişisel mail'ler için mavi OSINT rozeti.
- [x] Component: `src/lib/components/recon/contact-spy/SocialOsintBox.svelte`
  - *Input:* Bulunan sosyal platform URL'leri (LinkedIn, Github, Twitter, Instagram).
  - *Output:* Sosyal mühendislik (Social Engineering) profil kartları.
  - *Visualize:* İlgili platformun dinamik ikon renkleriyle renderlanmış aydınlatılmış (glow) icon box'ı (Örn: LinkedIn = Mavi parıltı).
- [x] Component: `src/lib/components/recon/contact-spy/CrawlingConsole.svelte`
  - *Input:* Tauri `Window::emit` tarafından gönderilen `CrawlStatus` payload'ları.
  - *Output:* Canlı tarama hızı (Req/s), mevcut inilen derinlik ağacı (BFS Depth-Level).
  - *Visualize:* Saniyede kayan hacker console satırları (Tailwind overflow-y-auto, font-mono, text-green-400). Otomatik auto-scroll (`$effect` ile).
- [x] Component: `src/lib/components/recon/guides/ContactSpyGuide.svelte`
  - *İçerik:* Phishing Vektörleri, Veri sızıntılarında (Breaches) açık personel veri istismarı eğitim serisi.
- [x] Backend: Rust motorunda BFS (Breadth-First-Search) crawler (reqwest/tokio) başlatan `crawl_for_contacts` komutu.

### 3.3 Advanced Content Scanner (`advanced_content_scanner`) [Mimari ve UX Planı]
- [x] Sayfa: `src/routes/recon/content-scanner/+page.svelte`
  - *Input:* Hedef URL, Tarama (Scan) Derinliği ve Wordlist profili seçimi (Small, Medium, Huge).
  - *Output:* `ContentScanSummary` modeli (Secret keys, Exposed dosyalar, JS bazlı API key tespitleri).
  - *Visualize:* Genişletilmiş terminal görünümü içeren rapor dashboard'u.
- [x] Component: `src/lib/components/recon/content-scanner/SensitiveFilesAlert.svelte`
  - *Input:* Tarayıcıdan tespit edilen kritik varyantlar (`.env`, `.git`, `config.php`, `docker-compose.yml`).
  - *Output:* Dinamik uyarılara sahip Critical Alert Box render'ı.
  - *Visualize:* Kalp atışı (animate-pulse) animasyonu ile acil eylem gerektiren dosyaların vurgulanması ve CVSS tahmini (10.0 Critical).
- [x] Component: `src/lib/components/recon/content-scanner/VulnerabilityLogTable.svelte`
  - *Input:* `$state` log history dizisi (SSRF, JS key sızıntıları).
  - *Output:* Zafiyet tipine ve skoruna göre renklendirilmiş (Kritik: Kırmızı, Medium: Sarı) güvenlik log veri gridi.
  - *Visualize:* Tabloda Dropdown (Bits UI dropdown) filtreleme opsiyonları (Örn: Sadece High severity göster).
- [x] Component: `src/lib/components/recon/guides/ContentScannerGuide.svelte`
  - *İçerik:* Google Dorks, Exposed Environment Variables riskleri, kaynak kodda sızan AWS key'lerinin istismarı hakkında SecOps notları.
- [x] Backend: Kritik 24 Secret paterni (Regex listesi) ve dizin tarayıcı (Directory Busting) altyapısı için `scan_content` Tauri Async fonksiyonu.
- [x] İhracat: Zafiyetleri PDF/JSON/CSV formatında sistem diskine export eden `export_scan_report` Rust hook'u.
- [x] Multi-language: Tüm statik UI bileşenlerinin ve eğitim kılavuzlarının `m.*` dil registry'si ile entegrasyonu.

## Faz 4: Security Assessment (Gelişmiş Güvenlik Değerlendirmesi)
### 4.1 Security Analysis Modülü (`security_analysis`)
- [x] Arka plan: Güvenlik Postür (Grade A+..F) komutunun entegrasyonu.
- [x] UI: Büyük, premium görünümlü "Overall Security Grade" Dashboard Kartı (LuxeCard).
- [x] Component: WAF (7 provider) tespiti ve durumu (Bypass edilebilir mi?) analiz paneli.
- [x] Component: CORS zafiyeti detayı ve Misconfiguration kod bloğu gösterimi.
- [x] Component: Missing Security Headers (HSTS, CSP, X-Frame) uyarısı gösteren Checklist.
- [x] Component: Zayıf Çerez (Cookies) Security, HttpOnly, SameSite analiz satırları.
- [x] Frontend: Güvenlik skoru 50'nin altındaysa sayfanın kırmızı alarm durumuna (Red State) girmesi.

### Faz 4.2 Subdomain Takeover Modülü (`subdomain_takeover`)
- [x] Arka plan: 36 Servislik Fingerprint ve Takeover veritabanını tarayan komut.
- [x] UI: Hangi subdomainin hangi serviste (AWS, Github Pages, Heroku, Azure vb.) açık unutulduğunu gösteren Grid.
- [x] Component: Exploit Edilebilirlik (Difficulty: Easy, High, Edge case) rozetleri (Badges).
- [x] Component: Takeover mitigation (çözüm yolu) önerilerini içeren Collapsible/Accordion bölümü.
- [x] Frontend: Ele geçirilebilen bir alt alan bulunduğunda animasyonlu ve sesli (vibrational) bildirim.

### Faz 4.3 Cloudflare Bypass Modülü (`cloudflare_bypass`)
- [x] Arka plan: Internet geçmişi DNS'lerinden (History Lookup) orijin IP çözen özellik.
- [x] UI: Bulunan potansiyel Orijin IP'leri ve "Bypass Status" paneli.
- [x] Component: TCP Doğrulaması yapılmış ve başarılı olmuş IP adreslerini vurgulayan yeşil onay ikonu.
- [x] Component: Tespit edilen Private IP adreslerinin (LAN) grileşmesi/filtrelenmesi.
- [x] Frontend: DNS geçmiş arama animasyonu.

### 4.4 Nmap Zero Day Modülü (`nmap_zero_day`)
- [x] Arka plan: Nmap entegrasyonu, XML çıktı okuyucu ve NVD CVE sorgusunu çalıştıran komut.
- [x] UI: Açık portlar ve Versiyon bilgilerini listeleyen Data Grid tablosu.
- [x] Component: CVE Zero-Day datagrid'i. (Description, Critical Score, ID verilerini listelemeli).
- [x] Component: Svelte derived logic ile skorların (CVSS) görsel renklendirmesinin entegrasyonu.
- [x] Component: Exploit-DB Referans linkleri için dışarı yönlendiren Butonlar.
- [-] Frontend: "Nmap çalışıyor..." süresi tahmini (ETA) Progress Bar. (Skipped: Nmap command execution blocks determinism for ETA)
- [x] Veri: Çıkan CVE'lerin özetinin popover (Hover) ile detaylandırılması.

### 4.5 API Security Scanner (`api_security_scanner`)
- [x] Arka plan: 9 Test Suiti (SQLi, XSS, SSRF vs.) paralel çalışan komut.
- [x] UI: Geniş ekran API Güvenlik Paneli (Radar Chart ile Zafiyet Dağılımı).
- [x] Component: SQL Injection açık bulguları gösteren Log arayüzü ve Payload Injection denemeleri.
- [x] Component: XSS açıklarını ve tetikleyici parametreleri gösteren tablo.
- [x] Component: Path Traversal veya LFI tespit panelinde, okunan örnek dizinlerin gösterimi (`/etc/passwd`).
- [x] Component: Rate Limiting veya Header Injection zaafiyetlerini vurgulayan test satırları.
- [x] Frontend: Payload'ların (Attack Vectors) tek tek denendiğini gösteren Fuzzing Speed göstergesi (Req/s).

### 4.6 Geo Analysis Modülü (`geo_analysis`)
- [x] Arka plan: llms.txt, AI crawler direktifleri bulucu komutu entegrasyonu.
- [x] UI: Sunucu lokasyonu, IP haritalandırma ve servis sağlayıcı detaylarını listeleyen interaktif Geofencing Map.
- [x] Component: LLM-Scraping Data (AI botlarına açık veri seti) sunan `GeofencingGuide.svelte` eğitim kartı.
- [x] Frontend: AI crawler erişimlerini simgeleyen bir Radar Chart veya Map konsepti çizimi.

## Faz 5: Sistem Entegrasyonu, Olay Yönetimi ve Uygulama Ayarları
### 5.1 Olay Yönetimi (Event Handling)
- [-] Payload verilerinin (JSON) parse hızını optimize eden Worker/WASM süreçleri entegresi (Gerekirse). details: Skipped, WASM overkill when Tauri backend is fast.
- [x] `web-analyzer` loglarının Tauri `tracing` üzerinden alınıp Frontend'e stream akışı olarak yollanması.
- [x] Svelte 5 `$effect` hook'ları ile Dashboard'daki Counter (Sayaç) animasyonlarının (NumberFlow) smooth yapılması.
- [x] Backend Error'lerin Global Toast bileşenine (Svelte-sonner) bağlanması.

### 5.2 Raporlama ve Dışa Aktarma (Reporting)
- [x] Markdown tabanlı otomatik rapor oluşturucu (`web-analyzer` çıktılarından).
- [x] Tüm istihbarat ve zafiyet raporunu tek bir JSON yapısına derleyip indirme işlevi.
- [x] Yöneticiler / C-Level için PDF biçiminde Minimalist Rapor üretimi.

### 5.3 UX Cila ve Mikro-Etkileşimler
- [x] Active page highlighting in Sidebar component (SvelteKit `$page.url.pathname`) and animation cuts for NeuralBootSequence (skip explode).
- [ ] Hover Spotlight efektleri ve LuxeCard component optimizasyonu.
- [ ] Input (Hedef Domain) kısmına Validasyon (RegEx) eklenmesi.
- [ ] Klavye kısayolları (Ctrl+K Arama, Ctrl+Enter Tarama başlat) entegrasyonu.
- [ ] Tam ekran (F11) ve Always-on-Top (Her zaman üstte) Tauri API ayarları.

## Faz 6: Release & DevOps
- [x] Nmap, Dig, Openssl bağımlılıklarının kontrol edilmesi, yoksalar UI üzerinden kullanıcıya "Install dependencies" bildirimi verilmesi.
- [x] Linux AppImage build dosyaları için Icon ve Meta verilerinin ayarlanması.
- [x] macOS (dmg) build süreçlerinde imzalama / M1 M2 ARM architecture ayarları.
- [x] Windows (msi / nsis) derleme profillerinin oluşturulması.
- [x] Github Actions (CI/CD) Workflow `release.yml` oluşturulması.
- [x] **[CI FIX] macOS Code Signing Failure:** Github Actions'ta `APPLE_CERTIFICATE` boş olmasına rağmen tauri-action'ın imzalamaya çalışıp çökmesini önlemek için release.yml içindeki Apple ortam değişkenleri yorum satırına alındı.
- [x] **[DEPS] Wildcard Versioning:** `web-analyzer` dependency wildcard (`*`) versiyonuna (Issue #5) doğru şekilde güncellendi.
- [ ] Son güvenlik testleri ve 1.0.0 Release Yayını.

## Faz 7: Scan History & SQLite Database Architecture (Offline Persistence)
### 7.1 Veritabanı Mimarisi & Şema Yapısı (Backend - SeaORM / Sqlx)
- [ ] Arkan plan: SQLite veritabanı entegrasyonu `src-tauri/src/db/` dizinine eklenecek. SQLite için `WAL` modu (Write-Ahead Logging) asenkron I/O performansını artırmak adına aktif edilecek.
- [ ] Schema 1: **`scans`** (Ana Tarama Oturum Tablosu)
  - `id`: UUID (Primary Key)
  - `target_domain`: String (Hedef adres e.g `example.com`, indekslenecek - `CREATE INDEX idx_scans_domain`)
  - `scan_module`: Enum (`SecurityAnalysis`, `DomainDns`, `SubdomainDiscovery`, `SeoAnalysis`, `NmapZeroDay`, `WebTech`, `ContactSpy`)
  - `status`: Enum (`Running`, `Completed`, `Failed`, `Cancelled`)
  - `error_message`: Option<String> (Tarama başarısız olursa stack-trace / neden hatası tutulacak)
  - `config_options`: Option<JSONB> (Kullanıcının tarama anında seçtiği konfigürasyon, örn: `{ "wordlist": "huge", "threads": 50 }`)
  - `is_favorite`: Boolean (Kullanıcının sonucu yıldızlaması/sabitlemesi için)
  - `duration_ms`: i64 (Taramanın ne kadar sürdüğü, performans istatistikleri için)
  - `started_at`: DateTime (Tarama başlama tarihi)
  - `finished_at`: Option<DateTime> (Tarama bitiş tarihi)
- [ ] Schema 2: **`scan_results`** (Detaylı Sonuç & İstatistik Tablosu)
  - `id`: UUID (Primary Key)
  - `scan_id`: Foreign Key (`scans.id` ile cascade delete, zorunlu constraint)
  - `security_score`: Option<i32> (Tarihsel Data-Grid Dashboardlarında hızlı line-chart çizimi için)
  - `critical_vulns`: i32 (Zero-day, Kritik açık sayıları - Tablo filtreleme için)
  - `high_vulns`: i32 (Yüksek riskli açık adedi)
  - `medium_vulns`: i32
  - `low_vulns`: i32
  - `raw_json_blob`: JSONB. Bu bloba doğrudan Serialize edilecek `web-analyzer` internal struct'ları:
    - **`SecurityAnalysisResult`**: (İçerisinde `WafMatch`, `HeaderAnalysis`, `SslAnalysisResult`, `CorsPolicyResult`, `VulnScanResult` node'ları var)
    - **`DomainDnsResult`**: (İçerisinde `DnsRecords` A, AAAA, MX, NS arrayleri)
    - **`SubdomainDiscoveryResult`**: (`host`, `ips`, `cdn_provider` listeleri)
    - **`SeoAnalysisResult`**: (`BasicSeoResult`, `TechnicalSeoResult`, `SocialMediaResult`, `HeadingInfo`)
    - **`WebTechResult`**: (`SecurityHeaderInfo`, `WordPressAnalysis`, `WpUser`, `CookieSecurityInfo`)
    - **`NmapScanResult`**: (`PortInfo`, `VulnerabilityInfo`, `SeverityInfo` - Zafiyet ağacı)
    - **`CloudflareBypassResult`**: (`FoundIp` array)
    - **`ContactSpyResult`**: (Emails, `SocialMedia` node'ları)
- [ ] Database Queries (Tauri Commands in `src-tauri/src/db.rs`):
  - `get_scans_paginated(limit, offset, filter_domain, filter_module, sort_by)`: Veri tablosu için Server-Side Pagination ve arama fonksiyonu.
  - `get_global_statistics()`: Son 30 günün ortalama security score'u, en çok test edilen 5 domain, module kullanım oranları, tespit edilen toplam zafiyetlerin `Group By` sorgusu (Frontend'deki Pie, Bar ve Line Chartlar için).
  - `get_scan_blob_details(id)`: UI, Dashboard Row açıldığında sadece gerekli olan bu ağır JSON blobu request edecek (Lazy loading logic).
  - `toggle_favorite(id)`: Scan'in favori statüsünü tetikleme.
  - `bulk_delete_scans(ids: Vec<UUID>)`: Seçili scan'leri veritabanından toplu kalıcı silme.
  - `export_scan_json(id)`: Json raw blobu disk üzerine yazdırma.

### 7.2 History Dashboard & Widgets (Frontend)
- [ ] Ana Sayfa (Main Dashboard) Widget: `src/routes/+page.svelte` içine "Recent Scans" veri tablosu mini-widget'ının (Son 5 tarama) eklenmesi.
- [ ] Sayfa: `src/routes/history/+page.svelte` Detaylı arama çubuğuna, tarihe, domain adına ve tarama modülüne (Nmap, DNS, ContactSpy) göre filtrelenebilir kompleks bir Scan History Data-Grid tablosu.
- [ ] Component: "Re-Scan" butonu içeren geçmiş scanleri tek tıkla tekrar çalıştırabilen etkileşimli satırlar.
- [ ] Component: Geçmiş scan detaylarını tıklayarak tam sayfa Dashboard Raporuna geri dönüştüren dinamik `/history/[id]/+page.svelte` tasarımı.

### 7.3 Mevcut Sayfa Modifikasyonları
- [ ] Zafiyet Raporlaması: Mevcut Recon ve Intelligence sayfalarında, sonuçlar ekrana basılırken arka planda asenkron olarak SQLite'a kaydolduğuna dair "Saved to DB" Svelte-Sonner onayı.
- [x] Ayarlar (Settings) Modifikasyonu: Veritabanını yedekleme (Export SQLite), veritabanı boyutunu görme ve komple sıfırlama (Nuke History) fonksiyonları için tehlike (Danger) butonları.

