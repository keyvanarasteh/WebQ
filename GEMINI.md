# WebQ Araç ve Mimari Kuralları (AI Agent Core Directives)

Bu belge, **WebQ** (Cross-platform Web İstihbarat ve Siber Güvenlik İnceleme Platformu) deposuna kod yazacak tüm AI ajanlarının asgari referans kaynağıdır.

## 1. Profesyonel GitHub ve Git İş Akışı
- **Dal İsimlendirmesi:** Ana dal (default branch) `main` olarak yapılandırılıp korunmalıdır. Tüm geliştirme yeni branch'lerde yapılmalıdır.
- **Dal Koruması (Branch Protection):** `main` doğrudan commit kabul etmez! Git pushlar remote'da PR mantığı ile "Linear History", approvals ve CI testlerinden geçtikten sonra birleştirilebilir.

## 2. Svelte 5 & Frontend Mimari Sınırları
- **Svelte 4 Reaktivitesi Yasaktır:** Değişken atamaları veya reaktivite blokları için Svelte 4 sözdiziminden tamamen kurtulun. Sadece `$state()`, `$derived()`, ve `$effect()` Runes mekanizmalarını inşa edin. Props bildirimleri için `let { foo, bar }: Props = $props();` kullanın.
- **Reaktif Performans (Veri Yoğunluğu):** Tarama sonuçları (örn: binlerce port, DNS kaydı veya API endpointi) saniye başına akarken DOM güncellemelerini kilitlememek için listelemelerde her `{#each}` dögüsüne mutlak suretle eşsiz `(key)` tanımlaması yapın.
- **Strict Typing:** TypeScript kullanımında değişkenlerde veya interfacelerde `any` kullanımı kesinlikle kabul edilemez.

## 2.1 Svelte 5 & SvelteKit Hata Çözüm Direktifleri (Rules)
Gelecek implementasyonlarda derleme (build) aşamasında çıkan Svelte hatalarını engellemek için aşağıdaki .agents kural dosyalarına mutlaka uyulmalıdır:
- **Missing Script Tags Hatası:** `file:///home/drvoid/ISU/WebQ/.agents/rules/svelte/script_tags_missing.md` (Dosyanın en üstünde logic katmanının `<script lang="ts">` ile sarmalanmaması durumu).
- **Paraglide-js Types Hatası:** `file:///home/drvoid/ISU/WebQ/.agents/rules/svelte/paraglide_language_types.md` (localStorage veya env verisinden gelen raw string'in Paraglide sistemine uygun literllere cast edilmesi).
- **`@const` Invalid Placement Hatası:** `file:///home/drvoid/ISU/WebQ/.agents/rules/svelte/const_invalid_placement.md` (Svelte 5'te `@const` etiketinin div vb. generic elementler içerisine değil, doğrudan {#each} altına veya `<script>` içerisinde `$derived` ile tanımlanması gereklidir).
- **`$app/paths` Module Not Found Hatası:** `file:///home/drvoid/ISU/WebQ/.agents/rules/svelte/sveltekit_tsconfig_paths.md` (Vite / SvelteKit projelerinde `tsconfig.json` içerisinde `"paths"` override edilmemeli, path alias'lar `svelte.config.js` üzerinden `kit.alias` bloğunda tanımlanmalıdır).

## 3. Rust, Tauri v2 & `web-analyzer` (Backend)
- **Güvenlik ve Analiz Motoru:** Bütün altyapı DrVoid (Keyvan Arasteh) tarıfından geliştirilen açık kaynaklı `web-analyzer` Crate'i üzerine bina edilecek. Kullanılacak potansiyel feature flagler: `domain-info`, `subdomain-discovery`, `security-analysis`, `nmap-zero-day` vb. 
- **Asenkron Optimizasyon:** Kapsamlı tarama ve keşif işlemleri donanım ve ağı zorlayacaktır. Kesinlikle `std::fs` vb. bloklayıcılar olmadan `tokio::spawn` ile tam asenkron kanallar kurulmalı, Backend Tauri UI loop'unu tıkamadan çalışmalıdır.
- **Sıfır Hata Politikası:** Herhangi bir API kısıtlaması, fail point veya panik halinde uygulama çökmemeli; Tauri komutları asenkron olmalı (`Result<T, AppError>`) ve hataları arayüze kibarca raporlamalıdır.

## 4. UI/UX "WebQ" Estetiği
Tasarımlar TailwindCSS v4 altyapısı ile beslenir. Standart renk temamız; **Siber Güvenlik, DevSecOps ve İstihbarat** hissiyatı veren Obsidiyen (simsiyah) zeminler üzerinden akan cam efektli elementler (Glassmorphism), Log Consol'ları ve Matrix neon çizgileridir. (Cyan, Teal veya Magenta veri hatları). UI hatları data-heavy uygulamalarda görüleceği gibi keskin ve yüksek yoğunluklu olmalıdır.

## 5. Algorithmic Processing & Planning (MANDATORY)
- **Görev ve Planlama**: Tüm aşamalar/ilerlemeler mutlaka Kesin Görevler (Tasks) ve yapılandırılmış bir Uygulama Planı (Implementation Plan) barındırmalıdır. 
- **TODO Güncellemeleri**: İlerledikçe `TODO.md` dosyası, kullanıcının bir sonraki adıma karar vermesine yardımcı olacak şekilde detayı artırılarak sürekli güncellenmelidir.
