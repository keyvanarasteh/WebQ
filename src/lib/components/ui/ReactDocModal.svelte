<script lang="ts">
  import { XIcon, BookOpen, ArrowRight, ArrowDown, Activity, ShieldAlert, Cpu, Server } from 'lucide-svelte';
  import { languageTag } from '$lib/paraglide/runtime';

  let { 
      isOpen = $bindable(false),
      section = "target"
  } : {
      isOpen: boolean;
      section: "target" | "leak" | "rce_command" | "rce_full" | "comprehensive";
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
          target: {
              title: "Attack Surface & Exploitation Vectors",
              desc: "Deep dive into the React Server Components (RSC) insecure deserialization vulnerabilities (CVE-2025-55182 / CVE-2025-55183).",
              sections: [
                  {
                      q: "Where Do We Attack?",
                      a: "The vulnerability resides deep within the React Server Components (RSC) parsing logic—specifically, how the \"Flight protocol\" insecurely deserializes incoming component payloads. We attack the backend endpoints that process Server Actions or RSC renders. In a typical Next.js App Router application, this means targeting Application routes or API routes that accept and process these Flight payloads."
                  },
                  {
                      q: "Do We Have to Detect ALL POST Requests?",
                      a: "No, it is not strictly necessary to monitor or detect every POST request across the site. We are specifically hunting for endpoints that interact with the RSC architecture. However, because Next.js handles Server Actions on the exact same URLs as the web pages themselves, virtually any valid URL on a Next.js App Router site can act as an attack vector. An attacker does not need to find a specific \"API endpoint\"; they can simply take a valid page URL, append the correct RSC headers, and send a POST request."
                  },
                  {
                      q: "Which POST Requests are Exploitable?",
                      a: "A POST request becomes an exploitable vector if the following conditions are met:\n1. Vulnerable Environment: The target application is running a vulnerable combination of Next.js and React (e.g., Next.js 15.x paired with React 19).\n2. App Router Active: The application utilizes the Next.js App Router architecture.\n3. Flight Protocol Headers: The request must explicitly instruct the server to engage the Flight parser using headers like `Content-Type: text/x-component`, `Next-Action: <action_id>`, and `RSC: 1`."
                  }
              ]
          },
          leak: {
              title: "Source Leak Phase (CVE-2025-55183)",
              desc: "This function simulates the extraction of source code from a vulnerable Next.js/React server. Instead of parsing the actual server response, it returns hardcoded mock data.",
              flow: [
                  { actor: "Attacker", action: "POST /", target: "Target" },
                  { note: "Headers: Content-Type: text/x-component, Next-Action: 1\nPayload: 0:[[\"$\",\"@source\",null,{\"type\":\"module\",\"request\":\"server_function_source\",\"expose\":true}]]" },
                  { actor: "Target", action: "HTTP Response", target: "Attacker", color: "gray" },
                  { note: "Attacker Injects Mock Data (mock_leaked_source)\nExtracts secrets using RegEx" }
              ]
          },
          rce_command: {
              title: "RCE Command Phase (CVE-2025-55182)",
              desc: "This function acts as a demonstration implementation for Remote Code Execution. It operates in a 'Demo mode' for educational simulation.",
              flow: [
                  { actor: "Attacker", action: "POST /", target: "Target" },
                  { note: "Headers: Content-Type: text/x-component, Next-Action: exploit-action\nPayload: Blob Handler Deserialization with child_process.exec" },
                  { actor: "Target", action: "HTTP Response", target: "Attacker", color: "green" },
                  { note: "Simulates Output Parsing: Directly maps raw response body to stdout" }
              ]
          },
          rce_full: {
              title: "Full RCE Chain",
              desc: "This function orchestrates the RCE phase using hardcoded mock commands to demonstrate an attacker's post-exploitation flow.",
              flow: [
                  { note: "Loop Recon Commands: id, whoami, hostname" },
                  { actor: "Attacker", action: "execute_rce_command(cmd)", target: "Target" },
                  { actor: "Target", action: "HTTP Response", target: "Attacker", color: "green" },
                  { note: "Attempt PoC File Creation: echo '...PWNED...' > /tmp/react2shell_pwned.txt" },
                  { actor: "Attacker", action: "execute_rce_command(poc_cmd)", target: "Target" },
                  { actor: "Target", action: "HTTP Response", target: "Attacker", color: "green" },
                  { note: "Validates exit_code == 0 and body contains 'react2shell_pwned.txt'" }
              ]
          },
          comprehensive: {
              title: "Comprehensive React2Shell Scan",
              desc: "Executes a fully-featured automated scan to detect CVE-2025-55182/CVE-2025-55183. Includes header fingerprinting, bundle source analysis, and secret leakage discovery.",
              flow: [
                  { actor: "Scanner", action: "Initial HTTP GET", target: "Target" },
                  { note: "Identify Next.js/React versions from headers and '__NEXT_DATA__' blocks" },
                  { actor: "Target", action: "HTTP Response", target: "Scanner", color: "gray" },
                  { note: "Discover RSC Endpoints by parsing client-side JavaScript bundles" },
                  { actor: "Scanner", action: "RSC Exploit Payload", target: "Target" },
                  { actor: "Target", action: "Source code leak", target: "Scanner", color: "green" },
                  { note: "Analyze leaked source for secrets, credentials, and configuration files" }
              ]
          }
      },
      tr: {
          target: {
              title: "Saldırı Yüzeyi ve Sömürü Vektörleri",
              desc: "React Server Components (RSC) güvenli olmayan serileştirmeden çıkarma (insecure deserialization) zafiyetlerinin (CVE-2025-55182 / CVE-2025-55183) derinlemesine analizi.",
              sections: [
                  {
                      q: "Nereden Saldırıyoruz?",
                      a: "Güvenlik açığı, React Server Components (RSC) ayrıştırma mantığının derinliklerinde yer alır; özellikle \"Flight protokolünün\" gelen bileşen yüklerini güvenli olmayan bir şekilde seri durumdan çıkarmasında. Server Actions veya RSC işlemlerini işleyen arka uç uç noktalarına saldırıyoruz. Tipik bir Next.js App Router uygulamasında bu, Flight yüklerini kabul eden ve işleyen Uygulama rotalarını veya API rotalarını hedeflemek anlamına gelir."
                  },
                  {
                      q: "TÜM POST İsteklerini Tespit Etmeli miyiz?",
                      a: "Hayır, site genelindeki her POST isteğini izlemek kesinlikle gerekli değildir. Özellikle RSC mimarisiyle etkileşime giren uç noktaları arıyoruz. Ancak, Next.js Server Actions işlemlerini web sayfalarının kendileriyle aynı URL'lerde işlediği için, bir Next.js App Router sitesindeki geçerli hemen hemen her URL bir saldırı vektörü olarak kullanılabilir. Yalnızca geçerli bir sayfa URL'sine doğru RSC başlıklarını ekleyerek bir POST isteği göndermek yeterlidir."
                  },
                  {
                      q: "Hangi POST İstekleri Sömürülebilir?",
                      a: "Bir POST isteği aşağıdaki koşulları karşılıyorsa sömürülebilir bir vektör haline gelir:\n1. Savunmasız Ortam: Hedef uygulama, Next.js ve React'in savunmasız bir kombinasyonunu çalıştırıyor (ör. React 19 ile Next.js 15.x).\n2. App Router Aktif: Uygulama Next.js App Router mimarisini kullanıyor.\n3. Flight Protokol Başlıkları: İstek, sunucuya Flight ayrıştırıcısını devreye sokması için `Content-Type: text/x-component`, `Next-Action: <action_id>` ve `RSC: 1` gibi başlıklarla talimat vermelidir."
                  }
              ]
          },
          leak: {
              title: "Kaynak Kodu Sızıntısı Aşaması (CVE-2025-55183)",
              desc: "Bu fonksiyon, savunmasız bir Next.js/React sunucusundan kaynak kodunun çıkarılmasını simüle eder. Gerçek sunucu yanıtını ayrıştırmak yerine, kod içine gömülü sahte veriler döndürür.",
              flow: [
                  { actor: "Saldırgan", action: "POST /", target: "Hedef" },
                  { note: "Başlıklar: Content-Type: text/x-component, Next-Action: 1\nYük: 0:[[\"$\",\"@source\",null,{\"type\":\"module\",\"request\":\"server_function_source\",\"expose\":true}]]" },
                  { actor: "Hedef", action: "HTTP Yanıtı", target: "Saldırgan", color: "gray" },
                  { note: "Saldırgan Sahte Veri Enjekte Eder (mock_leaked_source)\nRegEx kullanılarak sırlar çıkarılır" }
              ]
          },
          rce_command: {
              title: "Uzaktan Kod Çalıştırma (RCE) Aşaması (CVE-2025-55182)",
              desc: "Bu fonksiyon RCE için bir gösterim uygulaması olarak çalışır. Eğitim simülasyonu için 'Demo modu'nda çalışır.",
              flow: [
                  { actor: "Saldırgan", action: "POST /", target: "Hedef" },
                  { note: "Başlıklar: Content-Type: text/x-component, Next-Action: exploit-action\nYük: child_process.exec ile Blob İşleyici Seri Durumdan Çıkarma (Deserialization)" },
                  { actor: "Hedef", action: "HTTP Yanıtı", target: "Saldırgan", color: "green" },
                  { note: "Çıktı Ayrıştırmasını Simüle Eder: Ham yanıt gövdesini doğrudan stdout ile eşler" }
              ]
          },
          rce_full: {
              title: "Tam RCE Zinciri",
              desc: "Bu fonksiyon, bir saldırganın sömürü sonrası akışını göstermek için koda gömülü sahte komutlar kullanarak RCE aşamasını koordine eder.",
              flow: [
                  { note: "Keşif Komutları Döngüsü: id, whoami, hostname" },
                  { actor: "Saldırgan", action: "execute_rce_command(cmd)", target: "Hedef" },
                  { actor: "Hedef", action: "HTTP Yanıtı", target: "Saldırgan", color: "green" },
                  { note: "PoC Dosyası Oluşturma Denemesi: echo '...PWNED...' > /tmp/react2shell_pwned.txt" },
                  { actor: "Saldırgan", action: "execute_rce_command(poc_cmd)", target: "Hedef" },
                  { actor: "Hedef", action: "HTTP Yanıtı", target: "Saldırgan", color: "green" },
                  { note: "exit_code == 0 ve gövdenin 'react2shell_pwned.txt' içerdiği doğrulanır" }
              ]
          },
          comprehensive: {
              title: "Kapsamlı React2Shell Taraması",
              desc: "CVE-2025-55182/CVE-2025-55183 zafiyetlerini tespit etmek için tam donanımlı otomatik bir tarama yürütür. Sürüm parmak izi alma, JS kaynak kodu analizi ve sızdırılmış sır tespiti içerir.",
              flow: [
                  { actor: "Tarayıcı", action: "İlk HTTP GET", target: "Hedef" },
                  { note: "Başlıklardan ve '__NEXT_DATA__' bloklarından Next.js/React sürümlerini belirler" },
                  { actor: "Hedef", action: "HTTP Yanıtı", target: "Tarayıcı", color: "gray" },
                  { note: "İstemci tarafı JS dosyalarını ayrıştırarak RSC Uç Noktalarını keşfeder" },
                  { actor: "Tarayıcı", action: "RSC Sömürü Yükü", target: "Hedef" },
                  { actor: "Hedef", action: "Kaynak Kodu Sızıntısı", target: "Tarayıcı", color: "green" },
                  { note: "Sızdırılan kaynak kodunda sırlar, kimlik bilgileri ve ayar dosyalarını analiz eder" }
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
              <div class="p-2 bg-blue-500/10 rounded-lg">
                  <BookOpen class="w-5 h-5 text-blue-500 dark:text-blue-400" />
              </div>
              <div>
                  <h2 class="text-xl font-black text-primary-text tracking-wide">{currentData.title}</h2>
                  <p class="text-xs font-mono text-blue-500/70 dark:text-blue-400/70 tracking-widest uppercase">React2Shell Documentation</p>
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

          {#if section === 'target'}
              <div class="space-y-6">
                  {#each (currentData as any).sections as sec (sec.q)}
                      <div class="bg-surface border border-border/50 rounded-lg p-5 shadow-sm hover:border-border transition-colors">
                          <h3 class="text-lg font-bold text-red-500 dark:text-red-400 flex items-center gap-2 mb-3">
                              <ShieldAlert class="w-5 h-5" /> {sec.q}
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
                  
                  <h3 class="text-lg font-bold text-blue-500 dark:text-blue-400 mb-6">Logical Execution Flow</h3>

                  {#each (currentData as any).flow as step, i}
                      {#if step.note}
                          <div class="flex items-start gap-4 relative z-10">
                              <div class="w-14 shrink-0 flex justify-center">
                                  <div class="w-6 h-6 rounded-full bg-yellow-500/10 border border-yellow-500/30 flex items-center justify-center">
                                      <Activity class="w-3 h-3 text-yellow-400" />
                                  </div>
                              </div>
                              <div class="bg-yellow-500/5 border border-yellow-500/20 rounded-lg p-3 w-full shadow-sm">
                                  <pre class="text-xs font-mono text-yellow-600 dark:text-yellow-400/90 whitespace-pre-wrap">{step.note}</pre>
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
/* Scoped minimal scrollbar for guides */
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
