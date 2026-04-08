# WebQ Projesi Görev Takip Çizelgesi
📈 **Progress Statistics:** [0] done, [0] ongoing, [0] implemented, [0] skipped. Toplam Görev: ~14

## 1. Proje Hazırlık ve Konfigürasyon
- [x] AGENTS.md ve GEMINI.md global AI kuralları tanımlandı.
- [x] Git depositörü oluşturuldu ve remote conf yapıldı.
- [ ] Tauri (Rust) v2 ve Svelte 5 iskeleti kurulacak.
- [ ] Tailwind CSS v4 ve standart UI elementleri (Shadcn/Bits/Qix Style) entegre edilecek.

## 2. Backend Geliştirme (Rust & web-analyzer)
- [ ] `web-analyzer` crate'i projeye dahil edilecek (`cargo add web-analyzer`).
- [ ] Intelligence Gathering Modülleri: `domain-info`, `domain-dns`, `seo-analysis` vb. için Tauri komutları (commands) hazırlanacak.
- [ ] Reconnaissance Modülleri: `subdomain-discovery`, `contact-spy` vb. için Tauri komutları hazırlanacak.
- [ ] Security Assessment Modülleri: `security-analysis`, `subdomain-takeover`, `cloudflare-bypass`, `api-security-scanner` için entegrasyonlar yazılacak.
- [ ] Tarama (Scan) durumu, ilerleme yüzdesi ve log akışı için periyodik event (`Window::emit`) altyapısı oluşturulacak.

## 3. Frontend Geliştirme (Svelte 5)
- [ ] Siber Güvenlik temalı (Cyber-Neon, Obsidian, Glassmorphism) Dashboard iskeleti ve Layout kodlanacak.
- [ ] Zafiyet raporlama (Vulnerabilities) ve DNS/WHOIS verisi tabloları spesifik Svelte componentleri haline getirilecek.
- [ ] Eşzamanlı (Real-time) arama ve analiz izleme log penceresi (Console / Terminal component) eklenecek.
- [ ] Tarama aşamalarındaki modüller için etkileşimli Skeleton / Spinner (bekleme) tasarımları oluşturulacak.

## 4. Yayınlama ve Deployment
- [ ] Github pro ayarları (Branch protection, issues, vb) repo üzerinden onaylanacak.
- [ ] WebQ cross-platform derlemeleri (Linux, macOS, Windows) için CI süreçleri ve testler hazırlanacak.
