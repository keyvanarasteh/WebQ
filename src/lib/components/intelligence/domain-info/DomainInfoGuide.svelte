<script lang="ts">
  import SecOpsGuideModal from '$lib/components/ui/SecOpsGuideModal.svelte';
  import { Globe, Shield, Lock, Server, Wifi, AlertTriangle } from 'lucide-svelte';

  let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
</script>

<SecOpsGuideModal bind:isOpen title="Domain Intelligence & WHOIS Recon" icon={Globe}>
  <div class="space-y-8 text-gray-300">
      
      <!-- Introduction section -->
      <section>
          <p class="text-sm leading-relaxed text-gray-400">
              <strong class="text-white">Domain Intelligence</strong> is the first step in any penetration test or bug bounty engagement. By querying WHOIS databases, SSL certificates, and port scanners, an analyst builds a complete infrastructure profile of the target before any active exploitation begins.
          </p>
      </section>

      <!-- WHOIS & Registration -->
      <section class="bg-[#121214] border border-[#27272a] rounded-xl overflow-hidden">
          <div class="bg-cyan-500/10 border-b border-[#27272a] px-6 py-4 flex items-center gap-3">
              <Globe class="size-5 text-cyan-400" />
              <h3 class="font-bold text-white tracking-widest uppercase">WHOIS Intelligence</h3>
          </div>
          
          <div class="p-6 space-y-4">
              <div>
                  <h4 class="text-sm font-bold text-cyan-400 mb-1">1. Registrar & Registration Date</h4>
                  <p class="text-xs text-gray-400 mb-2">WHOIS data reveals the registrar (GoDaddy, Namecheap), registration date, and expiry. Recently registered domains are often used for phishing campaigns.</p>
                  <div class="mt-2 text-xs border-l-2 border-red-500 pl-3 text-red-300">
                      <strong>SecOps:</strong> Domains registered less than 30 days ago in an investigation are considered highly suspicious for command-and-control (C2) or phishing infrastructure.
                  </div>
              </div>

              <div>
                  <h4 class="text-sm font-bold text-cyan-400 mb-1">2. IP Geolocation Mapping</h4>
                  <p class="text-xs text-gray-400">Resolving the domain's A record reveals the hosting provider's IP range. This maps the target's geographical jurisdiction and hosting provider (AWS, Google Cloud, Hetzner, etc.).</p>
              </div>
          </div>
      </section>

      <!-- SSL/TLS Security -->
      <section class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div class="bg-[#121214] border border-[#27272a] p-5 rounded-xl border-t-2 border-t-green-500">
              <div class="flex items-center gap-2 mb-3">
                  <Lock class="size-5 text-green-400" />
                  <h3 class="font-bold text-gray-200">SSL Certificate Analysis</h3>
              </div>
              <p class="text-xs text-gray-400">Certificate chain inspection reveals the Certificate Authority (Let's Encrypt, DigiCert), protocol version (TLS 1.2/1.3), and expiry window. Expired certificates break trust and open MITM (Man-in-the-Middle) attack surfaces.</p>
          </div>
          
          <div class="bg-[#121214] border border-[#27272a] p-5 rounded-xl border-t-2 border-t-orange-500">
              <div class="flex items-center gap-2 mb-3">
                  <AlertTriangle class="size-5 text-orange-400" />
                  <h3 class="font-bold text-gray-200">Certificate Transparency Logs</h3>
              </div>
              <p class="text-xs text-gray-400">CT logs passively enumerate ALL subdomains that were ever issued a certificate. Tools like <code>crt.sh</code> query these logs to discover hidden subdomains without active scanning.</p>
          </div>
      </section>

      <!-- Port Scanning -->
      <section class="bg-red-500/5 border border-red-500/20 rounded-xl p-6">
          <div class="flex items-center gap-2 mb-4">
              <Server class="size-5 text-red-400" />
              <h3 class="text-lg font-bold text-red-400 tracking-widest uppercase">Port Security Matrix</h3>
          </div>
          <div class="space-y-3">
              <p class="text-xs text-gray-400">Open ports directly correspond to running services. Each open port is a potential attack entry point:</p>
              <div class="grid grid-cols-2 md:grid-cols-3 gap-3 mt-4">
                  <div class="bg-[#09090b] border border-[#27272a] p-3 rounded text-center">
                      <span class="font-mono text-green-400 text-sm font-bold">80/443</span>
                      <p class="text-[10px] text-gray-500 mt-1">HTTP/HTTPS — Expected</p>
                  </div>
                  <div class="bg-[#09090b] border border-red-500/30 p-3 rounded text-center">
                      <span class="font-mono text-red-400 text-sm font-bold">22</span>
                      <p class="text-[10px] text-gray-500 mt-1">SSH — Brute Force target</p>
                  </div>
                  <div class="bg-[#09090b] border border-red-500/30 p-3 rounded text-center">
                      <span class="font-mono text-red-400 text-sm font-bold">3306</span>
                      <p class="text-[10px] text-gray-500 mt-1">MySQL — Data Exfil risk</p>
                  </div>
              </div>
          </div>
      </section>

      <!-- Defense Section -->
      <section class="p-4 border-l-4 border-green-500 bg-green-500/5">
          <h4 class="font-bold text-green-400 mb-2">🔒 Secure Developer Protocols</h4>
          <p class="text-xs text-gray-400">
              Always enable WHOIS privacy protection to hide personal registrant details. Enforce TLS 1.3 minimum on all endpoints. Use firewall rules to restrict all ports except 80/443 on production servers. Monitor Certificate Transparency logs for unauthorized certificate issuance.
          </p>
      </section>

  </div>
</SecOpsGuideModal>
