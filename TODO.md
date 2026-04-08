# WebQ Projesi Görev Takip Çizelgesi
📈 **Progress Statistics:** [21] done, [0] ongoing, [0] implemented, [0] skipped. Toplam Görev: ~286

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
- [ ] Arkaplan: Rust Tauri Command `scan_domain_info` fonksiyonu `web_analyzer::domain_info`'yu asenkron Tokio thread'inde çağıracak. Hata yönetimi Paraglide ile lokalize edilip Svelte-Sonner toast olarak verilecek.

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
- [ ] Sayfa: `src/routes/recon/subdomain-discovery/+page.svelte` (Tabbed View)
  - *Input:* Hedef Domain
  - *Output:* Tauri'den dönen `SubdomainInfoResult` objesi (liste ve ağaç).
  - *Visualize:* Karanlık uzay/Neon UI ile Subdomain Tree (Hiyerarşik Dosya Sistemi görünümü) ve List View özellikleri.
- [ ] Component: `src/lib/components/recon/subdomain-discovery/SubdomainTree.svelte`
  - *Input:* TLD parçalanmış hiyerarşik JSON dönüşümü.
  - *Output:* Collapsible (Daraltılabilir) ağaç yapısında subdomain'ler.
  - *Visualize:* Her node için icon, wildcard (*) gösterimi, ağaç dalları çizgileri.
- [ ] Component: `src/lib/components/recon/subdomain-discovery/SubdomainGrid.svelte`
  - *Input:* Keşfedilen tüm subdomain'lerin düz listesi.
  - *Output:* "Tarama Başlat", "Ping", "Copy" hızlı aksiyon butonları olan tablo satırları.
  - *Visualize:* Her subdomain satırında animasyonlu (Matrix style stream) bulunma göstergesi.
- [ ] Component: `src/lib/components/recon/guides/SubdomainGuide.svelte`
  - *İçerik:* SecOps ve Güvenli web geliştirme açısından açıkta kalan subdomain'lerin (Dangling Subdomain Takeover) tehlikeleri.
- [ ] Arkaplan: Rust Tauri Command `scan_subdomains` asenkron Subdomain keşfi.
- [ ] İhracat: Subdomain listesinin txt ve JSON olarak dışa aktarımı.
- [ ] Multi-language: Eğitim modalları dahil tüm statik metinlerin `m.*` ile (Paraglide-js EN/TR) çevrilmesi.

### 3.2 Contact Spy Modülü (`contact_spy`) [Mimari ve UX Planı]
- [ ] Sayfa: `src/routes/recon/contact-spy/+page.svelte`
  - *Input:* Hedef URL / Toplu URL listesi.
  - *Output:* Tauri `ContactSpyResult` (E-postalar, Telefon Numaraları, Sosyal Profiller).
  - *Visualize:* Ekranın solunda OSINT kartları, sağında veri tablosu içeren Masonry Grid tasarımı.
- [ ] Component: `src/lib/components/recon/contact-spy/EmailHarvester.svelte`
  - *Input:* Çıkarılan e-posta listesi.
  - *Output:* "Role-based" (info@, admin@) vs "Personal" ayrımı yapan istatistiksel liste.
  - *Visualize:* Rol tabanlıysa kırmızı rozet (Hedefli Phishing riski!), kişiselse mavi rozet.
- [ ] Component: `src/lib/components/recon/contact-spy/SocialOsintBox.svelte`
  - *Input:* Çekilen sosyal medya bağlantıları (LinkedIn, Github, Twitter vb).
  - *Output:* Açık kaynak istihbarat profil bağlantıları.
  - *Visualize:* İlgili platformun marka ikonu (Lucide veya Custom SVG) ile aydınlatılmış (glow) icon box'ları.
- [ ] Component: `src/lib/components/recon/contact-spy/CrawlingConsole.svelte`
  - *Input:* Tarama logları ve derinlik (BFS Depth-Level).
  - *Output:* Canlı tarama hızı, işlem yapılan sayfa sayısı.
  - *Visualize:* Matrix temalı saniyede kayan console satırları.
- [ ] Component: `src/lib/components/recon/guides/ContactSpyGuide.svelte`
  - *İçerik:* Açıkta bırakılan personel bilgileri, Phishing vektörleri ve veri maskeleme eğitim materyali.
- [ ] Arkaplan: Rust BFS Crawl motorunu (tokio/reqwest) başlatan komut.
- [ ] Multi-language: Bileşenlerdeki statik string'lerin Paraglide (EN/TR) ile lokalize edilmesi.

### 3.3 Advanced Content Scanner (`advanced_content_scanner`) [Mimari ve UX Planı]
- [ ] Sayfa: `src/routes/recon/content-scanner/+page.svelte`
  - *Input:* URL ve Tarama Derinliği parametreleri.
  - *Output:* `ContentScanSummary` (Secret keys, exposed files, JS vulnerabilities).
  - *Visualize:* Terminal tabanlı, scroll-lock mekanizmalı log stream ağırlıklı karanlık hacker teması (Obsidian).
- [ ] Component: `src/lib/components/recon/content-scanner/SensitiveFilesAlert.svelte`
  - *Input:* Bulunan `.env`, `.git`, `config.php` gibi dosyalar.
  - *Output:* Kırmızı Alarm kritik uyarı barları (Alert Components).
  - *Visualize:* Kalp atışı (Pulse/Ping) animasyonu ile acil eylem gerektiren dosyaların vurgulanması.
- [ ] Component: `src/lib/components/recon/content-scanner/VulnerabilityLogTable.svelte`
  - *Input:* SSRF, JS analiz sonuçları.
  - *Output:* Renklendirilmiş (Kritik: Kırmızı, Medium: Sarı) güvenlik log tablosu.
  - *Visualize:* Hızlı arama / Filtreleme segmentleri olan Data Grid.
- [ ] Component: `src/lib/components/recon/guides/ContentScannerGuide.svelte`
  - *İçerik:* GitHub dorks, Exposed Environment Variables (Secret sızıntıları) risklerini anlatan educational modal.
- [ ] Arkaplan: 24 secret paterni (Regex) ve dizin bruit-force (Directory Busting) altyapısı entegrasyonu.
- [ ] İhracat: PDF/JSON formatında Vulnerability Report ihracatı.
- [ ] Multi-language: UI bileşenlerinin ve uyarıların Paraglide `m.*` dil dosyalarına eklenmesi.

## Faz 4: Security Assessment (Gelişmiş Güvenlik Değerlendirmesi)
### 4.1 Security Analysis Modülü (`security_analysis`)
- [ ] Arka plan: Güvenlik Postür (Grade A+..F) komutunun entegrasyonu.
- [ ] UI: Büyük, premium görünümlü "Overall Security Grade" Dashboard Kartı (LuxeCard).
- [ ] Component: WAF (7 provider) tespiti ve durumu (Bypass edilebilir mi?) analiz paneli.
- [ ] Component: CORS zafiyeti detayı ve Misconfiguration kod bloğu gösterimi.
- [ ] Component: Missing Security Headers (HSTS, CSP, X-Frame) uyarısı gösteren Checklist.
- [ ] Component: Zayıf Çerez (Cookies) Security, HttpOnly, SameSite analiz satırları.
- [ ] Frontend: Güvenlik skoru 50'nin altındaysa sayfanın kırmızı alarm durumuna (Red State) girmesi.

### 4.2 Subdomain Takeover Modülü (`subdomain_takeover`)
- [ ] Arka plan: 36 Servislik Fingerprint ve Takeover veritabanını tarayan komut.
- [ ] UI: Hangi subdomainin hangi serviste (AWS, Github Pages, Heroku, Azure vb.) açık unutulduğunu gösteren Grid.
- [ ] Component: Exploit Edilebilirlik (Difficulty: Easy, High, Edge case) rozetleri (Badges).
- [ ] Component: Takeover mitigation (çözüm yolu) önerilerini içeren Collapsible/Accordion bölümü.
- [ ] Frontend: Ele geçirilebilen bir alt alan bulunduğunda animasyonlu ve sesli (vibrational) bildirim.

### 4.3 Cloudflare Bypass Modülü (`cloudflare_bypass`)
- [ ] Arka plan: Internet geçmişi DNS'lerinden (History Lookup) orijin IP çözen özellik.
- [ ] UI: Bulunan potansiyel Orijin IP'leri ve "Bypass Status" paneli.
- [ ] Component: TCP Doğrulaması yapılmış ve başarılı olmuş IP adreslerini vurgulayan yeşil onay ikonu.
- [ ] Component: Tespit edilen Private IP adreslerinin (LAN) grileşmesi/filtrelenmesi.
- [ ] Frontend: DNS geçmiş arama animasyonu.

### 4.4 Nmap Zero Day Modülü (`nmap_zero_day`)
- [ ] Arka plan: Nmap entegrasyonu, XML çıktı okuyucu ve NVD CVE sorgusunu çalıştıran komut.
- [ ] UI: Siber güvenlik / Red Team görünümünde Port => Servis => Versiyon => CVE ağacı tablosu.
- [ ] Component: Nmap Service Versiyonuna bağlı Zero-Day veya genel Exploits listesi tablosu.
- [ ] Component: CVE skoru (CVSS 1-10) arası renklendirme (Low, Medium, High, Critical).
- [ ] Component: Exploit-DB Referans linkleri için dışarı yönlendiren Butonlar.
- [ ] Frontend: "Nmap çalışıyor..." süresi tahmini (ETA) Progress Bar.
- [ ] Veri: Çıkan CVE'lerin özetinin popover (Hover) ile detaylandırılması.

### 4.5 API Security Scanner (`api_security_scanner`)
- [ ] Arka plan: 9 Test Suiti (SQLi, XSS, SSRF vs.) paralel çalışan komut.
- [ ] UI: Geniş ekran API Güvenlik Paneli (Radar Chart ile Zafiyet Dağılımı).
- [ ] Component: SQL Injection açık bulguları gösteren Log arayüzü ve Payload Injection denemeleri.
- [ ] Component: XSS açıklarını ve tetikleyici parametreleri gösteren tablo.
- [ ] Component: Path Traversal veya LFI tespit panelinde, okunan örnek dizinlerin gösterimi (`/etc/passwd`).
- [ ] Component: Rate Limiting veya Header Injection zaafiyetlerini vurgulayan test satırları.
- [ ] Frontend: Payload'ların (Attack Vectors) tek tek denendiğini gösteren Fuzzing Speed göstergesi (Req/s).

### 4.6 Geo Analysis Modülü (`geo_analysis`)
- [ ] Arka plan: llms.txt, AI crawler direktifleri bulucu komutu entegrasyonu.
- [ ] UI: Geofencing verileri ve Sunucu lokasyon haritası (Map/Globe component opsiyonel).
- [ ] Component: Sitenin AI/LLM tarayıcılarına (ChatGPT-User, GPTBot vb.) izin verip vermediğini gösteren tablo.
- [ ] Component: WebMCP protokol uyumluluğuna dair rozet.

## Faz 5: Sistem Entegrasyonu, Olay Yönetimi ve Uygulama Ayarları
### 5.1 Olay Yönetimi (Event Handling)
- [ ] Payload verilerinin (JSON) parse hızını optimize eden Worker/WASM süreçleri entegresi (Gerekirse).
- [ ] `web-analyzer` loglarının Tauri `tracing` üzerinden alınıp Frontend'e stream akışı olarak yollanması.
- [ ] Svelte 5 `$effect` hook'ları ile Dashboard'daki Counter (Sayaç) animasyonlarının (NumberFlow) smooth yapılması.
- [ ] Backend Error'lerin Global Toast bileşenine (Svelte-sonner) bağlanması.

### 5.2 Raporlama ve Dışa Aktarma (Reporting)
- [ ] Markdown tabanlı otomatik rapor oluşturucu (`web-analyzer` çıktılarından).
- [ ] Tüm istihbarat ve zafiyet raporunu tek bir JSON yapısına derleyip indirme işlevi.
- [ ] Yöneticiler / C-Level için PDF biçiminde Minimalist Rapor üretimi.

### 5.3 UX Cila ve Mikro-Etkileşimler
- [ ] Hover Spotlight efektleri ve LuxeCard component optimizasyonu.
- [ ] Input (Hedef Domain) kısmına Validasyon (RegEx) eklenmesi.
- [ ] Klavye kısayolları (Ctrl+K Arama, Ctrl+Enter Tarama başlat) entegrasyonu.
- [ ] Tam ekran (F11) ve Always-on-Top (Her zaman üstte) Tauri API ayarları.

## Faz 6: Release & DevOps
- [ ] Nmap, Dig, Openssl bağımlılıklarının kontrol edilmesi, yoksalar UI üzerinden kullanıcıya "Install dependencies" bildirimi verilmesi.
- [ ] Linux AppImage build dosyaları için Icon ve Meta verilerinin ayarlanması.
- [ ] macOS (dmg) build süreçlerinde imzalama / M1 M2 ARM architecture ayarları.
- [ ] Windows (msi / nsis) derleme profillerinin oluşturulması.
- [ ] Github Actions (CI/CD) Workflow `release.yml` oluşturulması.
- [ ] Son güvenlik testleri ve 1.0.0 Release Yayını.
