<script lang="ts">
  import SecOpsGuideModal from '$lib/components/ui/SecOpsGuideModal.svelte';
  import { Search, Info, ShieldAlert, Code2 } from 'lucide-svelte';

  let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
</script>

<SecOpsGuideModal bind:isOpen title="Search Engine & Metadata Recon" icon={Search}>
  <div class="space-y-8 text-gray-300">
      
      <!-- Introduction section -->
      <section>
          <p class="text-sm leading-relaxed text-gray-400">
              While typically the domain of marketing teams, <strong class="text-white">SEO (Search Engine Optimization) and Metadata</strong> attributes hold massive implications for OSINT (Open-Source Intelligence) and footprinting. Attackers extract software versions, authors, indexing directives, and social network integrations from meta tags.
          </p>
      </section>

      <!-- Advanced Technical SEO Data Leakage -->
      <section class="bg-[#121214] border border-[#27272a] rounded-xl overflow-hidden">
          <div class="bg-blue-500/10 border-b border-[#27272a] px-6 py-4 flex items-center gap-3">
              <ShieldAlert class="size-5 text-blue-400" />
              <h3 class="font-bold text-white tracking-widest uppercase">Metadata Threat Vectors</h3>
          </div>
          
          <div class="p-6 space-y-6">
              <div>
                  <h4 class="text-sm font-bold text-cyan-400 mb-1">1. The "Generator" Tag Disclosure</h4>
                  <p class="text-xs text-gray-400 mb-2">CMS platforms (WordPress, Joomla, Jekyll) often inject a generator tag indicating the exact software and version making the site.</p>
                  <code class="block bg-[#09090b] text-blue-300 p-3 rounded text-xs font-mono border border-[#27272a]">
                      &lt;meta name="generator" content="WordPress 5.8.1" /&gt;
                  </code>
                  <div class="mt-2 text-xs border-l-2 border-red-500 pl-3 text-red-300">
                      <strong>SecOps:</strong> Directly map version to known CVE exploits.
                  </div>
              </div>

              <div>
                  <h4 class="text-sm font-bold text-cyan-400 mb-1">2. Robots.txt & Indexing Directives</h4>
                  <p class="text-xs text-gray-400 mb-2"><code>noindex, nofollow</code> directives often exist to hide staging sites, admin portals, or internal API documentation.</p>
                  <code class="block bg-[#09090b] text-blue-300 p-3 rounded text-xs font-mono border border-[#27272a]">
                      &lt;meta name="robots" content="noindex, nofollow" /&gt;
                  </code>
                  <div class="mt-2 text-xs border-l-2 border-green-500 pl-3 text-green-300">
                      <strong>Secure Dev:</strong> Use these tags to ensure internal admin dashboards don't accidentally leak into Google Dorks, but realize they do not prevent direct enumeration. They are not an access control mechanism.
                  </div>
              </div>
          </div>
      </section>

      <!-- Social Media Graph -->
      <section>
          <h3 class="text-lg font-bold text-white mb-4 flex items-center gap-2 border-b border-[#27272a] pb-2">
              <Info class="size-5 text-cyan-400" /> OSINT & Open Graph Metrics
          </h3>
          <p class="text-sm text-gray-400 mb-4">
              Open Graph (og:title, og:image) and Twitter Cards provide structured data for link unfurling.
          </p>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="p-4 bg-gray-50/5 dark:bg-[#09090b] border border-[#27272a] rounded">
                  <h4 class="text-sm font-bold text-gray-200 mb-2">Corporate Identity Mapping</h4>
                  <p class="text-xs text-gray-500">Extracts linked social accounts, publisher identities, and authors. Heavily utilized in spear-phishing campaigns to identify target personnel and corporate branding assets.</p>
              </div>
              <div class="p-4 bg-gray-50/5 dark:bg-[#09090b] border border-[#27272a] rounded flex justify-center items-center">
                  <Code2 class="size-10 text-gray-600" />
              </div>
          </div>
      </section>

  </div>
</SecOpsGuideModal>
