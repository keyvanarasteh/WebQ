<script lang="ts">
  import SecOpsGuideModal from '$lib/components/ui/SecOpsGuideModal.svelte';
  import { ListChecks, Eye, ShieldCheck } from 'lucide-svelte';

  let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
</script>

<SecOpsGuideModal bind:isOpen title="Bulk Validation & Live Recon" icon={ListChecks}>
  <div class="space-y-8 text-gray-300">
      
      <!-- Introduction section -->
      <section>
          <p class="text-sm leading-relaxed text-gray-400">
              Bulk validation acts as the initial broad stroke of <strong class="text-white">Attack Surface Management (ASM)</strong>. When dealing with thousands of potential subdomains, confirming what is alive and actively resolving over HTTP/HTTPS is crucial for scoping engagements and reducing noise.
          </p>
      </section>

      <!-- The Checks Matrix -->
      <section>
          <h3 class="text-lg font-bold text-cyan-400 mb-4 tracking-widest uppercase text-sm border-b border-[#27272a] pb-2">Diagnostic Layers</h3>
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              
              <div class="bg-[#121214] border border-[#27272a] p-4 rounded-lg flex flex-col items-center text-center">
                  <div class="p-3 bg-purple-500/10 text-purple-400 rounded-full mb-3"><Eye class="size-5" /></div>
                  <h4 class="font-bold text-gray-200 text-sm mb-2">DNS Resolution</h4>
                  <p class="text-[11px] text-gray-500">Detects if the domain actually resolves to an IP. A missing DNS record but present subdomain in a bruteforce list might indicate a potential Subdomain Takeover vector.</p>
              </div>

              <div class="bg-[#121214] border border-[#27272a] p-4 rounded-lg flex flex-col items-center text-center">
                  <div class="p-3 bg-blue-500/10 text-blue-400 rounded-full mb-3"><ListChecks class="size-5" /></div>
                  <h4 class="font-bold text-gray-200 text-sm mb-2">Port 80 (HTTP)</h4>
                  <p class="text-[11px] text-gray-500">Unencrypted web traffic. Modern sites should immediately 301 redirect this to HTTPS. A 200 OK on Port 80 without redirection is an instant security audit flag.</p>
              </div>

              <div class="bg-[#121214] border border-[#27272a] p-4 rounded-lg flex flex-col items-center text-center">
                  <div class="p-3 bg-green-500/10 text-green-400 rounded-full mb-3"><ShieldCheck class="size-5" /></div>
                  <h4 class="font-bold text-gray-200 text-sm mb-2">Port 443 (HTTPS)</h4>
                  <p class="text-[11px] text-gray-500">Secure web traffic. Successful connection indicates an active service and SSL certificate deployment, meaning the endpoint is a live target for deep scanning.</p>
              </div>

          </div>
      </section>

      <!-- Threat Intelligence -->
      <section class="bg-gradient-to-r from-cyan-500/10 to-transparent border-l-4 border-cyan-500 p-4 rounded-r-lg">
          <h4 class="font-bold text-cyan-400 mb-2">Automated Discovery Funnel</h4>
          <p class="text-xs text-gray-400">
              In a real-world Red Team scenario, this Bulk Validator acts as the pipeline between Subdomain Scraping (Amass, Subfinder) and Deep Vulnerability Scanning (Nuclei, ZAP). Out of 10,000 subdomains, only 300 might be <strong class="text-white">VALID</strong>. SecOps tooling focuses solely on those 300 live targets to preserve time, bandwidth, and OPSEC.
          </p>
      </section>

  </div>
</SecOpsGuideModal>
