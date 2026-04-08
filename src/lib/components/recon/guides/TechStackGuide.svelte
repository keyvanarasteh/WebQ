<script lang="ts">
  import SecOpsGuideModal from '$lib/components/ui/SecOpsGuideModal.svelte';
  import { Layers, CheckCircle2, Shield, Fingerprint } from 'lucide-svelte';

  let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
</script>

<SecOpsGuideModal bind:isOpen title="Web Technologies & Fingerprinting" icon={Layers}>
  <div class="space-y-8 text-gray-300">
      
      <!-- Introduction section -->
      <section>
          <p class="text-sm leading-relaxed text-gray-400">
              Identifying the exact <strong class="text-white">technology stack</strong> (frameworks, servers, CMS, analytics) of a target is Step 1 in performing a vulnerability assessment. If you know the version, you know the exploits.
          </p>
      </section>

      <!-- Active vs Passive Fingerprinting -->
      <section class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div class="bg-[#121214] border border-[#27272a] p-5 rounded-xl border-t-2 border-t-cyan-500">
              <div class="flex items-center gap-2 mb-3">
                  <Fingerprint class="size-5 text-cyan-400" />
                  <h3 class="font-bold text-gray-200">Passive Fingerprinting</h3>
              </div>
              <p class="text-xs text-gray-400">Analyzing the raw DOM, standard headers, and JavaScript global objects (e.g. <code>window.React</code> or Wappalyzer signatures) to detect tech without ringing alarms.</p>
          </div>
          
          <div class="bg-[#121214] border border-[#27272a] p-5 rounded-xl border-t-2 border-t-red-500">
              <div class="flex items-center gap-2 mb-3">
                  <Shield class="size-5 text-red-400" />
                  <h3 class="font-bold text-gray-200">Active Probing (e.g., WordPress)</h3>
              </div>
              <p class="text-xs text-gray-400">Sending deliberate requests to known API endpoints (e.g., <code>/wp-json/wp/v2/users</code>) to forcibly extract sensitive user lists, exact version numbers, and plugin hashes.</p>
          </div>
      </section>

      <!-- The "Big Target": WordPress -->
      <section class="bg-blue-500/5 border border-blue-500/20 rounded-xl p-6">
          <h3 class="text-lg font-bold text-blue-400 mb-4 tracking-widest uppercase">Why WordPress is highly targeted</h3>
          <ul class="space-y-4">
              <li class="flex items-start gap-3">
                  <CheckCircle2 class="size-4 text-blue-500 mt-0.5 shrink-0" />
                  <div>
                      <p class="text-sm font-bold text-gray-200">User Enumeration via REST API</p>
                      <p class="text-xs text-gray-500 mt-1">By default, WordPress allows unauthenticated access to the user endpoint. Attackers scrape usernames and perform targeted brute-force attacks on the <code>/wp-login.php</code> portal.</p>
                  </div>
              </li>
              <li class="flex items-start gap-3">
                  <CheckCircle2 class="size-4 text-blue-500 mt-0.5 shrink-0" />
                  <div>
                      <p class="text-sm font-bold text-gray-200">Vulnerable Plugin Sprawl</p>
                      <p class="text-xs text-gray-500 mt-1">90% of WP hacks occur via outdated 3rd-party plugins. Fingerprinting which plugins exist is a gold mine for Remote Code Execution (RCE) via CVE databases.</p>
                  </div>
              </li>
          </ul>
      </section>

      <!-- Defense Section -->
      <section class="p-4 border-l-4 border-green-500 bg-green-500/5">
          <h4 class="font-bold text-green-400 mb-2">🔒 Secure Developer Protocols</h4>
          <p class="text-xs text-gray-400">
              Always disable `X-Powered-By` headers in Node/PHP. In WordPress, use security plugins to block generic REST API access, randomize login paths, and disable user enumeration queries (e.g. `?author=1`).
          </p>
      </section>

  </div>
</SecOpsGuideModal>
