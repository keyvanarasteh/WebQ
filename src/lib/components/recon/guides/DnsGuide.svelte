<script lang="ts">
  import SecOpsGuideModal from '$lib/components/ui/SecOpsGuideModal.svelte';
  import { Network, ShieldAlert, ShieldCheck, Mail, Globe, Server } from 'lucide-svelte';

  let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
</script>

<SecOpsGuideModal bind:isOpen title="DNS Intelligence & Security Records" icon={Network}>
  <div class="space-y-8 text-gray-300">
      
      <!-- Introduction section -->
      <section>
          <p class="text-sm leading-relaxed text-gray-400">
              The <strong class="text-white">Domain Name System (DNS)</strong> is the foundational phonebook of the internet. Analyzing a target's DNS footprint reveals their hosting infrastructure, internal network topologies, and email security posture.
          </p>
      </section>

      <!-- Record Types visual grid -->
      <section>
          <h3 class="text-lg font-bold text-cyan-400 mb-4 flex items-center gap-2">
              <Globe class="size-5" /> Standard Resolution Records
          </h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="bg-[#121214] border border-[#27272a] p-4 rounded-lg">
                  <div class="flex items-center gap-3 mb-2 font-mono text-xs">
                      <span class="px-2 py-1 bg-cyan-500/20 text-cyan-400 font-bold rounded">A / AAAA</span>
                      <span class="text-gray-500">Address Mapping</span>
                  </div>
                  <p class="text-xs text-gray-400">Maps domains to IPv4 (A) and IPv6 (AAAA) addresses. Directly exposes the underlying proxy/WAF or origin hosting servers. <strong class="text-cyan-400">SecOps Target:</strong> Locating origin IP addresses hidden behind Cloudflare.</p>
              </div>
              <div class="bg-[#121214] border border-[#27272a] p-4 rounded-lg">
                  <div class="flex items-center gap-3 mb-2 font-mono text-xs">
                      <span class="px-2 py-1 bg-purple-500/20 text-purple-400 font-bold rounded">CNAME</span>
                      <span class="text-gray-500">Canonical Name</span>
                  </div>
                  <p class="text-xs text-gray-400">Aliases one name to another. Very critical for attack vectors. <strong class="text-purple-400">SecOps Target:</strong> Subdomain Takeovers. If a CNAME points to an unregistered cloud bucket (e.g. AWS/Azure), attackers can register it and hijack the subdomain.</p>
              </div>
          </div>
      </section>

      <!-- Email Security Visual Board -->
      <section class="bg-gray-50/5 dark:bg-[#121214] border border-[#27272a] rounded-xl p-6">
          <h3 class="text-lg font-bold tracking-widest uppercase text-white mb-6 flex items-center gap-2">
              <Mail class="size-5 text-gray-400" /> Organizational Email Security
          </h3>
          
          <div class="space-y-6">
              <!-- SPF -->
              <div class="flex gap-4">
                  <div class="shrink-0 pt-1">
                      <ShieldCheck class="size-6 text-green-500" />
                  </div>
                  <div>
                      <h4 class="font-bold text-gray-200">Sender Policy Framework (SPF)</h4>
                      <p class="text-xs text-gray-400 mt-1 mb-2">Defined in a <span class="font-mono text-yellow-500">TXT</span> record. It explicitly lists which IP addresses are authorized to send email on behalf of the domain.</p>
                      <div class="bg-[#09090b] border border-[#27272a] p-3 rounded font-mono text-xs text-gray-300">
                          <span class="text-cyan-500">v=spf1</span> include:_spf.google.com <span class="text-red-400">~all</span>
                      </div>
                      <p class="text-xs mt-2 text-red-400"><strong class="font-bold text-white">SecOps Threat:</strong> If SPF is missing or ends with <code class="bg-[#27272a] px-1 rounded">?all</code> / <code class="bg-[#27272a] px-1 rounded">+all</code>, attackers can send malicious phishing emails perfectly spoofing the target domain.</p>
                  </div>
              </div>
              
              <div class="h-px bg-[#27272a] w-full"></div>

              <!-- DMARC -->
              <div class="flex gap-4">
                  <div class="shrink-0 pt-1">
                      <ShieldAlert class="size-6 text-orange-500" />
                  </div>
                  <div>
                      <h4 class="font-bold text-gray-200">Domain-based Message Authentication (DMARC)</h4>
                      <p class="text-xs text-gray-400 mt-1 mb-2">Tells receiving mail servers what to do if SPF or DKIM fails (none, quarantine, reject).</p>
                      <div class="bg-[#09090b] border border-[#27272a] p-3 rounded font-mono text-xs text-gray-300">
                          <span class="text-cyan-500">v=DMARC1;</span> p=<span class="text-orange-400">quarantine</span>; rua=mailto:admin@domain.com
                      </div>
                      <p class="text-xs mt-2 text-green-400"><strong class="font-bold text-white">Secure Dev Usage:</strong> Always set p=reject in production to drop spoofed emails before they reach your customers' inboxes.</p>
                  </div>
              </div>
          </div>
      </section>

  </div>
</SecOpsGuideModal>
