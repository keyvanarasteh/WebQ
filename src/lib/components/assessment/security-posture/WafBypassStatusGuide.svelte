<script lang="ts">
    import { X, Server, Shield, Search, ShieldAlert, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. WAF Fundamentals", icon: Shield },
        { id: 1, label: "2. Detection Methods", icon: Search },
        { id: 2, label: "3. Exposed Risks", icon: ShieldAlert }
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
                            <Server size={28} class="text-orange-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">WAF Detection Analysis</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">Web Application Firewall identification via HTTP fingerprinting and response behavioral analysis.</p>
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
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">What a WAF Does</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            A Web Application Firewall operates at <strong class="text-primary-text">Layer 7</strong> (the application layer) of the OSI model. Unlike a network firewall (Layer 3/4) that filters by IP and port, a WAF inspects the actual content of every HTTP request — the URL, headers, POST body, cookies — and blocks requests matching attack patterns.
                        </p>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                            <div class="space-y-4">
                                <div class="bg-orange-950/20 border border-orange-500/10 rounded-xl p-4">
                                    <h4 class="text-orange-400 font-medium mb-3 text-sm">OWASP Top 10 Coverage</h4>
                                    <ul class="space-y-2 text-xs text-muted">
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-orange-500 mt-0.5 shrink-0"/><span><strong class="text-primary-text">SQL Injection:</strong> Detects <code class="text-cyan-300">OR 1=1</code>, <code class="text-cyan-300">UNION SELECT</code>, quote-escaping attempts.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-orange-500 mt-0.5 shrink-0"/><span><strong class="text-primary-text">Cross-Site Scripting:</strong> Blocks <code class="text-cyan-300">&lt;script&gt;</code>, <code class="text-cyan-300">javascript:</code>, and event handler injections.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-orange-500 mt-0.5 shrink-0"/><span><strong class="text-primary-text">File Inclusion:</strong> Catches <code class="text-cyan-300">../../../etc/passwd</code> path traversal patterns.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={12} class="text-orange-500 mt-0.5 shrink-0"/><span><strong class="text-primary-text">Rate Limiting / DDoS:</strong> Identifies and throttles bot floods and credential stuffing campaigns.</span></li>
                                    </ul>
                                </div>
                            </div>

                            <!-- WAF Type Comparison -->
                            <div class="space-y-3">
                                <h4 class="text-sm font-semibold text-primary-text">WAF Deployment Types</h4>
                                <div class="p-3 bg-emerald-950/10 border border-emerald-500/20 rounded-lg">
                                    <div class="text-xs font-semibold text-emerald-300 mb-1">Cloud WAF (Cloudflare, AWS WAF, Akamai)</div>
                                    <p class="text-xs text-muted">DNS is pointed to the WAF edge network. All traffic flows through their PoPs before reaching your origin. Zero infrastructure changes required. Best default choice.</p>
                                </div>
                                <div class="p-3 bg-blue-950/10 border border-blue-500/20 rounded-lg">
                                    <div class="text-xs font-semibold text-blue-300 mb-1">On-Premise WAF (ModSecurity, NAXSI)</div>
                                    <p class="text-xs text-muted">Runs as an nginx/Apache module on your own server. Higher latency overhead but full control over rules. Required in air-gapped environments.</p>
                                </div>
                                <div class="p-3 bg-red-950/10 border border-red-500/20 rounded-lg">
                                    <div class="text-xs font-semibold text-red-300 mb-1">No WAF</div>
                                    <p class="text-xs text-muted">All HTTP traffic reaches your application directly. A single unpatched vulnerability in your framework can be exploited by automated scanners within minutes of disclosure.</p>
                                </div>
                            </div>
                        </div>

                        <!-- Network Diagram -->
                        <div class="bg-glass border border-subtle rounded-xl p-5">
                            <h4 class="text-sm font-medium text-primary-text mb-4">Traffic Flow: WAF-Protected vs Unprotected</h4>
                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-6 font-mono text-xs">
                                <div>
                                    <div class="text-emerald-400 font-semibold mb-2">✓ With WAF</div>
                                    <div class="space-y-1 text-muted">
                                        <div class="flex items-center gap-2"><span class="text-primary-text">Client</span> <span>→ DNS resolves to WAF edge</span></div>
                                        <div class="flex items-center gap-2 ml-4"><span>→ WAF inspects request</span></div>
                                        <div class="flex items-center gap-2 ml-4"><span class="text-red-400">✗ Attack blocked</span> <span>/ </span><span class="text-emerald-400">✓ Clean traffic proxied</span></div>
                                        <div class="flex items-center gap-2 ml-4"><span>→ Origin server receives clean traffic only</span></div>
                                    </div>
                                </div>
                                <div>
                                    <div class="text-red-400 font-semibold mb-2">✗ Without WAF</div>
                                    <div class="space-y-1 text-muted">
                                        <div class="flex items-center gap-2"><span class="text-primary-text">Client</span> <span>→ DNS resolves to origin IP directly</span></div>
                                        <div class="flex items-center gap-2 ml-4"><span class="text-red-400">Attack traffic hits app directly</span></div>
                                        <div class="flex items-center gap-2 ml-4"><span>→ App must validate everything itself</span></div>
                                        <div class="flex items-center gap-2 ml-4"><span class="text-red-400">→ One bug = full compromise</span></div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">How WAF Detection Works</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            WAF detection uses passive HTTP fingerprinting — analyzing the response the server sends without sending any attack payloads. Multiple signal types are combined to build a confidence score for each possible WAF provider.
                        </p>

                        <div class="space-y-4 mb-8">
                            <div class="p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl">
                                <h4 class="text-sm font-semibold text-orange-300 mb-3">Signal 1: Response Header Fingerprinting</h4>
                                <p class="text-xs text-muted mb-3">Many WAFs inject identifying headers or modify existing ones. These are highly reliable signals:</p>
                                <div class="bg-[#0d1117] rounded-lg p-3 font-mono text-xs space-y-1 border border-subtle">
                                    <div><span class="text-muted"># Cloudflare</span></div>
                                    <div><span class="text-cyan-300">Server: cloudflare</span></div>
                                    <div><span class="text-cyan-300">CF-Ray: 8a4b2c3d4e5f6789-LHR</span></div>
                                    <div class="mt-2"><span class="text-muted"># AWS WAF / CloudFront</span></div>
                                    <div><span class="text-cyan-300">X-Cache: Hit from cloudfront</span></div>
                                    <div><span class="text-cyan-300">X-Amz-Cf-Id: abc123...</span></div>
                                    <div class="mt-2"><span class="text-muted"># Akamai</span></div>
                                    <div><span class="text-cyan-300">X-Check-Cacheable: YES</span></div>
                                    <div><span class="text-cyan-300">X-Akamai-Transformed: 9 -</span></div>
                                </div>
                            </div>

                            <div class="p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl">
                                <h4 class="text-sm font-semibold text-orange-300 mb-3">Signal 2: Cookie Injection Fingerprinting</h4>
                                <p class="text-xs text-muted mb-3">WAFs often inject tracking/challenge cookies that carry provider-specific names:</p>
                                <div class="bg-[#0d1117] rounded-lg p-3 font-mono text-xs space-y-1 border border-subtle">
                                    <div><span class="text-muted"># Cloudflare challenge cookie</span></div>
                                    <div><span class="text-cyan-300">Set-Cookie: cf_clearance=abc123; SameSite=None; Secure</span></div>
                                    <div class="mt-2"><span class="text-muted"># Incapsula / Imperva</span></div>
                                    <div><span class="text-cyan-300">Set-Cookie: incap_ses_123_456=...; Path=/; Domain=.target.com</span></div>
                                    <div class="mt-2"><span class="text-muted"># F5 BIG-IP ASM</span></div>
                                    <div><span class="text-cyan-300">Set-Cookie: TS01ab2cd3=...; Path=/</span></div>
                                </div>
                            </div>

                            <div class="p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl">
                                <h4 class="text-sm font-semibold text-orange-300 mb-3">Signal 3: Error Page & Timing Analysis</h4>
                                <p class="text-xs text-muted">When a crafted probe request (e.g., URL with <code class="text-cyan-300">?q=&lt;script&gt;</code>) triggers a block, the resulting 403/406 error page often contains WAF branding. Timing differences between blocked and unblocked requests also indicate active inspection middleware.</p>
                            </div>
                        </div>

                        <div class="bg-orange-950/20 border border-orange-500/20 rounded-xl p-4">
                            <h4 class="text-orange-400 font-medium mb-2 text-sm">What "detection_methods" in Results Means</h4>
                            <p class="text-xs text-muted leading-relaxed">The <code class="text-cyan-300">detection_methods</code> array in WAF results lists which specific signals matched — e.g., <code class="text-cyan-300">["response_headers", "cookie_name"]</code>. Multiple matching methods increase confidence. A single match produces "Low" confidence; three or more produces "High".</p>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">Risks Without WAF Protection</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            An unprotected origin server is exposed to the full volume of internet attack traffic. Automated exploit scanners continuously probe every publicly routable IP address and will find vulnerabilities within hours of deployment.
                        </p>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-8">
                            <div class="p-4 bg-red-950/10 border border-red-500/20 rounded-xl">
                                <h4 class="text-sm font-semibold text-red-300 mb-3">Unprotected Threat Surface</h4>
                                <ul class="space-y-2 text-xs text-muted">
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-red-500 mt-0.5 shrink-0"/><span><strong>Automated SQLi scanners</strong> — tools like SQLMap run continuously against all public IPs, testing thousands of injection vectors per minute.</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-red-500 mt-0.5 shrink-0"/><span><strong>Credential stuffing bots</strong> — leaked username/password combos from data breaches tested against your login endpoint at machine speed.</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-red-500 mt-0.5 shrink-0"/><span><strong>Malicious bot traffic</strong> — content scrapers, vulnerability scanners, DDoS botnets all hit unprotected origins directly.</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-red-500 mt-0.5 shrink-0"/><span><strong>Zero-day exposure window</strong> — new CVEs in popular frameworks (WordPress, Laravel) are exploited within hours of public disclosure.</span></li>
                                </ul>
                            </div>

                            <div class="p-4 bg-emerald-950/10 border border-emerald-500/20 rounded-xl">
                                <h4 class="text-sm font-semibold text-emerald-300 mb-3">WAF-Protected Traffic Analysis</h4>
                                <ul class="space-y-2 text-xs text-muted">
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-emerald-500 mt-0.5 shrink-0"/><span>Cloudflare blocks ~140 billion cyber threats daily across its network — these never reach customer origins.</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-emerald-500 mt-0.5 shrink-0"/><span>Bot management distinguishes verified bots (Googlebot) from malicious ones using JS challenges and fingerprinting.</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-emerald-500 mt-0.5 shrink-0"/><span>Virtual patching: WAF rules can block exploitation of a known CVE in your framework before you have time to patch.</span></li>
                                </ul>
                            </div>
                        </div>

                        <div class="space-y-3">
                            <h4 class="text-sm font-semibold text-primary-text mb-2">Choosing the Right WAF Tier</h4>
                            <div class="p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl">
                                <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 text-xs">
                                    <div>
                                        <div class="font-semibold text-emerald-300 mb-1">Free Tier (Cloudflare Free)</div>
                                        <p class="text-muted">Sufficient for most public websites. Covers OWASP Top 10, DDoS mitigation, and bot filtering. Set up in under 10 minutes via DNS change.</p>
                                    </div>
                                    <div>
                                        <div class="font-semibold text-orange-300 mb-1">Paid WAF ($20–200/mo)</div>
                                        <p class="text-muted">Required for e-commerce, healthcare, or SaaS. Adds advanced bot management, custom ruleset tuning, API gateway protection, and compliance reporting.</p>
                                    </div>
                                    <div>
                                        <div class="font-semibold text-red-300 mb-1">Enterprise WAF (PCI DSS, HIPAA)</div>
                                        <p class="text-muted">Mandatory if handling card data or PHI. Requires WAF with logging, incident response, and audit trail features. Cloudflare Enterprise or AWS WAF + Shield Advanced.</p>
                                    </div>
                                </div>
                            </div>
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
