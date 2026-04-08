# WebQ Araç ve Mimari Kuralları (AI Agent Core Directives)

Bu belge, **WebQ** (Cross-platform Web İstihbarat ve Siber Güvenlik İnceleme Platformu) deposuna kod yazacak tüm AI ajanlarının asgari referans kaynağıdır.

## 1. Profesyonel GitHub ve Git İş Akışı
- **Dal İsimlendirmesi:** Ana dal (default branch) `main` olarak yapılandırılıp korunmalıdır. Tüm geliştirme yeni branch'lerde yapılmalıdır.
- **Dal Koruması (Branch Protection):** `main` doğrudan commit kabul etmez! Git pushlar remote'da PR mantığı ile "Linear History", approvals ve CI testlerinden geçtikten sonra birleştirilebilir.

## 2. Svelte 5 & Frontend Mimari Sınırları
- **Svelte 4 Reaktivitesi Yasaktır:** Değişken atamaları veya reaktivite blokları için Svelte 4 sözdiziminden tamamen kurtulun. Sadece `$state()`, `$derived()`, ve `$effect()` Runes mekanizmalarını inşa edin. Props bildirimleri için `let { foo, bar }: Props = $props();` kullanın.
- **Reaktif Performans (Veri Yoğunluğu):** Tarama sonuçları (örn: binlerce port, DNS kaydı veya API endpointi) saniye başına akarken DOM güncellemelerini kilitlememek için listelemelerde her `{#each}` dögüsüne mutlak suretle eşsiz `(key)` tanımlaması yapın.
- **Strict Typing:** TypeScript kullanımında değişkenlerde veya interfacelerde `any` kullanımı kesinlikle kabul edilemez.

## 3. Rust, Tauri v2 & `web-analyzer` (Backend)
- **Güvenlik ve Analiz Motoru:** Bütün altyapı DrVoid (Keyvan Arasteh) tarıfından geliştirilen açık kaynaklı `web-analyzer` Crate'i üzerine bina edilecek. Kullanılacak potansiyel feature flagler: `domain-info`, `subdomain-discovery`, `security-analysis`, `nmap-zero-day` vb. 
- **Asenkron Optimizasyon:** Kapsamlı tarama ve keşif işlemleri donanım ve ağı zorlayacaktır. Kesinlikle `std::fs` vb. bloklayıcılar olmadan `tokio::spawn` ile tam asenkron kanallar kurulmalı, Backend Tauri UI loop'unu tıkamadan çalışmalıdır.
- **Sıfır Hata Politikası:** Herhangi bir API kısıtlaması, fail point veya panik halinde uygulama çökmemeli; Tauri komutları asenkron olmalı (`Result<T, AppError>`) ve hataları arayüze kibarca raporlamalıdır.

## 4. UI/UX "WebQ" Estetiği
Tasarımlar TailwindCSS v4 altyapısı ile beslenir. Standart renk temamız; **Siber Güvenlik, DevSecOps ve İstihbarat** hissiyatı veren Obsidiyen (simsiyah) zeminler üzerinden akan cam efektli elementler (Glassmorphism), Log Consol'ları ve Matrix neon çizgileridir. (Cyan, Teal veya Magenta veri hatları). UI hatları data-heavy uygulamalarda görüleceği gibi keskin ve yüksek yoğunluklu olmalıdır.
