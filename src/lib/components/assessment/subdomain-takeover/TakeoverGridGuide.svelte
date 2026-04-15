<script lang="ts">
    import { X, Network, Search, ShieldCheck, Link, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Takeover Mechanics", icon: Network },
        { id: 1, label: "2. Reading Results", icon: Search },
        { id: 2, label: "3. Remediation", icon: ShieldCheck }
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
            class="bg-[#0A0C10] border border-amber-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(245,158,11,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-linear-to-r from-amber-950/40 via-amber-900/10 to-transparent p-6 border-b border-amber-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-amber-500/10 blur-3xl rounded-full pointer-events-none"></div>
                <button
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-amber-500/20 rounded-xl transition-all border border-subtle hover:border-amber-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-linear-to-br from-amber-500/20 to-amber-600/10 flex items-center justify-center border border-amber-500/30 shadow-inner">
                            <Link size={28} class="text-amber-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">Vulnerable Subdomain Inventory</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">Detected dangling DNS records pointing to unclaimed third-party service endpoints.</p>
                        </div>
                    </div>

                    <!-- Tab Navigation -->
                    <div class="flex bg-glass p-1 rounded-lg border border-subtle mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-amber-500/20 text-amber-300 border border-amber-500/20 shadow-sm' : 'text-muted hover:text-primary-text hover:bg-glass border border-transparent'}"
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
                        <h3 class="text-lg font-semibold text-amber-300 mb-4">How Subdomain Takeover Works</h3>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                            <div class="space-y-4">
                                <p class="text-primary-text text-sm leading-relaxed">
                                    A subdomain takeover occurs when a DNS CNAME record points to a third-party service that has been deprovisioned or deleted, but the DNS record itself was never removed. An attacker can then claim that service endpoint under their own account, and because the CNAME still points there, they effectively control content served from your subdomain.
                                </p>
                                <div class="bg-amber-950/20 border border-amber-500/10 rounded-xl p-4">
                                    <h4 class="text-amber-400 font-medium mb-3 flex items-center gap-2"><Network size={16}/> The Attack Pattern</h4>
                                    <ul class="space-y-3 text-sm text-muted">
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-amber-500 mt-1 shrink-0"/><span><strong class="text-primary-text">Step 1 — Team sets up service:</strong> A developer creates <code class="text-amber-300 bg-amber-500/10 px-1 rounded">blog.company.com CNAME → company.github.io</code> to host a blog on GitHub Pages.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-amber-500 mt-1 shrink-0"/><span><strong class="text-primary-text">Step 2 — Service is removed:</strong> The developer deletes the GitHub Pages repository or the GitHub organization, but <em>forgets to remove the DNS CNAME record</em>.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-amber-500 mt-1 shrink-0"/><span><strong class="text-primary-text">Step 3 — Attacker claims it:</strong> The attacker creates a GitHub account, claims <code class="text-amber-300 bg-amber-500/10 px-1 rounded">company.github.io</code> as their GitHub Pages site, and now controls what <code class="text-amber-300 bg-amber-500/10 px-1 rounded">blog.company.com</code> serves.</span></li>
                                    </ul>
                                </div>
                            </div>

                            <!-- CNAME Chain Diagram -->
                            <div class="bg-glass border border-subtle rounded-xl p-5 flex flex-col gap-4">
                                <h4 class="text-primary-text text-sm font-medium">CNAME Chain Visualization</h4>

                                <!-- Normal flow -->
                                <div>
                                    <p class="text-xs text-muted uppercase tracking-wider mb-2 font-semibold">Normal (Before Service Removed)</p>
                                    <div class="flex flex-col gap-1.5 text-xs font-mono">
                                        <div class="flex items-center gap-2">
                                            <div class="bg-emerald-500/20 border border-emerald-500/30 rounded px-2 py-1 text-emerald-300">blog.target.com</div>
                                            <ArrowRight size={12} class="text-muted shrink-0"/>
                                            <div class="bg-emerald-500/20 border border-emerald-500/30 rounded px-2 py-1 text-emerald-300">target.github.io</div>
                                            <ArrowRight size={12} class="text-muted shrink-0"/>
                                            <div class="bg-emerald-500/20 border border-emerald-500/30 rounded px-2 py-1 text-emerald-300">185.199.x.x</div>
                                        </div>
                                        <p class="text-muted text-[10px]">DNS resolves → GitHub serves your content correctly</p>
                                    </div>
                                </div>

                                <!-- Dangling state -->
                                <div>
                                    <p class="text-xs text-amber-400 uppercase tracking-wider mb-2 font-semibold">Dangling (Service Deleted, CNAME Remains)</p>
                                    <div class="flex flex-col gap-1.5 text-xs font-mono">
                                        <div class="flex items-center gap-2 flex-wrap">
                                            <div class="bg-amber-500/20 border border-amber-500/30 rounded px-2 py-1 text-amber-300">blog.target.com</div>
                                            <ArrowRight size={12} class="text-muted shrink-0"/>
                                            <div class="bg-red-500/20 border border-red-500/30 rounded px-2 py-1 text-red-400 line-through">target.github.io</div>
                                            <span class="text-red-400 text-[10px]">(not claimed)</span>
                                        </div>
                                        <p class="text-muted text-[10px]">DNS resolves but GitHub returns "404 Not Found" / "There isn't a GitHub Pages site here"</p>
                                    </div>
                                </div>

                                <!-- Taken over state -->
                                <div>
                                    <p class="text-xs text-red-400 uppercase tracking-wider mb-2 font-semibold">Taken Over (Attacker Claims Service)</p>
                                    <div class="flex flex-col gap-1.5 text-xs font-mono">
                                        <div class="flex items-center gap-2 flex-wrap">
                                            <div class="bg-amber-500/20 border border-amber-500/30 rounded px-2 py-1 text-amber-300">blog.target.com</div>
                                            <ArrowRight size={12} class="text-muted shrink-0"/>
                                            <div class="bg-red-500/20 border border-red-500/30 rounded px-2 py-1 text-red-400">target.github.io</div>
                                            <ArrowRight size={12} class="text-muted shrink-0"/>
                                            <div class="bg-red-500/20 border border-red-500/30 rounded px-2 py-1 text-red-400">Attacker's Content</div>
                                        </div>
                                        <p class="text-red-400 text-[10px]">Visitors to blog.target.com now see attacker-controlled content</p>
                                    </div>
                                </div>

                                <div class="bg-red-950/20 border border-red-500/20 rounded-lg p-3 mt-auto">
                                    <p class="text-red-400 text-xs font-medium">Common Vulnerable Services</p>
                                    <p class="text-muted text-[10px] mt-1">GitHub Pages, Heroku, AWS S3 buckets, Azure Blob Storage, Fastly, Netlify, Shopify, Tumblr, WP Engine, Zendesk, HubSpot, Surge.sh</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-amber-300 mb-4">Understanding the Results Table</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">Each row in this table represents a subdomain that was identified as potentially vulnerable to takeover. Understanding each column helps prioritize which records to investigate and remediate first.</p>

                        <!-- Column Explanations -->
                        <div class="space-y-3 mb-6">
                            {#each [
                                { col: "Subdomain", color: "amber", desc: "The vulnerable asset — the full DNS hostname (e.g., blog.company.com) that has a dangling DNS record. This is the subdomain an attacker would take over. Click any row to expand and see the full CNAME chain and DNS record details." },
                                { col: "Service", color: "emerald", desc: "The third-party platform the subdomain's CNAME currently points to (GitHub Pages, Heroku, S3, Netlify, etc.). 'Unknown' means the target service could not be fingerprinted from the response. The service type determines how easy the takeover is — some services require a paid account, others are free." },
                                { col: "Vulnerability Type", color: "blue", desc: "How the dangling record was detected: 'Dangling CNAME' means the CNAME target doesn't have an active service; 'NXDomain' means the CNAME resolves to a non-existent domain entirely; 'Unregistered Domain' means the domain the CNAME points to is available to register." },
                                { col: "Confidence", color: "purple", desc: "The scanner's confidence that this is a real takeover vulnerability, not a false positive. High = strong fingerprint match (e.g., known error page text from the service). Medium = partial match. Low = anomaly detected but manual verification required. ALL confidence levels warrant manual review." }
                            ] as item}
                                <div class="flex gap-3 p-4 bg-glass border border-subtle rounded-xl hover:border-amber-500/20 transition-colors">
                                    <div class="shrink-0">
                                        <span class="px-2 py-1 text-xs font-bold bg-amber-500/20 text-amber-300 border border-amber-500/30 rounded">{item.col}</span>
                                    </div>
                                    <p class="text-sm text-muted leading-relaxed">{item.desc}</p>
                                </div>
                            {/each}
                        </div>

                        <!-- Expanded Row Explanation -->
                        <div class="bg-amber-950/10 border border-amber-500/20 rounded-xl p-5">
                            <h4 class="text-amber-400 font-medium mb-3">Expanded Row Details</h4>
                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 text-sm text-muted">
                                <div>
                                    <p class="text-primary-text text-xs font-semibold mb-1.5">Left Panel</p>
                                    <ul class="space-y-1.5">
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-amber-500 mt-1 shrink-0"/><span><strong>Description:</strong> Human-readable explanation of why this record is vulnerable.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-amber-500 mt-1 shrink-0"/><span><strong>Mitigation:</strong> Specific remediation steps for this subdomain and service type.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-amber-500 mt-1 shrink-0"/><span><strong>Exploitation Difficulty:</strong> How hard it is for an attacker to take over this specific subdomain. "Easy" means freely available service, "Hard" means paid account or complex process required.</span></li>
                                    </ul>
                                </div>
                                <div>
                                    <p class="text-primary-text text-xs font-semibold mb-1.5">Right Panel — DNS Records</p>
                                    <ul class="space-y-1.5">
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-amber-500 mt-1 shrink-0"/><span><strong>A Records:</strong> IP addresses the subdomain currently resolves to (may be CDN IPs if partially resolving).</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-amber-500 mt-1 shrink-0"/><span><strong>CNAME Records:</strong> The full chain of CNAME aliases — this shows exactly where the dangling pointer is in the chain.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-amber-500 mt-1 shrink-0"/><span><strong>NXDomain flag:</strong> If shown in red, the DNS record returns NXDOMAIN (non-existent domain) — the most severe type indicating the pointed-to domain doesn't exist at all.</span></li>
                                    </ul>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Remediation by Vulnerability Type</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">The correct remediation depends on the specific vulnerability type and your intent for the subdomain. The safest default is to remove any DNS record pointing to a service you no longer actively use.</p>

                        <!-- Remediation by type -->
                        <div class="space-y-4 mb-6">
                            <div class="bg-red-950/10 border border-red-500/20 rounded-xl p-5">
                                <div class="flex items-center gap-2 mb-3">
                                    <span class="px-2 py-0.5 text-xs font-bold bg-red-500/20 text-red-400 border border-red-500/30 rounded uppercase">Dangling CNAME</span>
                                </div>
                                <p class="text-sm text-muted mb-3 leading-relaxed">The CNAME points to a third-party service endpoint that exists as a hostname but has no active account/project behind it. An attacker can claim the service.</p>
                                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                                    <div class="bg-emerald-950/20 border border-emerald-500/20 rounded-lg p-3">
                                        <p class="text-emerald-400 text-xs font-bold mb-2">Option A: Remove the DNS record</p>
                                        <div class="bg-[#0d1117] rounded p-2 font-mono text-[10px] space-y-1">
                                            <p class="text-muted"># In your DNS provider dashboard:</p>
                                            <p class="text-red-400">DELETE blog.target.com CNAME target.github.io</p>
                                            <p class="text-muted"># Verify removal:</p>
                                            <p class="text-green-400">dig blog.target.com CNAME +short</p>
                                            <p class="text-muted"># Should return empty</p>
                                        </div>
                                    </div>
                                    <div class="bg-blue-950/20 border border-blue-500/20 rounded-lg p-3">
                                        <p class="text-blue-400 text-xs font-bold mb-2">Option B: Reclaim the service</p>
                                        <p class="text-muted text-xs leading-relaxed">If the subdomain should still be active, re-create the GitHub Pages site / Heroku app / S3 bucket at the original endpoint so it's claimed by your account again. Then update the DNS if needed.</p>
                                    </div>
                                </div>
                            </div>

                            <div class="bg-orange-950/10 border border-orange-500/20 rounded-xl p-5">
                                <div class="flex items-center gap-2 mb-3">
                                    <span class="px-2 py-0.5 text-xs font-bold bg-orange-500/20 text-orange-400 border border-orange-500/30 rounded uppercase">NXDomain</span>
                                </div>
                                <p class="text-sm text-muted mb-3 leading-relaxed">The CNAME points to a domain that does not exist at all (returns NXDOMAIN). This is the most critical type — an attacker can register the pointed-to domain and immediately take over your subdomain.</p>
                                <div class="bg-[#0d1117] rounded-lg p-3 font-mono text-[10px] space-y-1 mb-3">
                                    <p class="text-muted"># blog.target.com CNAME → deletedapp.herokuapp.com</p>
                                    <p class="text-muted"># deletedapp.herokuapp.com → NXDOMAIN</p>
                                    <p class="text-red-400"># IMMEDIATE: Remove the DNS CNAME record</p>
                                    <p class="text-green-400">DELETE blog.target.com CNAME deletedapp.herokuapp.com</p>
                                </div>
                                <p class="text-xs text-orange-400">Do not wait — NXDomain records should be treated as P0 emergencies. The effort to exploit is minimal.</p>
                            </div>

                            <div class="bg-yellow-950/10 border border-yellow-500/20 rounded-xl p-5">
                                <div class="flex items-center gap-2 mb-3">
                                    <span class="px-2 py-0.5 text-xs font-bold bg-yellow-500/20 text-yellow-400 border border-yellow-500/30 rounded uppercase">Expired / Deleted Service</span>
                                </div>
                                <p class="text-sm text-muted mb-3 leading-relaxed">A subscription-based service (Heroku dyno, Netlify site) has expired or been deleted. The provider may have released the namespace, making it available for re-registration.</p>
                                <ul class="space-y-2 text-sm text-muted">
                                    <li class="flex items-start gap-2"><ArrowRight size={14} class="text-yellow-500 mt-1 shrink-0"/><span>Remove the CNAME record immediately if the service is no longer needed.</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={14} class="text-yellow-500 mt-1 shrink-0"/><span>If the subdomain should still exist, update the CNAME to point to the new, active service endpoint.</span></li>
                                </ul>
                            </div>
                        </div>

                        <!-- Low Confidence Warning -->
                        <div class="bg-amber-950/20 border border-amber-500/30 rounded-xl p-4 mb-4">
                            <h4 class="text-amber-400 font-medium mb-2">Even Low Confidence Results Must Be Audited</h4>
                            <p class="text-sm text-muted leading-relaxed">A "Low confidence" result means the scanner detected an anomaly but could not definitively fingerprint a vulnerable service. These can be false positives, but the cost of ignoring a real subdomain takeover far outweighs the cost of a manual 5-minute DNS record audit. Review every result — even Low confidence ones.</p>
                        </div>

                        <!-- Monitoring -->
                        <div class="bg-glass border border-subtle rounded-xl p-4">
                            <h4 class="text-primary-text font-medium mb-3">Ongoing Monitoring Strategy</h4>
                            <p class="text-sm text-muted leading-relaxed mb-3">Subdomain takeover is not a one-time fix — new dangling records appear as infrastructure evolves. Implement continuous monitoring:</p>
                            <ul class="space-y-2 text-sm text-muted">
                                <li class="flex items-start gap-2"><ArrowRight size={14} class="text-emerald-500 mt-1 shrink-0"/><span><strong class="text-primary-text">Run this scan regularly:</strong> Schedule subdomain enumeration + CNAME validation weekly. Catch drift before attackers do.</span></li>
                                <li class="flex items-start gap-2"><ArrowRight size={14} class="text-emerald-500 mt-1 shrink-0"/><span><strong class="text-primary-text">DNS change alerts:</strong> Use your DNS provider's audit logs or tools like Dnstwist to alert on any CNAME additions or modifications.</span></li>
                                <li class="flex items-start gap-2"><ArrowRight size={14} class="text-emerald-500 mt-1 shrink-0"/><span><strong class="text-primary-text">Offboarding process:</strong> Add a DNS record cleanup checklist to your service decommission runbook. Require PR/ticket approval to verify DNS cleanup when deleting a hosted service.</span></li>
                                <li class="flex items-start gap-2"><ArrowRight size={14} class="text-emerald-500 mt-1 shrink-0"/><span><strong class="text-primary-text">Bug bounty scope:</strong> Include subdomain takeover in your bug bounty program scope — security researchers actively look for these and can serve as an additional detection layer.</span></li>
                            </ul>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-amber-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
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
                            class="flex items-center gap-1.5 px-6 py-2.5 bg-amber-600 hover:bg-amber-500 text-primary-text text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(245,158,11,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-amber-500/50 outline-none"
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
