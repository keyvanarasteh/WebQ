<script lang="ts">
    import { X, Shield, Activity, GraduationCap, ShieldCheck, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Scoring Model", icon: Activity },
        { id: 1, label: "2. Grade Tiers", icon: GraduationCap },
        { id: 2, label: "3. Grade Improvement", icon: ShieldCheck }
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
                            <Shield size={28} class="text-orange-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">Security Posture Score</h2>
                            <p class="text-sm text-muted mt-1 max-w-md">Composite security grade computed from headers, SSL, WAF, methods, and vulnerability signals.</p>
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
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">How the Score is Computed</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            The security posture score is a weighted composite of six distinct assessment dimensions. Each dimension tests a different layer of your web application's defenses, and their scores are combined using the weights below to produce a final numeric score from 0 to 100, which is then converted to a letter grade.
                        </p>

                        <div class="space-y-3 mb-8">
                            <div class="flex items-center gap-4 p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl hover:border-orange-500/30 transition-colors">
                                <div class="w-14 text-right shrink-0">
                                    <span class="text-2xl font-bold text-orange-400">30%</span>
                                </div>
                                <div class="flex-1">
                                    <h4 class="text-sm font-semibold text-primary-text mb-0.5">HTTP Security Headers</h4>
                                    <p class="text-xs text-muted leading-relaxed">Presence and correctness of CSP, HSTS, X-Frame-Options, X-Content-Type-Options, Referrer-Policy, and Permissions-Policy. The largest single factor.</p>
                                </div>
                                <div class="w-32 h-2 bg-surface rounded-full overflow-hidden shrink-0">
                                    <div class="h-full bg-orange-500 rounded-full" style="width: 30%"></div>
                                </div>
                            </div>

                            <div class="flex items-center gap-4 p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl hover:border-orange-500/30 transition-colors">
                                <div class="w-14 text-right shrink-0">
                                    <span class="text-2xl font-bold text-orange-400">25%</span>
                                </div>
                                <div class="flex-1">
                                    <h4 class="text-sm font-semibold text-primary-text mb-0.5">SSL/TLS Grade</h4>
                                    <p class="text-xs text-muted leading-relaxed">Protocol version (TLS 1.3 > 1.2 > 1.1 > 1.0), cipher suite strength, certificate validity, and HSTS enforcement.</p>
                                </div>
                                <div class="w-32 h-2 bg-surface rounded-full overflow-hidden shrink-0">
                                    <div class="h-full bg-orange-500 rounded-full" style="width: 25%"></div>
                                </div>
                            </div>

                            <div class="flex items-center gap-4 p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl hover:border-orange-500/30 transition-colors">
                                <div class="w-14 text-right shrink-0">
                                    <span class="text-2xl font-bold text-orange-400">20%</span>
                                </div>
                                <div class="flex-1">
                                    <h4 class="text-sm font-semibold text-primary-text mb-0.5">WAF Presence</h4>
                                    <p class="text-xs text-muted leading-relaxed">Whether a Web Application Firewall was detected. Binary factor — either protected or not. Adds baseline bot/SQLi/XSS filtering.</p>
                                </div>
                                <div class="w-32 h-2 bg-surface rounded-full overflow-hidden shrink-0">
                                    <div class="h-full bg-orange-500 rounded-full" style="width: 20%"></div>
                                </div>
                            </div>

                            <div class="flex items-center gap-4 p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl hover:border-orange-500/30 transition-colors">
                                <div class="w-14 text-right shrink-0">
                                    <span class="text-2xl font-bold text-orange-400">10%</span>
                                </div>
                                <div class="flex-1">
                                    <h4 class="text-sm font-semibold text-primary-text mb-0.5">HTTP Methods</h4>
                                    <p class="text-xs text-muted leading-relaxed">Whether dangerous methods (TRACE, PUT, DELETE without auth) are enabled. Each dangerous method reduces score.</p>
                                </div>
                                <div class="w-32 h-2 bg-surface rounded-full overflow-hidden shrink-0">
                                    <div class="h-full bg-orange-500 rounded-full" style="width: 10%"></div>
                                </div>
                            </div>

                            <div class="flex items-center gap-4 p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl hover:border-orange-500/30 transition-colors">
                                <div class="w-14 text-right shrink-0">
                                    <span class="text-2xl font-bold text-orange-400">10%</span>
                                </div>
                                <div class="flex-1">
                                    <h4 class="text-sm font-semibold text-primary-text mb-0.5">Vulnerability Signals</h4>
                                    <p class="text-xs text-muted leading-relaxed">Results from the static vulnerability scan — information disclosure, misconfigured headers, server banner leakage, and other passive signals.</p>
                                </div>
                                <div class="w-32 h-2 bg-surface rounded-full overflow-hidden shrink-0">
                                    <div class="h-full bg-orange-500 rounded-full" style="width: 10%"></div>
                                </div>
                            </div>

                            <div class="flex items-center gap-4 p-4 bg-orange-950/20 border border-orange-500/10 rounded-xl hover:border-orange-500/30 transition-colors">
                                <div class="w-14 text-right shrink-0">
                                    <span class="text-2xl font-bold text-orange-400">5%</span>
                                </div>
                                <div class="flex-1">
                                    <h4 class="text-sm font-semibold text-primary-text mb-0.5">HTTPS Enforcement</h4>
                                    <p class="text-xs text-muted leading-relaxed">Whether the domain is accessible over HTTPS and whether HTTP automatically redirects to HTTPS (httpsAvailable + httpsRedirect badges).</p>
                                </div>
                                <div class="w-32 h-2 bg-surface rounded-full overflow-hidden shrink-0">
                                    <div class="h-full bg-orange-500 rounded-full" style="width: 5%"></div>
                                </div>
                            </div>
                        </div>

                        <div class="bg-orange-950/20 border border-orange-500/20 rounded-xl p-4">
                            <h4 class="text-orange-400 font-medium mb-2 flex items-center gap-2"><Activity size={16}/> Badge Meanings</h4>
                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-xs text-muted">
                                <div class="flex items-center gap-2"><span class="px-2 py-0.5 rounded text-[10px] font-bold uppercase tracking-widest border border-emerald-500/20 bg-emerald-500/10 text-emerald-400 shrink-0">HTTPS</span> Domain responds on port 443 with a valid certificate.</div>
                                <div class="flex items-center gap-2"><span class="px-2 py-0.5 rounded text-[10px] font-bold uppercase tracking-widest border border-emerald-500/20 bg-emerald-500/10 text-emerald-400 shrink-0">HSTS</span> HTTP traffic auto-redirects to HTTPS (no downgrade attacks).</div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">What Each Grade Means</h3>

                        <div class="space-y-4 mb-8">
                            <div class="p-5 border border-emerald-500/30 bg-emerald-950/10 rounded-xl">
                                <div class="flex items-center justify-between mb-3">
                                    <div class="flex items-center gap-3">
                                        <span class="text-4xl font-bold text-emerald-400">A</span>
                                        <div>
                                            <h4 class="text-sm font-semibold text-emerald-300">Excellent Posture — Score 90–100</h4>
                                            <p class="text-xs text-muted">Production-ready security configuration.</p>
                                        </div>
                                    </div>
                                </div>
                                <ul class="space-y-1.5 text-xs text-muted ml-2">
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-emerald-500 mt-0.5 shrink-0"/><span>All critical headers present: CSP, HSTS (with preload), X-Frame-Options, X-Content-Type-Options</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-emerald-500 mt-0.5 shrink-0"/><span>TLS 1.3 only, strong cipher suite (ECDHE-RSA-AES256-GCM-SHA384 or better)</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-emerald-500 mt-0.5 shrink-0"/><span>WAF active and detecting bot/attack traffic</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-emerald-500 mt-0.5 shrink-0"/><span>No dangerous HTTP methods enabled, no server banner leakage</span></li>
                                </ul>
                            </div>

                            <div class="p-5 border border-blue-500/30 bg-blue-950/10 rounded-xl">
                                <div class="flex items-center gap-3 mb-3">
                                    <span class="text-4xl font-bold text-blue-400">B</span>
                                    <div>
                                        <h4 class="text-sm font-semibold text-blue-300">Good — Score 70–89</h4>
                                        <p class="text-xs text-muted">Solid baseline with some hardening opportunities remaining.</p>
                                    </div>
                                </div>
                                <ul class="space-y-1.5 text-xs text-muted ml-2">
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-blue-500 mt-0.5 shrink-0"/><span>Most headers present but CSP may be missing or set to unsafe-inline</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-blue-500 mt-0.5 shrink-0"/><span>TLS 1.2 with strong ciphers — not yet upgraded to TLS 1.3</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-blue-500 mt-0.5 shrink-0"/><span>WAF present or no dangerous methods found</span></li>
                                </ul>
                            </div>

                            <div class="p-5 border border-yellow-500/30 bg-yellow-950/10 rounded-xl">
                                <div class="flex items-center gap-3 mb-3">
                                    <span class="text-4xl font-bold text-yellow-400">C</span>
                                    <div>
                                        <h4 class="text-sm font-semibold text-yellow-300">Significant Gaps — Score 50–69</h4>
                                        <p class="text-xs text-muted">Improvements required before handling sensitive user data.</p>
                                    </div>
                                </div>
                                <ul class="space-y-1.5 text-xs text-muted ml-2">
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-yellow-500 mt-0.5 shrink-0"/><span>Multiple critical headers absent (CSP and HSTS both missing is common here)</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-yellow-500 mt-0.5 shrink-0"/><span>TLS 1.2 with mixed cipher quality or certificate approaching expiry</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-yellow-500 mt-0.5 shrink-0"/><span>No WAF or one dangerous method enabled (e.g., TRACE)</span></li>
                                </ul>
                            </div>

                            <div class="p-5 border border-red-500/30 bg-red-950/10 rounded-xl">
                                <div class="flex items-center gap-3 mb-3">
                                    <span class="text-4xl font-bold text-red-400">D / F</span>
                                    <div>
                                        <h4 class="text-sm font-semibold text-red-300">Critical Exposure — Score &lt;50</h4>
                                        <p class="text-xs text-muted">Immediate action required. Not suitable for production traffic.</p>
                                    </div>
                                </div>
                                <ul class="space-y-1.5 text-xs text-muted ml-2">
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-red-500 mt-0.5 shrink-0"/><span>Most or all security headers absent, site runs over HTTP only</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-red-500 mt-0.5 shrink-0"/><span>TLS 1.0/1.1 still active, weak ciphers (RC4, 3DES), or no HTTPS at all</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={12} class="text-red-500 mt-0.5 shrink-0"/><span>No WAF, multiple dangerous methods, server banner fully exposed</span></li>
                                </ul>
                            </div>
                        </div>

                        <div class="bg-orange-950/20 border border-orange-500/20 rounded-xl p-4">
                            <h4 class="text-orange-400 font-medium mb-2 text-sm">Understanding the HTTPS Badges</h4>
                            <p class="text-xs text-muted leading-relaxed">The <span class="text-emerald-400 font-semibold">HTTPS</span> badge confirms port 443 is active with a valid cert. The <span class="text-emerald-400 font-semibold">HSTS</span> badge (shown as "REDIRECT") means HTTP requests are automatically upgraded — without it, a network attacker can perform SSL stripping, downgrading your connection to plaintext even when HTTPS is available.</p>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-orange-300 mb-4">Actionable Improvement Path</h3>
                        <p class="text-sm text-muted mb-6 max-w-3xl">
                            The items below are sorted by impact — the points each fix adds to your total score. A site scoring a C (around 55 points) can reach an A by completing the first four steps in this list.
                        </p>

                        <div class="space-y-3 mb-8">
                            <div class="flex items-start gap-4 p-4 bg-orange-950/20 border border-orange-500/20 rounded-xl">
                                <div class="w-12 h-12 rounded-xl bg-orange-500/20 flex items-center justify-center shrink-0">
                                    <span class="text-orange-300 font-bold text-sm">+15</span>
                                </div>
                                <div class="flex-1">
                                    <h4 class="text-sm font-semibold text-primary-text mb-1">Add HSTS Header</h4>
                                    <p class="text-xs text-muted leading-relaxed mb-2">The single highest-impact header. Forces all future requests to use HTTPS. Prevents SSL stripping attacks entirely once the browser sees it once.</p>
                                    <code class="text-xs font-mono bg-surface/80 text-cyan-300 px-3 py-1.5 rounded block border border-subtle">Strict-Transport-Security: max-age=31536000; includeSubDomains; preload</code>
                                </div>
                            </div>

                            <div class="flex items-start gap-4 p-4 bg-orange-950/20 border border-orange-500/20 rounded-xl">
                                <div class="w-12 h-12 rounded-xl bg-orange-500/20 flex items-center justify-center shrink-0">
                                    <span class="text-orange-300 font-bold text-sm">+15</span>
                                </div>
                                <div class="flex-1">
                                    <h4 class="text-sm font-semibold text-primary-text mb-1">Deploy Content-Security-Policy</h4>
                                    <p class="text-xs text-muted leading-relaxed mb-2">Defines which sources browsers may load scripts, styles, images, and frames from. Eliminates most XSS attack vectors entirely.</p>
                                    <code class="text-xs font-mono bg-surface/80 text-cyan-300 px-3 py-1.5 rounded block border border-subtle">Content-Security-Policy: default-src 'self'; script-src 'self'; object-src 'none'</code>
                                </div>
                            </div>

                            <div class="flex items-start gap-4 p-4 bg-orange-950/20 border border-orange-500/20 rounded-xl">
                                <div class="w-12 h-12 rounded-xl bg-orange-500/20 flex items-center justify-center shrink-0">
                                    <span class="text-orange-300 font-bold text-sm">+10</span>
                                </div>
                                <div class="flex-1">
                                    <h4 class="text-sm font-semibold text-primary-text mb-1">Deploy a WAF</h4>
                                    <p class="text-xs text-muted leading-relaxed">Cloudflare free tier provides instant OWASP Top 10 protection. Point your DNS through Cloudflare and enable the managed ruleset. No code changes required.</p>
                                </div>
                            </div>

                            <div class="flex items-start gap-4 p-4 bg-orange-950/20 border border-orange-500/20 rounded-xl">
                                <div class="w-12 h-12 rounded-xl bg-orange-500/20 flex items-center justify-center shrink-0">
                                    <span class="text-orange-300 font-bold text-sm">+5</span>
                                </div>
                                <div class="flex-1">
                                    <h4 class="text-sm font-semibold text-primary-text mb-1">Upgrade to TLS 1.3 Only</h4>
                                    <p class="text-xs text-muted leading-relaxed mb-2">Disable TLS 1.0 and 1.1. TLS 1.3 drops broken cipher suites and provides perfect forward secrecy on every connection by design.</p>
                                    <code class="text-xs font-mono bg-surface/80 text-cyan-300 px-3 py-1.5 rounded block border border-subtle">ssl_protocols TLSv1.3;  # nginx</code>
                                </div>
                            </div>

                            <div class="flex items-start gap-4 p-4 bg-orange-950/20 border border-orange-500/20 rounded-xl">
                                <div class="w-12 h-12 rounded-xl bg-orange-500/20 flex items-center justify-center shrink-0">
                                    <span class="text-orange-300 font-bold text-sm">+5</span>
                                </div>
                                <div class="flex-1">
                                    <h4 class="text-sm font-semibold text-primary-text mb-1">Disable Dangerous HTTP Methods</h4>
                                    <p class="text-xs text-muted leading-relaxed mb-2">Disable TRACE (enables XST attacks), PUT and DELETE (file write/delete without auth). Only allow GET, POST, HEAD, OPTIONS.</p>
                                    <code class="text-xs font-mono bg-surface/80 text-cyan-300 px-3 py-1.5 rounded block border border-subtle">limit_except GET POST HEAD OPTIONS {"{ deny all; }"}</code>
                                </div>
                            </div>
                        </div>

                        <div class="bg-emerald-950/20 border border-emerald-500/20 rounded-xl p-4">
                            <h4 class="text-emerald-400 font-medium mb-2 text-sm flex items-center gap-2"><ShieldCheck size={14}/> Quickest Path from C to A</h4>
                            <p class="text-xs text-muted leading-relaxed">A C-grade site (score ~55) typically needs: add HSTS (+15) → add CSP (+15) → disable dangerous methods (+5) → upgrade TLS (+5) = +40 points → score ~95 = A. Total effort: 2–4 hours of nginx/Apache config work.</p>
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
