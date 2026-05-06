<script lang="ts">
  import { XIcon, BookOpen, ArrowRight, Activity, ShieldAlert, Cpu, Server, Shield } from 'lucide-svelte';
  import { languageTag } from '$lib/paraglide/runtime';

  let { 
      isOpen = $bindable(false),
      section = "overview"
  } : {
      isOpen: boolean;
      section: "overview" | "vectors" | "profiling" | "simulation";
  } = $props();

  let lang = $derived(languageTag() as "en" | "tr");

  function close() {
      isOpen = false;
  }
  
  $effect(() => {
      const handleEscape = (e: KeyboardEvent) => {
          if (e.key === 'Escape' && isOpen) close();
      };
      
      if (isOpen) document.addEventListener('keydown', handleEscape);
      return () => document.removeEventListener('keydown', handleEscape);
  });

  const content = {
      en: {
          overview: {
              title: "Architecture Overview",
              desc: "A realistic React Server Components (RSC) honeypot that silently detects 45+ real attack payload categories while collecting comprehensive attacker intelligence.",
              flow: [
                  { actor: "Attacker", action: "HTTP Request", target: "Honeypot Engine" },
                  { note: "Extracts headers, body, query, path, IP" },
                  { actor: "Honeypot Engine", action: "Build/update fingerprint", target: "Profiling Engine", color: "blue" },
                  { note: "Parses User-Agent → Browser/OS\nTracks request cadence (automated?)" },
                  { actor: "Honeypot Engine", action: "Scan against 45+ attack vectors", target: "Detection Layer", color: "red" },
                  { note: "Matches regex patterns, validates context keywords\nCalculates confidence score & classifies severity" },
                  { actor: "Detection Layer", action: "Vec<AttackEvent>", target: "Honeypot Engine", color: "red" },
                  { actor: "Honeypot Engine", action: "Update profile with detections", target: "Profiling Engine", color: "blue" },
                  { actor: "Honeypot Engine", action: "Simulate realistic response", target: "Simulation Layer", color: "green" },
                  { actor: "Honeypot Engine", action: "DetectionResult (status, body, delay)", target: "Attacker", color: "gray" }
              ]
          },
          vectors: {
              title: "45+ Attack Vector Categories",
              desc: "Detects a comprehensive array of modern and legacy web vulnerabilities tailored for RSC endpoints.",
              sections: [
                  {
                      q: "SQL & NoSQL Injection",
                      a: "Detects Tautologies, Union Selects, Blind/Time-Based, Error-Based, and Stacked Queries. Also detects MongoDB and Redis injections."
                  },
                  {
                      q: "Cross-Site Scripting (XSS) & Command Injection",
                      a: "Identifies Reflected, Polyglot, and Stored XSS payloads. Spots Unix Pipe, Windows CMD, and Blind OOB Command Injections."
                  },
                  {
                      q: "Deserialization & RSC Attacks",
                      a: "Detects Java, PHP, Python Pickle, and Node.js deserialization payloads. Specifically hunts for RSC Flight Protocol injection and Server Action probes."
                  },
                  {
                      q: "SSRF, XXE & SSTI",
                      a: "Identifies Cloud Metadata SSRF, Internal Port probing, DNS Rebinding, External Entity XXE, and Template Injection across Jinja2, Twig, and Freemarker."
                  }
              ]
          },
          profiling: {
              title: "Attacker Profiling & Intelligence",
              desc: "Collects behavioral profiles to identify automated scanners, headless browsers, and determined adversaries.",
              sections: [
                  {
                      q: "What Intelligence is Collected?",
                      a: "IP addresses, Browser/OS fingerprints from User-Agents, Request timing (cadence), Attack payloads, Target endpoints, and Headless browser detection (e.g., Puppeteer, Selenium)."
                  },
                  {
                      q: "How is Risk Calculated?",
                      a: "A 0-100 Risk Score is calculated based on the severity of the attack categories (e.g., SQLi/RCE = 15 points), technique diversity, automation bonuses, and request volume."
                  },
                  {
                      q: "Automation Detection",
                      a: "Bots and scripts are identified by analyzing inter-request intervals, speed thresholds (<100ms), headless User-Agent markers, and the rapid deployment of diverse techniques."
                  }
              ]
          },
          simulation: {
              title: "Realistic RSC Simulation",
              desc: "Generates plausible React Server Components responses with timing jitter to keep attackers engaged without tipping them off.",
              sections: [
                  {
                      q: "Fake Flight Protocol Responses",
                      a: "The honeypot returns realistic RSC wire format payloads, such as:\n`0:[\"$\",\"@2\",null,{\"id\":\"__PAGE__\",\"children\":[...]}]`\n`1:{\"status\":\"resolved\",\"data\":...}`"
                  },
                  {
                      q: "Fake Endpoints Exposed",
                      a: "Simulates high-value endpoints including `/` (text/x-component), `/api/graphql`, `/api/auth/callback`, `/_rsc/__PAGE__`, and `/dashboard`."
                  },
                  {
                      q: "Timing & Progressive Sizing",
                      a: "Introduces random jitter (20–180ms) per response and gradually increases response sizes to simulate real backend processing and retain attacker engagement."
                  }
              ]
          }
      },
      tr: {
          overview: {
              title: "Mimariye Genel Bakış",
              desc: "45'ten fazla gerçek saldırı yükü kategorisini sessizce tespit ederken kapsamlı saldırgan istihbaratı toplayan gerçekçi bir React Server Components (RSC) bal küpü.",
              flow: [
                  { actor: "Saldırgan", action: "HTTP İsteği", target: "Bal Küpü Motoru" },
                  { note: "Başlıkları, gövdeyi, sorguyu, yolu, IP'yi çıkarır" },
                  { actor: "Bal Küpü Motoru", action: "Parmak izi oluştur/güncelle", target: "Profilleme Motoru", color: "blue" },
                  { note: "User-Agent ayrıştırması → Tarayıcı/İS\nİstek ritmini izler (otomatik mi?)" },
                  { actor: "Bal Küpü Motoru", action: "45+ saldırı vektörüne karşı tara", target: "Tespit Katmanı", color: "red" },
                  { note: "Regex eşleştirme, bağlam doğrulama\nGüven puanı hesaplama ve şiddet sınıflandırması" },
                  { actor: "Tespit Katmanı", action: "Vec<AttackEvent>", target: "Bal Küpü Motoru", color: "red" },
                  { actor: "Bal Küpü Motoru", action: "Tespitle profili güncelle", target: "Profilleme Motoru", color: "blue" },
                  { actor: "Bal Küpü Motoru", action: "Gerçekçi yanıt simüle et", target: "Simülasyon Katmanı", color: "green" },
                  { actor: "Bal Küpü Motoru", action: "DetectionResult (durum, gövde, gecikme)", target: "Saldırgan", color: "gray" }
              ]
          },
          vectors: {
              title: "45+ Saldırı Vektörü Kategorisi",
              desc: "RSC uç noktalarına özel olarak tasarlanmış modern ve eski web zafiyetlerini kapsayan geniş bir dizi tespit eder.",
              sections: [
                  {
                      q: "SQL & NoSQL Enjeksiyonu",
                      a: "Tautolojileri, Union Select, Kör/Zaman Tabanlı, Hata Tabanlı ve Yığılmış Sorguları tespit eder. Ayrıca MongoDB ve Redis enjeksiyonlarını yakalar."
                  },
                  {
                      q: "Cross-Site Scripting (XSS) & Komut Enjeksiyonu",
                      a: "Yansıyan, Poliglot ve Saklanan XSS yüklerini tanımlar. Unix Pipe, Windows CMD ve Kör OOB Komut Enjeksiyonlarını tespit eder."
                  },
                  {
                      q: "Serileştirmeden Çıkarma (Deserialization) & RSC Saldırıları",
                      a: "Java, PHP, Python Pickle ve Node.js yüklerini tespit eder. Özellikle RSC Flight Protokolü enjeksiyonu ve Server Action problarını avlar."
                  },
                  {
                      q: "SSRF, XXE & SSTI",
                      a: "Bulut Meta Veri SSRF, İç Port tarama, DNS Rebinding, Harici Varlık XXE ve Jinja2, Twig, Freemarker şablon enjeksiyonlarını belirler."
                  }
              ]
          },
          profiling: {
              title: "Saldırgan Profilleme & İstihbarat",
              desc: "Otomatik tarayıcıları, başsız tarayıcıları ve kararlı saldırganları belirlemek için davranışsal profiller toplar.",
              sections: [
                  {
                      q: "Hangi İstihbarat Toplanıyor?",
                      a: "IP adresleri, User-Agent üzerinden Tarayıcı/İS parmak izleri, İstek zamanlaması (ritim), Saldırı yükleri, Hedeflenen uç noktalar ve Başsız (Headless) tarayıcı tespiti (ör. Puppeteer, Selenium)."
                  },
                  {
                      q: "Risk Nasıl Hesaplanır?",
                      a: "0-100 arası bir Risk Puanı, saldırı kategorilerinin şiddetine (ör. SQLi/RCE = 15 puan), teknik çeşitliliğine, otomasyon bonuslarına ve istek hacmine dayalı olarak hesaplanır."
                  },
                  {
                      q: "Otomasyon Tespiti",
                      a: "Botlar ve scriptler, istekler arası aralıklar, hız eşikleri (<100ms), başsız User-Agent işaretleri ve çeşitli tekniklerin hızlıca uygulanması analiz edilerek tespit edilir."
                  }
              ]
          },
          simulation: {
              title: "Gerçekçi RSC Simülasyonu",
              desc: "Saldırganları şüphelendirmeden meşgul tutmak için zamanlama oynamaları ile makul React Server Components yanıtları oluşturur.",
              sections: [
                  {
                      q: "Sahte Flight Protokolü Yanıtları",
                      a: "Bal küpü, şu gibi gerçekçi RSC iletişim formatı yükleri döndürür:\n`0:[\"$\",\"@2\",null,{\"id\":\"__PAGE__\",\"children\":[...]}]`\n`1:{\"status\":\"resolved\",\"data\":...}`"
                  },
                  {
                      q: "Açıkta Bırakılan Sahte Uç Noktalar",
                      a: "Yüksek değerli hedefleri simüle eder: `/` (text/x-component), `/api/graphql`, `/api/auth/callback`, `/_rsc/__PAGE__`, ve `/dashboard`."
                  },
                  {
                      q: "Zamanlama & Aşamalı Boyutlandırma",
                      a: "Yanıt başına rastgele gecikmeler (20–180ms) ekler ve saldırganın ilgisini canlı tutmak ile gerçek arka uç işlemlerini taklit etmek için yanıt boyutlarını kademeli olarak artırır."
                  }
              ]
          }
      }
  };

  let currentData = $derived(content[lang][section]);
</script>

{#if isOpen}
<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div 
  class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm transition-all duration-300"
  onclick={close}
>
  <div 
      class="bg-background border flex flex-col border-border rounded-xl shadow-2xl w-full max-w-4xl max-h-[85vh] overflow-hidden"
      onclick={(e) => e.stopPropagation()}
  >
      <!-- HEADER -->
      <div class="px-6 py-4 border-b border-border bg-surface/80 backdrop-blur-md flex justify-between items-center sticky top-0 z-10 shrink-0">
          <div class="flex items-center gap-3">
              <div class="p-2 bg-yellow-500/10 rounded-lg">
                  <Shield class="w-5 h-5 text-yellow-500 dark:text-yellow-400" />
              </div>
              <div>
                  <h2 class="text-xl font-black text-primary-text tracking-wide">{currentData.title}</h2>
                  <p class="text-xs font-mono text-yellow-500/70 dark:text-yellow-400/70 tracking-widest uppercase">React Honeypot Documentation</p>
              </div>
          </div>
          
          <div class="flex items-center gap-4">
              <button 
                  onclick={close}
                  class="p-2 text-muted hover:text-primary-text hover:bg-surface rounded-lg transition-colors cursor-pointer"
              >
                  <XIcon class="w-5 h-5" />
              </button>
          </div>
      </div>

      <!-- SCROLLABLE CONTENT AREA -->
      <div class="p-6 overflow-y-auto w-full h-full custom-scrollbar">
          
          <div class="mb-8 p-4 bg-surface/50 border border-border rounded-lg shadow-sm">
              <p class="text-primary-text text-sm leading-relaxed">{currentData.desc}</p>
          </div>

          {#if section !== 'overview'}
              <div class="space-y-6">
                  {#each (currentData as any).sections as sec (sec.q)}
                      <div class="bg-surface border border-border/50 rounded-lg p-5 shadow-sm hover:border-border transition-colors">
                          <h3 class="text-lg font-bold text-yellow-500 dark:text-yellow-400 flex items-center gap-2 mb-3">
                              <BookOpen class="w-5 h-5" /> {sec.q}
                          </h3>
                          <div class="text-sm text-primary-text/90 leading-relaxed space-y-2 whitespace-pre-wrap">
                              {sec.a}
                          </div>
                      </div>
                  {/each}
              </div>
          {:else}
              <div class="space-y-4 relative">
                  <div class="absolute left-[27px] top-[40px] bottom-[40px] w-0.5 bg-border z-0"></div>
                  
                  <h3 class="text-lg font-bold text-yellow-500 dark:text-yellow-400 mb-6">Logical Execution Flow</h3>

                  {#each (currentData as any).flow as step, i}
                      {#if step.note}
                          <div class="flex items-start gap-4 relative z-10">
                              <div class="w-14 shrink-0 flex justify-center">
                                  <div class="w-6 h-6 rounded-full bg-purple-500/10 border border-purple-500/30 flex items-center justify-center">
                                      <Activity class="w-3 h-3 text-purple-400" />
                                  </div>
                              </div>
                              <div class="bg-purple-500/5 border border-purple-500/20 rounded-lg p-3 w-full shadow-sm">
                                  <pre class="text-xs font-mono text-purple-600 dark:text-purple-400/90 whitespace-pre-wrap">{step.note}</pre>
                              </div>
                          </div>
                      {:else}
                          <div class="flex items-start gap-4 relative z-10">
                              <div class="w-14 shrink-0 flex justify-center mt-1">
                                  <div class="p-2 rounded-lg {step.actor === 'Attacker' || step.actor === 'Saldırgan' ? 'bg-red-500/10 text-red-500 dark:text-red-400 border border-red-500/20' : 'bg-blue-500/10 text-blue-500 dark:text-blue-400 border border-blue-500/20'}">
                                      {#if step.actor === 'Attacker' || step.actor === 'Saldırgan'}
                                          <Cpu class="w-4 h-4" />
                                      {:else}
                                          <Server class="w-4 h-4" />
                                      {/if}
                                  </div>
                              </div>
                              <div class="flex-1 bg-surface border border-border rounded-lg p-4 shadow-sm hover:border-border/80 transition-colors flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                                  <span class="font-bold text-sm {step.actor === 'Attacker' || step.actor === 'Saldırgan' ? 'text-red-500 dark:text-red-400' : 'text-blue-500 dark:text-blue-400'}">{step.actor}</span>
                                  
                                  <div class="flex flex-col items-center flex-1">
                                      <span class="text-xs font-mono text-muted mb-1">{step.action}</span>
                                      <div class="w-full h-px bg-border/50 relative flex items-center justify-center">
                                          <ArrowRight class="w-4 h-4 text-border absolute right-0 translate-x-2 bg-surface" />
                                      </div>
                                  </div>

                                  <span class="font-bold text-sm {step.target === 'Attacker' || step.target === 'Saldırgan' ? 'text-red-500 dark:text-red-400' : 'text-blue-500 dark:text-blue-400'}">{step.target}</span>
                              </div>
                          </div>
                      {/if}
                  {/each}
              </div>
          {/if}

      </div>
  </div>
</div>
{/if}

<style>
.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(150, 150, 150, 0.3);
  border-radius: 4px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(150, 150, 150, 0.5);
}
</style>
