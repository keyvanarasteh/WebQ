# WebQ Projesi Görev Takip Çizelgesi
📈 **Progress Statistics:** [14] done, [3] ongoing, [0] implemented, [0] skipped. Toplam Görev: ~285

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

### 2.2 Domain DNS Modülü (`domain_dns`)
- [ ] Arka plan: A, AAAA, MX, NS, SOA, TXT, CNAME query'lerini çeken Tauri Command.
- [ ] UI: DNS Kayıtları (Records) için sekmeli (Tabbed) bir görünüm eklenecek.
- [ ] Component: TXT kayıtları (SPF, DMARC) güvenlik doğrulamaları özel badge'lerle gösterilecek.
- [ ] Component: SOA ve MX sunucularının listeleneceği satır renkli Data Grid eklenecek.
- [ ] Frontend: DNS çözümleme esnasında "Resolving..." skeleton loader'ları geliştirilecek.
- [ ] Export: DNS Kayıtlarının JSON formatında kopyalanması için "Copy to Clipboard" butonu.

### 2.3 SEO Analysis Modülü (`seo_analysis`)
- [ ] Arka plan: 13 kategorilik SEO tarayıcısını başlatan komut yazılacak.
- [ ] UI: SEO Overview kartları (Performance, Accessibility, Best Practices, SEO).
- [ ] Component: Robot.txt ve Sitemap.xml dedektörleri UI'da gösterilecek.
- [ ] Component: Meta etiketleri, OpenGraph ve Twitter card analizlerini gösteren Accordion eklenecek.
- [ ] Component: Schema yönlendirme (Markup) tabloları.
- [ ] Component: 13 tracking aracı (Google Analytics, Pixels vb.) tespiti için liste.
- [ ] Frontend: Sayfadaki H1-H6 hiyerarşisini görselleyen ağaç (Tree) view.

### 2.4 Web Technologies Modülü (`web_technologies`)
- [ ] Arka plan: Teknolojik yığını parmak izini çıkaran Tauri Command yazılacak.
- [ ] State: Gelen verideki 16 kategorinin parse edilmesi işlemleri.
- [ ] UI: Kategoriler (Server, Backend, Frontend, CMS vs.) için Flex/Wrap Layout.
- [ ] Component: Her bir teknoloji (React, Nginx, PHP vb.) için teknoloji ikonları barındıran Badge (Çip) bileşenleri.
- [ ] Component: WordPress özel analizi (Tema, Plugin, User Enumeration) için ayrı bir Detail Modal eklenecek.
- [ ] Component: Tespit edilen CDN (Cloudflare, Akamai vb.) ve E-Ticaret altyapılarının (Shopify, Magento) uyarıları.
- [ ] Frontend: Parmak izi alınamadığında "Unknown/Not Detected" fallback state tasarımları.
- [ ] Veri: Bulunan teknolojilerin CVSS vulnerability listesiyle cross-check yapılması.

### 2.5 Domain Validator Modülü (`domain_validator`)
- [ ] Arka plan: Bulk domain yükleme ve paralel test `tokio` kanallarına aktarımı.
- [ ] UI: Dosyadan içeri aktarma (.txt, .csv) için Drag & Drop alanı.
- [ ] Component: Paralel doğrulama (DNS + HTTP + SSL) süreçlerini gösteren Canlı Progress Bar.
- [ ] Component: Bulk operasyon sonuçlarını listeleyen Virtualized List Veya Data Table.
- [ ] Component: Taranan domainlerin canlı olarak Geçerli (Valid), Geçersiz (Invalid), Atlandı (Skipped) durum badge'leri.
- [ ] Frontend: 34 skip limit aşımı uyarısı (Rate-limit) toast bildirimleri.

## Faz 3: Reconnaissance (Keşif ve Zafiyet Tespiti)
### 3.1 Subdomain Discovery Modülü (`subdomain_discovery`)
- [ ] Arka plan: `subfinder` çağrısını yapan, asenkron Subdomain keşfi başlatan Komut.
- [ ] State: TLD parçalama işlemleri ve hiyerarşik JSON dönüşümü.
- [ ] UI: Bulunan subdomain'leri Liste ve File-Tree hiyerarşisinde render edilmesi.
- [ ] Component: Kapsamlı (Wildcard) tespit edilen hedeflerin yıldızla (*) belirtilmesi.
- [ ] Component: Keşfedilen her subdomain satırında "Tarama Başlat" hızlı ping aksiyon butonu.
- [ ] Frontend: Alt alan adı bulunma animasyonu (Matrix style stream).
- [ ] Export: Subdomain listesinin txt olarak çekilmesi.

### 3.2 Contact Spy Modülü (`contact_spy`)
- [ ] Arka plan: BFS Crawl motorunu başlatan ve contact extraction yapan komut.
- [ ] UI: Hedef siteden Email, Telefon numaraları ve Sosyal Medya (15 platform) linklerini ayrıştıran Grid.
- [ ] Component: E-postalarda "Role-based" (info@, admin@) vs. "Personal" ayrımını görselleştiren liste.
- [ ] Component: Açık kaynaklı istihbarattan (OSINT) elde edilen sosyal profil icon box'ları.
- [ ] Frontend: Crawling (tarama) sırasında BFS deepth-level (Derinlik seviyesi) göstergesi.
- [ ] Veri: Çekilen numaraların format doğrulama işaretleri (Checkmark).

### 3.3 Advanced Content Scanner (`advanced_content_scanner`)
- [ ] Arka plan: 24 secret paterni ve dizin (directory) tarayıcısını entegre eden komut.
- [ ] UI: Çok yoğun veriyi kilitlenmeden gösterecek Console/Terminal Component tasarımı.
- [ ] Component: Hassas dosya (Sensitive files) (.env, .git, config) uyarıları için Kırmızı Kritik Alert barları.
- [ ] Component: SSRF ve JS Vuln testlerinin çıktılarının döküleceği satır renkli log tablosu.
- [ ] Frontend: Scroll-lock (Auto-scroll to bottom) özelliği olan Log Stream.
- [ ] Export: Açıkların "Vulnerability Report" biçiminde Dışa Aktarımı.

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
