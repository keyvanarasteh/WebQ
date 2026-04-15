<script lang="ts">
    import { X, ServerCrash, AlertCircle, Search, ShieldCheck, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. What Gets Leaked", icon: AlertCircle },
        { id: 1, label: "2. Fingerprinting Attack Chain", icon: Search },
        { id: 2, label: "3. Banner Cloaking", icon: ShieldCheck }
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
        class="fixed inset-0 z-60 flex items-center justify-center p-4 sm:p-6 bg-overlay backdrop-blur-md"
        transition:fade={{ duration: 200 }}
        onclick={close}
    >
        <div
            class="bg-[#0A0C10] border border-orange-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(249,115,22,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-linear-to-r from-orange-950/40 via-orange-900/10 to-transparent p-6 border-b border-orange-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-orange-500/10 blur-3xl rounded-full pointer-events-none"></div>
                <button
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-orange-500/20 rounded-xl transition-all border border-subtle hover:border-orange-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-linear-to-br from-orange-500/20 to-orange-600/10 flex items-center justify-center border border-orange-500/30 shadow-inner">
                            <ServerCrash size={28} class="text-orange-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">Server Fingerprinting</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">Information disclosure analysis revealing server technology signatures and version leakage.</p>
                        </div>
                    </div>

                    <!-- Tab Navigation -->
                    <div class="flex bg-glass p-1 rounded-lg border border-subtle mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-orange-500/20 text-orange-300 border border-orange-500/20 shadow-sm' : 'text-muted hover:text-primary-text hover:bg-glass border border-transparent'}"
                            >
                                <tab.icon size={14} />
                                <span class="hidden sm:inline">{tab.label}</span>
                            </button>
                        {/each}
                    </div>
                </div>
            </div>

            <!-- Content Area - Scrollable -->
            <div class="flex-1 overflow-y-auto p-6 relative custom-scrollbar">
                {#if activeTab === 0}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">Information Disclosure via Headers</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            Every HTTP response from your server potentially includes headers that announce exactly what software you are running and which version. This is called <strong class="text-primary-text">information disclosure</strong>. Attackers use this to instantly narrow down which known CVEs (Common Vulnerabilities and Exposures) may apply to your specific version.
                        </p>

                        <!-- Leaky Headers Table -->
                        <div class="bg-[#0d1117] rounded-xl border border-orange-500/20 overflow-hidden font-mono text-xs mb-6">
                            <div class="bg-surface/80 px-4 py-2 flex items-center gap-2 border-b border-subtle">
                                <AlertCircle size={12} class="text-red-400" />
                                <span class="text-muted text-xs">Common Information-Leaking Headers</span>
                            </div>
                            <div class="overflow-x-auto">
                                <table class="w-full">
                                    <thead>
                                        <tr class="border-b border-subtle">
                                            <th class="text-left px-4 py-2 text-muted font-medium text-[10px] uppercase tracking-wider">Header</th>
                                            <th class="text-left px-4 py-2 text-muted font-medium text-[10px] uppercase tracking-wider">Example Value</th>
                                            <th class="text-left px-4 py-2 text-muted font-medium text-[10px] uppercase tracking-wider">What It Reveals</th>
                                        </tr>
                                    </thead>
                                    <tbody class="divide-y divide-subtle">
                                        <tr class="hover:bg-surface/30 transition-colors">
                                            <td class="px-4 py-2.5 text-red-300">Server</td>
                                            <td class="px-4 py-2.5 text-cyan-300">Apache/2.4.41 (Ubuntu)</td>
                                            <td class="px-4 py-2.5 text-muted">Web server vendor, exact version, OS</td>
                                        </tr>
                                        <tr class="hover:bg-surface/30 transition-colors">
                                            <td class="px-4 py-2.5 text-red-300">X-Powered-By</td>
                                            <td class="px-4 py-2.5 text-cyan-300">PHP/7.4.3</td>
                                            <td class="px-4 py-2.5 text-muted">Server-side language and exact version</td>
                                        </tr>
                                        <tr class="hover:bg-surface/30 transition-colors">
                                            <td class="px-4 py-2.5 text-red-300">X-AspNet-Version</td>
                                            <td class="px-4 py-2.5 text-cyan-300">4.0.30319</td>
                                            <td class="px-4 py-2.5 text-muted">ASP.NET framework version</td>
                                        </tr>
                                        <tr class="hover:bg-surface/30 transition-colors">
                                            <td class="px-4 py-2.5 text-red-300">X-Generator</td>
                                            <td class="px-4 py-2.5 text-cyan-300">Drupal 9 (https://www.drupal.org)</td>
                                            <td class="px-4 py-2.5 text-muted">CMS vendor and version</td>
                                        </tr>
                                        <tr class="hover:bg-surface/30 transition-colors">
                                            <td class="px-4 py-2.5 text-yellow-300">Via</td>
                                            <td class="px-4 py-2.5 text-cyan-300">1.1 nginx/1.18.0</td>
                                            <td class="px-4 py-2.5 text-muted">Proxy software and version</td>
                                        </tr>
                                    </tbody>
                                </table>
                            </div>
                        </div>

                        <div class="bg-red-950/10 border border-red-500/20 rounded-xl p-4">
                            <h4 class="text-sm font-semibold text-red-300 mb-2 flex items-center gap-2"><AlertCircle size={14}/> Why Version Strings Are Dangerous</h4>
                            <p class="text-xs text-muted leading-relaxed">
                                The National Vulnerability Database (NVD) catalogues thousands of CVEs. Each CVE lists exactly which software versions are vulnerable. When your response says <code class="text-cyan-300">Server: Apache/2.4.41</code>, an attacker immediately runs <code class="text-cyan-300">searchsploit apache 2.4.41</code> and has a list of known exploits in seconds. Removing version banners does not fix vulnerabilities — but it does eliminate the instant lookup.
                            </p>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">The Fingerprinting Attack Chain</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            Server fingerprinting is typically the reconnaissance phase of a larger attack. Here is the end-to-end flow from discovery to exploitation.
                        </p>

                        <div class="space-y-3 mb-8">
                            {#each [
                                { step: "1", color: "blue", label: "Discover Server Banner", desc: "Send any HTTP request and read the response headers. Tools: curl, Burp Suite, browser DevTools. Takes under 1 second.", code: "curl -I https://target.com\n> Server: nginx/1.18.0\n> X-Powered-By: PHP/7.4.3" },
                                { step: "2", color: "orange", label: "Cross-reference CVE Database", desc: "Query NVD (nvd.nist.gov) or use searchsploit. Instantly finds all known vulnerabilities for the exact version.", code: "searchsploit nginx 1.18\n> nginx 1.18.0 — HTTP Request Smuggling (CVE-2021-23017)" },
                                { step: "3", color: "red", label: "Find Matching Exploit", desc: "Public exploit code is available for most CVEs within days of disclosure on Exploit-DB, GitHub, and Metasploit modules.", code: "msfconsole\n> use exploit/multi/http/nginx_chunked_request\n> set RHOSTS target.com" },
                                { step: "4", color: "red", label: "Test Against Target", desc: "The attacker launches the exploit. Without the version banner, they would have needed to manually probe all possible versions — reducing attack speed dramatically.", code: "> exploit\n[*] Started reverse TCP handler\n[+] Session 1 opened" }
                            ] as item}
                                <div class="flex items-start gap-4 p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl">
                                    <div class="w-8 h-8 rounded-full bg-orange-500/20 border border-orange-500/30 flex items-center justify-center shrink-0 font-bold text-orange-300 text-sm">
                                        {item.step}
                                    </div>
                                    <div class="flex-1">
                                        <h4 class="text-sm font-semibold text-primary-text mb-1">{item.label}</h4>
                                        <p class="text-xs text-muted mb-2 leading-relaxed">{item.desc}</p>
                                        <div class="bg-[#0d1117] rounded-lg p-2.5 font-mono text-xs text-cyan-300 border border-subtle whitespace-pre">{item.code}</div>
                                    </div>
                                </div>
                            {/each}
                        </div>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div class="p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl">
                                <h4 class="text-sm font-semibold text-orange-300 mb-2">Passive Fingerprinting</h4>
                                <p class="text-xs text-muted leading-relaxed">Reading server banners from normal HTTP responses. Zero network noise. This is what our scanner does — no attack payloads sent. Shodan and Censys perform passive scanning at internet scale and index your server headers publicly.</p>
                            </div>
                            <div class="p-4 bg-red-950/10 border border-red-500/20 rounded-xl">
                                <h4 class="text-sm font-semibold text-red-300 mb-2">Active Fingerprinting</h4>
                                <p class="text-xs text-muted leading-relaxed">Sending specific probe requests to elicit behavior differences that reveal server type. Tools like Nmap's <code class="text-cyan-300">-sV</code> flag try multiple techniques to confirm the stack even when banners are hidden. More detectable in logs.</p>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">Banner Cloaking Techniques</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            Hiding server banners is a quick win that costs nothing in functionality but eliminates the instant CVE-lookup capability. It is not a substitute for patching — it is a defense-in-depth measure that slows down reconnaissance.
                        </p>

                        <!-- nginx Config -->
                        <div class="relative bg-[#0d1117] rounded-xl border border-orange-500/20 overflow-hidden font-mono text-xs mb-5">
                            <div class="absolute top-0 right-0 bg-orange-500/10 text-orange-400 px-4 py-1.5 rounded-bl-xl text-xs font-bold border-b border-l border-orange-500/30 tracking-widest uppercase">nginx.conf</div>
                            <div class="p-5 overflow-x-auto pt-10 custom-scrollbar">
                                <pre class="text-primary-text leading-relaxed">
<span class="text-muted"># nginx — hide version from Server header</span>
<span class="text-cyan-300">server_tokens</span> <span class="text-red-400">off</span>;
<span class="text-muted"># Result: Server: nginx  (no version)</span>

<span class="text-muted"># Go further — set a custom server name</span>
<span class="text-cyan-300">more_set_headers</span> <span class="text-emerald-300">'Server: webserver'</span>;
<span class="text-muted"># Requires ngx_headers_more module</span>
<span class="text-muted"># Result: Server: webserver  (reveals nothing)</span>

<span class="text-muted"># Remove X-Powered-By (often PHP or framework)</span>
<span class="text-cyan-300">fastcgi_hide_header</span> X-Powered-By;
<span class="text-cyan-300">proxy_hide_header</span> X-Powered-By;</pre>
                            </div>
                        </div>

                        <!-- Apache Config -->
                        <div class="relative bg-[#0d1117] rounded-xl border border-orange-500/20 overflow-hidden font-mono text-xs mb-5">
                            <div class="absolute top-0 right-0 bg-orange-500/10 text-orange-400 px-4 py-1.5 rounded-bl-xl text-xs font-bold border-b border-l border-orange-500/30 tracking-widest uppercase">httpd.conf / apache2.conf</div>
                            <div class="p-5 overflow-x-auto pt-10 custom-scrollbar">
                                <pre class="text-primary-text leading-relaxed">
<span class="text-muted"># Apache — hide OS and version from Server header</span>
<span class="text-cyan-300">ServerTokens</span> <span class="text-orange-300">Prod</span>
<span class="text-muted"># Result: Server: Apache  (no version/OS)</span>

<span class="text-muted"># Remove Server header from error pages</span>
<span class="text-cyan-300">ServerSignature</span> <span class="text-red-400">Off</span>

<span class="text-muted"># Remove X-Powered-By from PHP</span>
<span class="text-muted"># In php.ini:</span>
<span class="text-cyan-300">expose_php</span> = <span class="text-red-400">Off</span></pre>
                            </div>
                        </div>

                        <!-- Full Security Headers Block -->
                        <div class="relative bg-[#0d1117] rounded-xl border border-emerald-500/20 overflow-hidden font-mono text-xs mb-5">
                            <div class="absolute top-0 right-0 bg-emerald-500/10 text-emerald-400 px-4 py-1.5 rounded-bl-xl text-xs font-bold border-b border-l border-emerald-500/30 tracking-widest uppercase">Complete nginx Security Headers Block</div>
                            <div class="p-5 overflow-x-auto pt-10 custom-scrollbar">
                                <pre class="text-primary-text leading-relaxed">
<span class="text-cyan-300">add_header</span> X-Frame-Options <span class="text-emerald-300">"DENY"</span> always;
<span class="text-cyan-300">add_header</span> X-Content-Type-Options <span class="text-emerald-300">"nosniff"</span> always;
<span class="text-cyan-300">add_header</span> X-XSS-Protection <span class="text-emerald-300">"1; mode=block"</span> always;
<span class="text-cyan-300">add_header</span> Referrer-Policy <span class="text-emerald-300">"strict-origin-when-cross-origin"</span> always;
<span class="text-cyan-300">add_header</span> Permissions-Policy <span class="text-emerald-300">"geolocation=(), microphone=(), camera=()"</span> always;
<span class="text-cyan-300">add_header</span> Strict-Transport-Security <span class="text-emerald-300">"max-age=31536000; includeSubDomains; preload"</span> always;
<span class="text-cyan-300">server_tokens</span> <span class="text-red-400">off</span>;</pre>
                            </div>
                        </div>

                        <div class="bg-orange-950/20 border border-orange-500/20 rounded-xl p-4">
                            <h4 class="text-orange-400 font-medium mb-2 text-sm flex items-center gap-2"><ShieldCheck size={14}/> The "Security by Obscurity" Nuance</h4>
                            <p class="text-xs text-muted leading-relaxed">Hiding banners alone is not a security strategy — a determined attacker can fingerprint software through behavior. However, removing version strings eliminates automated low-effort attacks that rely on banner → CVE lookup pipelines. This reduces your attack surface at essentially zero cost. Combined with patching, it provides genuine defense in depth.</p>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-orange-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
                <div class="text-xs text-muted hidden sm:block">
                    Use <kbd class="px-2 py-1 bg-glass rounded mx-1 text-muted border border-glass font-mono">Esc</kbd> or click outside to dismiss
                </div>
                <div class="flex items-center gap-3 w-full sm:w-auto justify-between sm:justify-end">
                    {#if activeTab > 0}
                        <button
                            onclick={prevTab}
                            class="flex items-center gap-1.5 px-5 py-2.5 bg-glass hover:bg-glass-hover text-primary-text text-sm font-medium rounded-xl border border-subtle hover:border-glass transition-all focus:ring-2 focus:ring-glass outline-none"
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
