<script lang="ts">
    import { X, Database, Search, ShieldCheck, ArrowRight, ChevronRight, ChevronLeft, Server } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Data Sources", icon: Database },
        { id: 1, label: "2. Confidence Scoring", icon: Search },
        { id: 2, label: "3. Hardening", icon: ShieldCheck }
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
        class="fixed inset-0 z-[60] flex items-center justify-center p-4 sm:p-6 bg-overlay backdrop-blur-md"
        transition:fade={{ duration: 200 }}
        onclick={close}
    >
        <div
            class="bg-[#0A0C10] border border-violet-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(139,92,246,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-gradient-to-r from-violet-950/40 via-purple-900/10 to-transparent p-6 border-b border-violet-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-violet-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-violet-500/20 rounded-xl transition-all border border-subtle hover:border-violet-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-violet-500/20 to-purple-600/10 flex items-center justify-center border border-violet-500/30 shadow-inner">
                            <Search size={28} class="text-violet-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">Origin IP Leak Candidates</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">Discovered IP addresses that may bypass WAF/CDN protection layers.</p>
                        </div>
                    </div>

                    <!-- Tab Navigation -->
                    <div class="flex bg-glass p-1 rounded-lg border border-subtle mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-violet-500/20 text-violet-300 border border-violet-500/20 shadow-sm' : 'text-muted hover:text-primary-text hover:bg-glass border border-transparent'}"
                            >
                                <tab.icon size={14} />
                                <span class="hidden sm:inline">{tab.label}</span>
                            </button>
                        {/each}
                    </div>
                </div>
            </div>

            <!-- Content Area - Scrollable -->
            <div class="flex-1 overflow-y-auto p-6 custom-scrollbar relative">
                {#if activeTab === 0}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-violet-300 mb-4">Where Each IP Comes From</h3>
                        <p class="text-sm text-muted mb-6 leading-relaxed max-w-3xl">
                            The grid aggregates results from four distinct intelligence sources. Each row's <strong class="text-primary-text">Source</strong> column tells you exactly which channel exposed the IP — this matters for assessing how credible the leak is and how quickly an attacker could find it.
                        </p>

                        <div class="space-y-4 mb-6">
                            <div class="bg-violet-950/20 border border-violet-500/20 rounded-xl p-5 hover:border-violet-500/40 transition-colors">
                                <div class="flex items-start gap-4">
                                    <div class="bg-violet-500/10 p-2.5 rounded-lg shrink-0">
                                        <Database size={18} class="text-violet-400" />
                                    </div>
                                    <div>
                                        <h4 class="text-sm font-semibold text-violet-300 mb-1">Censys — Internet-wide SSL Scan</h4>
                                        <p class="text-xs text-muted leading-relaxed mb-2">Censys continuously scans the entire IPv4 space on common ports (443, 8443, etc.) and indexes TLS certificate metadata. If your origin server serves HTTPS with your domain's certificate while directly exposed, Censys will catalogue it. This is the highest-confidence channel.</p>
                                        <div class="bg-[#0d1117] rounded-lg p-3 font-mono text-xs">
                                            <span class="text-muted">Query: </span><span class="text-violet-300">services.tls.certificates.leaf.names: "target.com"</span><br />
                                            <span class="text-muted mt-1 block">Match: </span><span class="text-emerald-300">IP 198.51.100.42 — CN=target.com (direct exposure!)</span>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <div class="bg-violet-950/20 border border-violet-500/20 rounded-xl p-5 hover:border-violet-500/40 transition-colors">
                                <div class="flex items-start gap-4">
                                    <div class="bg-blue-500/10 p-2.5 rounded-lg shrink-0">
                                        <Search size={18} class="text-blue-400" />
                                    </div>
                                    <div>
                                        <h4 class="text-sm font-semibold text-blue-300 mb-1">Shodan — Service Fingerprinting</h4>
                                        <p class="text-xs text-muted leading-relaxed">Shodan crawls ports and records service banners, HTTP headers, and certificate data. An origin server running Nginx with <code class="text-blue-300 bg-glass px-1 rounded">Server: nginx/1.24</code> and a matching SSL cert will appear in Shodan searches even if behind Cloudflare for normal traffic.</p>
                                    </div>
                                </div>
                            </div>

                            <div class="bg-violet-950/20 border border-violet-500/20 rounded-xl p-5 hover:border-violet-500/40 transition-colors">
                                <div class="flex items-start gap-4">
                                    <div class="bg-orange-500/10 p-2.5 rounded-lg shrink-0">
                                        <Server size={18} class="text-orange-400" />
                                    </div>
                                    <div>
                                        <h4 class="text-sm font-semibold text-orange-300 mb-1">SecurityTrails — Historical DNS</h4>
                                        <p class="text-xs text-muted leading-relaxed">SecurityTrails maintains a history of DNS A records. Before Cloudflare was added, the domain's A record likely pointed directly to the origin IP. Attackers query this history to find the pre-CDN IP and attempt a direct connection to see if the server is still reachable there.</p>
                                    </div>
                                </div>
                            </div>

                            <div class="bg-violet-950/20 border border-violet-500/20 rounded-xl p-5 hover:border-violet-500/40 transition-colors">
                                <div class="flex items-start gap-4">
                                    <div class="bg-red-500/10 p-2.5 rounded-lg shrink-0">
                                        <Database size={18} class="text-red-400" />
                                    </div>
                                    <div>
                                        <h4 class="text-sm font-semibold text-red-300 mb-1">MX / SPF Record Leakage</h4>
                                        <p class="text-xs text-muted leading-relaxed">Mail infrastructure often lives on the same or adjacent IP. An MX record pointing to <code class="text-red-300 bg-glass px-1 rounded">mail.target.com</code> that resolves to a non-proxied IP exposes the hosting network. SPF records listing <code class="text-red-300 bg-glass px-1 rounded">ip4:x.x.x.x</code> ranges directly name outbound mail IPs.</p>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <!-- Table row mockup with source highlighted -->
                        <div class="bg-[#0d1117] rounded-xl border border-violet-500/10 overflow-hidden font-mono text-xs">
                            <div class="bg-surface/80 px-4 py-2.5 border-b border-subtle text-muted text-[10px]">Grid row example — source column highlighted</div>
                            <table class="w-full text-xs">
                                <thead class="text-muted border-b border-subtle">
                                    <tr>
                                        <th class="px-4 py-2 text-left font-medium">IP Address</th>
                                        <th class="px-4 py-2 text-left font-medium bg-violet-500/5 border-x border-violet-500/20 text-violet-300">Source</th>
                                        <th class="px-4 py-2 text-left font-medium">Confidence</th>
                                        <th class="px-4 py-2 text-left font-medium">Status</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr>
                                        <td class="px-4 py-2.5 text-primary-text">198.51.100.42</td>
                                        <td class="px-4 py-2.5 bg-violet-500/5 border-x border-violet-500/20"><span class="text-violet-300 bg-glass px-2 py-0.5 rounded">Censys</span></td>
                                        <td class="px-4 py-2.5 text-red-400">Very High</td>
                                        <td class="px-4 py-2.5 text-emerald-400">active</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>

                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-violet-300 mb-4">Understanding Confidence Scores</h3>
                        <p class="text-sm text-muted mb-6 leading-relaxed max-w-3xl">
                            Confidence reflects how strong the evidence is that this IP is the real origin and can be directly reached to bypass the CDN. Higher confidence = lower false-positive risk = higher exploitation priority.
                        </p>

                        <div class="space-y-4 mb-6">
                            <div class="bg-red-950/20 border border-red-500/30 rounded-xl p-5">
                                <div class="flex items-center gap-3 mb-3">
                                    <span class="px-3 py-1 text-xs font-bold rounded-full border text-red-400 bg-red-400/10 border-red-400/20">Very High</span>
                                    <span class="text-xs text-muted">Certificate match + direct connection confirmed</span>
                                </div>
                                <p class="text-xs text-muted leading-relaxed">The server at this IP serves the target domain's TLS certificate AND a direct HTTP request returns a 200 response matching the real site. The WAF is fully bypassable. This IP should be treated as critically exposed.</p>
                            </div>

                            <div class="bg-orange-950/20 border border-orange-500/30 rounded-xl p-5">
                                <div class="flex items-center gap-3 mb-3">
                                    <span class="px-3 py-1 text-xs font-bold rounded-full border text-orange-400 bg-orange-400/10 border-orange-400/20">High</span>
                                    <span class="text-xs text-muted">Certificate match only — connection not yet verified</span>
                                </div>
                                <p class="text-xs text-muted leading-relaxed">Censys or Shodan found this IP serving the domain's SSL certificate, but a live connection test wasn't performed or didn't complete in the scan window. Very likely a real origin — manual verification will confirm.</p>
                            </div>

                            <div class="bg-yellow-950/20 border border-yellow-500/30 rounded-xl p-5">
                                <div class="flex items-center gap-3 mb-3">
                                    <span class="px-3 py-1 text-xs font-bold rounded-full border text-yellow-400 bg-yellow-400/10 border-yellow-400/20">Medium</span>
                                    <span class="text-xs text-muted">Historical / indirect evidence</span>
                                </div>
                                <p class="text-xs text-muted leading-relaxed">Found via historical DNS (IP was an A record before the CDN was added) or via MX/SPF records suggesting network proximity. The IP may no longer be the active origin but still warrants investigation — many operators forget to rotate old IPs.</p>
                            </div>
                        </div>

                        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                            <div class="bg-glass border border-subtle rounded-xl p-4 text-center">
                                <div class="text-emerald-400 text-lg font-bold mb-1">active</div>
                                <p class="text-xs text-muted">Direct HTTP connection returned site content — origin is reachable without the WAF</p>
                            </div>
                            <div class="bg-glass border border-subtle rounded-xl p-4 text-center">
                                <div class="text-rose-400 text-lg font-bold mb-1">inactive</div>
                                <p class="text-xs text-muted">IP not responding or returning connection refused — may have been patched or rotated</p>
                            </div>
                            <div class="bg-glass border border-subtle rounded-xl p-4 text-center">
                                <div class="text-muted text-lg font-bold mb-1">unverified</div>
                                <p class="text-xs text-muted">Connection not yet attempted — IP sourced from passive OSINT only, needs manual probe</p>
                            </div>
                        </div>
                    </div>

                {:else if activeTab === 2}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Hardening the Origin</h3>
                        <p class="text-sm text-muted mb-6 leading-relaxed max-w-3xl">
                            Hiding the origin IP is only half the battle. Configure your infrastructure so that even if the IP is discovered, direct connections are blocked at the network level.
                        </p>

                        <!-- nginx config block -->
                        <div class="relative bg-[#0d1117] rounded-xl border border-emerald-500/30 shadow-[0_0_30px_rgba(16,185,129,0.1)] overflow-hidden font-mono text-xs mb-6">
                            <div class="absolute top-0 right-0 bg-emerald-500/10 text-emerald-400 px-4 py-1.5 rounded-bl-xl text-[10px] font-bold border-b border-l border-emerald-500/30 tracking-widest uppercase">nginx — block non-Cloudflare traffic</div>
                            <div class="p-5 pt-10 overflow-x-auto custom-scrollbar">
                                <pre class="text-primary-text leading-relaxed">
<span class="text-muted"># /etc/nginx/snippets/cloudflare-only.conf</span>

<span class="text-violet-300">geo</span> <span class="text-orange-300">$cloudflare_ip</span> <span class="text-primary-text">{'{'}
    default         0;</span>
<span class="text-muted">    # Cloudflare IPv4 ranges (update via cron)</span>
<span class="text-emerald-300">    103.21.244.0/22  1;
    103.22.200.0/22  1;
    103.31.4.0/22    1;
    104.16.0.0/13    1;
    104.24.0.0/14    1;
    108.162.192.0/18 1;
    131.0.72.0/22    1;
    141.101.64.0/18  1;
    162.158.0.0/15   1;
    172.64.0.0/13    1;
    173.245.48.0/20  1;
    188.114.96.0/20  1;
    190.93.240.0/20  1;
    197.234.240.0/22 1;
    198.41.128.0/17  1;</span>
<span class="text-primary-text">{'}'}</span>

<span class="text-violet-300">server</span> <span class="text-primary-text">{'{'}
    listen 443 ssl;</span>

<span class="text-muted">    # Block any direct IP access not from Cloudflare</span>
<span class="text-primary-text">    if (<span class="text-orange-300">$cloudflare_ip</span> = 0) {'{'}
        return 444; <span class="text-muted"># Drop with no response</span>
    {'}'}
{'}'}</span></pre>
                            </div>
                        </div>

                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                            <div class="bg-emerald-950/10 border border-emerald-500/20 rounded-xl p-5 flex gap-4 hover:border-emerald-500/40 transition-colors">
                                <div class="bg-emerald-500/10 p-2 rounded-lg h-fit shrink-0">
                                    <ShieldCheck size={20} class="text-emerald-400" />
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-emerald-300 mb-1.5">Authenticated Origin Pulls</h4>
                                    <p class="text-xs text-emerald-200/70 leading-relaxed">Enable mTLS so Cloudflare presents a client certificate on every request. Your origin verifies <code class="text-emerald-300 bg-glass px-1 rounded">ssl_verify_client on</code> — only CF-originated connections carry the cert and are accepted.</p>
                                </div>
                            </div>
                            <div class="bg-violet-950/10 border border-violet-500/20 rounded-xl p-5 flex gap-4 hover:border-violet-500/40 transition-colors">
                                <div class="bg-violet-500/10 p-2 rounded-lg h-fit shrink-0">
                                    <Database size={20} class="text-violet-400" />
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-violet-300 mb-1.5">Separate Mail Infrastructure</h4>
                                    <p class="text-xs text-violet-200/70 leading-relaxed">Never host your mail server on the same physical server or /24 subnet as the origin. Use a dedicated transactional email service (Postmark, SES) — keep SPF <code class="text-violet-300 bg-glass px-1 rounded">ip4:</code> ranges on a completely separate network.</p>
                                </div>
                            </div>
                            <div class="bg-blue-950/10 border border-blue-500/20 rounded-xl p-5 flex gap-4 hover:border-blue-500/40 transition-colors sm:col-span-2">
                                <div class="bg-blue-500/10 p-2 rounded-lg h-fit shrink-0">
                                    <ArrowRight size={20} class="text-blue-400" />
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-blue-300 mb-1.5">Automate Cloudflare IP List Updates</h4>
                                    <p class="text-xs text-blue-200/70 leading-relaxed">Cloudflare publishes their current IP ranges at <code class="text-blue-300 bg-glass px-1 rounded">https://www.cloudflare.com/ips-v4</code>. Set up a cron job or Lambda to fetch these ranges daily and update your firewall rules / security group automatically. Stale allowlists break legitimate traffic.</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-violet-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
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
                            class="flex items-center gap-1.5 px-6 py-2.5 bg-violet-600 hover:bg-violet-500 text-primary-text text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(139,92,246,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-violet-500/50 outline-none"
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
    .custom-scrollbar::-webkit-scrollbar { width: 8px; height: 8px; }
    .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
    .custom-scrollbar::-webkit-scrollbar-thumb { background-color: rgba(255,255,255,0.1); border-radius: 10px; }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover { background-color: rgba(255,255,255,0.2); }
</style>
