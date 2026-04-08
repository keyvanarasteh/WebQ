<script lang="ts">
  import SecOpsGuideModal from '$lib/components/ui/SecOpsGuideModal.svelte';
  import { Shield, Check, X } from 'lucide-svelte';

  let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
</script>

<SecOpsGuideModal bind:isOpen title="HTTP Security Headers" icon={Shield}>
  <div class="space-y-8 text-gray-300">
      
      <!-- Introduction section -->
      <section>
          <p class="text-sm leading-relaxed text-gray-400">
              <strong class="text-white">Security Headers</strong> are HTTP response headers that restrict modern browsers from running into easily preventable vulnerabilities like Cross-Site Scripting (XSS), Clickjacking, or packet sniffing. Finding missing security headers is step one of any fundamental penetration test or bug bounty.
          </p>
      </section>

      <!-- Key Headers Breakdown -->
      <section class="space-y-4">
          <!-- Strict-Transport-Security -->
          <div class="bg-[#121214] border border-[#27272a] p-4 rounded-lg">
              <h4 class="font-bold text-green-400 text-sm mb-1 font-mono">Strict-Transport-Security (HSTS)</h4>
              <p class="text-xs text-gray-400 mb-3">Forces the browser to always connect via HTTPS, preventing man-in-the-middle SSL downgrade attacks.</p>
              <div class="bg-[#09090b] p-2 rounded text-[10px] font-mono text-gray-500 flex justify-between">
                  <span>max-age=31536000; includeSubDomains</span>
                  <strong class="text-green-500">CRITICAL</strong>
              </div>
          </div>

          <!-- Content-Security-Policy -->
          <div class="bg-[#121214] border border-[#27272a] p-4 rounded-lg">
              <h4 class="font-bold text-blue-400 text-sm mb-1 font-mono">Content-Security-Policy (CSP)</h4>
              <p class="text-xs text-gray-400 mb-3">The ultimate defense against Cross-Site Scripting (XSS). It defines a strict whitelist of locations where the browser is allowed to load scripts, images, and styles from.</p>
              <div class="bg-[#09090b] p-2 rounded text-[10px] font-mono text-gray-500 flex justify-between overflow-x-auto custom-scrollbar">
                  <span class="whitespace-nowrap">default-src 'self'; script-src 'self' https://apis.google.com</span>
                  <strong class="text-blue-500 shrink-0 ml-2">ADVANCED</strong>
              </div>
          </div>

          <!-- X-Frame-Options -->
          <div class="bg-[#121214] border border-[#27272a] p-4 rounded-lg">
              <h4 class="font-bold text-orange-400 text-sm mb-1 font-mono">X-Frame-Options</h4>
              <p class="text-xs text-gray-400 mb-3">Prevents Clickjacking attacks by denying browsers from rendering the page inside an invisible `&lt;iframe&gt;` on an attacker's website.</p>
              <div class="bg-[#09090b] p-2 rounded text-[10px] font-mono text-gray-500 flex justify-between">
                  <span>DENY / SAMEORIGIN</span>
                  <strong class="text-orange-500">CRITICAL</strong>
              </div>
          </div>
      </section>

      <!-- The Threat Reality -->
      <section class="flex flex-col md:flex-row gap-4">
          <div class="flex-1 p-4 bg-red-500/5 border border-red-500/20 rounded">
              <h5 class="text-xs font-bold text-red-500 uppercase tracking-widest mb-2 flex items-center gap-1">
                  <X class="size-4" /> SecOps Reality
              </h5>
              <p class="text-xs text-gray-400">Missing headers are usually marked as 'Low' or 'Informational' severity in automated scanners. However, chaining a missing CSP with a minor Stored XSS vulnerability immediately escalates the threat to 'Critical' Account Takeover.</p>
          </div>
          <div class="flex-1 p-4 bg-green-500/5 border border-green-500/20 rounded">
              <h5 class="text-xs font-bold text-green-500 uppercase tracking-widest mb-2 flex items-center gap-1">
                  <Check class="size-4" /> Secure Dev Duty
              </h5>
              <p class="text-xs text-gray-400">Implementation is extremely easy but widely forgotten. In SvelteKit frontend servers or reverse proxies like NGINX, a standard block of hardcoded default headers solves 99% of configuration audits.</p>
          </div>
      </section>

  </div>
</SecOpsGuideModal>
