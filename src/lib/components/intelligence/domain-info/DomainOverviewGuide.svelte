<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Globe, Server, Fingerprint, ArrowRight, ChevronRight, ChevronLeft, Search } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() { isOpen = false; }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: '1. WHOIS Forensics', icon: Fingerprint },
        { id: 1, label: '2. IP & Reverse DNS', icon: Server },
    ];
    function nextTab() { if (activeTab < tabs.length - 1) activeTab++; }
    function prevTab() { if (activeTab > 0) activeTab--; }
</script>

{#if isOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="fixed inset-0 z-[60] flex items-center justify-center p-4 sm:p-6 bg-overlay backdrop-blur-md"
        transition:fade={{ duration: 200 }} onclick={close}>
        <div class="bg-[#0A0C10] border border-cyan-500/20 rounded-2xl w-full max-w-3xl overflow-hidden shadow-[0_0_50px_-12px_rgba(6,182,212,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }} onclick={(e) => e.stopPropagation()}>

            <!-- Header -->
            <div class="relative bg-gradient-to-r from-cyan-950/40 via-blue-900/10 to-transparent p-6 border-b border-cyan-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-cyan-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button onclick={close} class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-cyan-500/20 rounded-xl transition-all border border-subtle hover:border-cyan-500/30" aria-label="Close">
                    <X size={18} />
                </button>
                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30">
                            <Globe size={28} class="text-accent" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.guide_domain_overview_title()}</h2>
                            <p class="text-sm text-muted mt-1">{m.guide_domain_overview_subtitle()}</p>
                        </div>
                    </div>
                    <div class="flex bg-glass p-1 rounded-lg border border-subtle mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/20' : 'text-muted hover:text-primary-text hover:bg-glass border border-transparent'}">
                                <tab.icon size={14} />
                                <span class="hidden sm:inline">{tab.label}</span>
                            </button>
                        {/each}
                    </div>
                </div>
            </div>

            <!-- Content -->
            <div class="flex-1 overflow-y-auto p-6">
                {#if activeTab === 0}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">WHOIS Database Forensics</h3>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                            <div class="space-y-4">
                                <p class="text-sm text-primary-text leading-relaxed">WHOIS is the public registry of every domain registration. It maps a domain to its owner, registrar, creation date, name servers, and expiry — all publicly queryable without touching the target server.</p>
                                <div class="bg-cyan-950/20 border border-cyan-500/10 rounded-xl p-4">
                                    <h4 class="text-accent font-medium mb-3 flex items-center gap-2"><Fingerprint size={16}/> Key Intelligence Points</h4>
                                    <ul class="space-y-3 text-sm text-muted">
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><strong>Age:</strong> Domains &lt;30 days old are highly suspicious — often phishing or C2 infrastructure.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><strong>Registrar:</strong> Bulletproof registrars that ignore abuse reports are favored by threat actors.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><strong>Name Servers:</strong> Immediately reveals the DNS/CDN provider (Cloudflare, AWS Route53, etc).</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><strong>Privacy:</strong> WHOIS privacy guard hides registrant PII — its absence may expose contacts for spear-phishing.</span></li>
                                    </ul>
                                </div>
                            </div>
                            <div class="bg-[#0d1117] border border-subtle rounded-xl p-4 font-mono text-xs flex flex-col justify-center">
                                <p class="text-emerald-400 mb-3 font-semibold flex items-center gap-2"><Search size={12}/> whois example.com</p>
                                <div class="text-muted space-y-1 bg-glass p-3 rounded-lg border border-subtle">
                                    <p>Registrar: GoDaddy.com, LLC</p>
                                    <p class="text-red-400 font-bold">Creation Date: 2024-11-02  <span class="text-red-500/70 text-[10px]">← 5 days old!</span></p>
                                    <p>Registrant: REDACTED FOR PRIVACY</p>
                                    <p class="text-orange-400">Name Server: NS1.CLOUDFLARE.COM</p>
                                    <p class="text-orange-400">Name Server: NS2.CLOUDFLARE.COM</p>
                                    <p>Status: clientTransferProhibited</p>
                                    <p>Expiry: 2025-11-02</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">IP Resolution & Reverse DNS</h3>
                        <p class="text-sm text-muted mb-6 max-w-2xl">IP addresses reveal hosting providers, geographic footprints, and shared infrastructure. Reverse DNS maps an IP back to a hostname — often exposing internal naming conventions.</p>
                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-6">
                            <div class="bg-sunken border border-base p-5 rounded-xl border-t-2 border-t-cyan-500">
                                <div class="flex items-center gap-2 mb-3"><Globe class="size-5 text-cyan-400"/><h4 class="font-bold text-primary-text">IPv4 / IPv6</h4></div>
                                <p class="text-sm text-muted leading-relaxed">Multiple A records indicate load balancers or CDN edge nodes. A single IP points to a bare metal / VPS server — often easier to fingerprint with Shodan or Censys.</p>
                            </div>
                            <div class="bg-sunken border border-base p-5 rounded-xl border-t-2 border-t-emerald-500">
                                <div class="flex items-center gap-2 mb-3"><Server class="size-5 text-emerald-400"/><h4 class="font-bold text-primary-text">Reverse DNS (PTR)</h4></div>
                                <p class="text-sm text-muted leading-relaxed">PTR records reveal server naming like <code class="text-cyan-300 text-xs">mail.internal.corp.com</code> — disclosing network topology and service roles without active probing.</p>
                            </div>
                        </div>
                        <div class="bg-glass border border-subtle rounded-xl p-4 font-mono text-xs">
                            <p class="text-emerald-400 mb-2">➜ dig +short example.com A</p>
                            <p class="text-primary-text">104.21.x.x  <span class="text-cyan-400 text-[10px] ml-2">← Cloudflare edge (proxied)</span></p>
                            <p class="text-emerald-400 mt-3 mb-2">➜ dig -x 104.21.x.x +short</p>
                            <p class="text-primary-text">edge-node-lax-01.cloudflare.com.  <span class="text-orange-400 text-[10px] ml-2">← PTR reveals datacenter region</span></p>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer -->
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
