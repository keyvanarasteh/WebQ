<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Mail, ShieldCheck, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
    function close() { isOpen = false; }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: '1. SPF & DMARC', icon: Mail },
        { id: 1, label: '2. Email Attacks', icon: ShieldCheck },
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

            <div class="relative bg-gradient-to-r from-cyan-950/40 via-blue-900/10 to-transparent p-6 border-b border-cyan-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-cyan-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button onclick={close} class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-cyan-500/20 rounded-xl transition-all border border-subtle hover:border-cyan-500/30" aria-label="Close"><X size={18}/></button>
                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30">
                            <Mail size={28} class="text-accent"/>
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.guide_dns_security_title()}</h2>
                            <p class="text-sm text-muted mt-1">{m.guide_dns_security_subtitle()}</p>
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
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">SPF & DMARC Explained</h3>
                        <div class="space-y-4 mb-6">
                            <div class="bg-sunken border border-base rounded-xl p-5 border-t-2 border-t-cyan-500">
                                <h4 class="font-bold text-primary-text mb-2">SPF — Sender Policy Framework</h4>
                                <p class="text-sm text-muted mb-3 leading-relaxed">A TXT record listing all IP addresses authorized to send email on behalf of the domain. If an email arrives from an IP not in this list, receiving mail servers can reject or flag it.</p>
                                <div class="bg-[#0d1117] rounded-lg p-3 font-mono text-xs text-muted">
                                    <p class="text-emerald-400">v=spf1 include:_spf.google.com ip4:203.0.113.5 ~all</p>
                                    <p class="mt-2 text-xs space-y-1">
                                        <span class="text-cyan-300">~all</span> = softfail (mark as spam) · <span class="text-red-400">-all</span> = hardfail (reject)
                                    </p>
                                </div>
                            </div>
                            <div class="bg-sunken border border-base rounded-xl p-5 border-t-2 border-t-purple-500">
                                <h4 class="font-bold text-primary-text mb-2">DMARC — Domain-based Message Authentication</h4>
                                <p class="text-sm text-muted mb-3 leading-relaxed">Builds on SPF and DKIM. Tells receiving servers what to do with emails that fail SPF/DKIM checks (none / quarantine / reject) and where to send failure reports.</p>
                                <div class="bg-[#0d1117] rounded-lg p-3 font-mono text-xs text-muted">
                                    <p class="text-emerald-400">v=DMARC1; p=reject; rua=mailto:dmarc@example.com; pct=100</p>
                                    <p class="mt-2 text-xs"><span class="text-cyan-300">p=reject</span> = reject failing emails · <span class="text-cyan-300">rua</span> = aggregate report address</p>
                                </div>
                            </div>
                        </div>
                        <div class="bg-cyan-950/20 border border-cyan-500/10 rounded-xl p-4">
                            <p class="text-sm text-muted"><strong class="text-primary-text">DKIM</strong> (DomainKeys Identified Mail) is the third pillar — a cryptographic signature added by the sending server. WebQ checks for SPF and DMARC which are readable from DNS; DKIM keys are in DNS too but vary per mail provider selector.</p>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-rose-400 mb-4">Email Spoofing & BEC Attacks</h3>
                        <p class="text-sm text-muted mb-5">Without SPF and DMARC, anyone on the internet can send emails that appear to come from your domain. This enables highly convincing Business Email Compromise (BEC) attacks.</p>
                        <div class="space-y-4">
                            <div class="bg-red-950/10 border border-red-500/20 rounded-xl p-5">
                                <h4 class="text-sm font-bold text-red-400 mb-2">Missing SPF → Direct Domain Spoofing</h4>
                                <p class="text-sm text-muted leading-relaxed">Any server worldwide can send <code class="text-rose-300">From: ceo@yourcompany.com</code> without any SPF records blocking it. Attackers use this to send wire transfer requests to finance teams or credential-harvesting links to employees.</p>
                            </div>
                            <div class="bg-orange-950/10 border border-orange-500/20 rounded-xl p-5">
                                <h4 class="text-sm font-bold text-orange-400 mb-2">SPF Without DMARC → Incomplete Protection</h4>
                                <p class="text-sm text-muted leading-relaxed">SPF alone isn't enough — it only checks the envelope sender (SMTP MAIL FROM), not the visible From: header. DMARC enforces alignment between them, closing the spoofing gap that SPF alone leaves open.</p>
                            </div>
                            <div class="bg-emerald-950/10 border border-emerald-500/20 rounded-xl p-5 flex gap-3">
                                <ShieldCheck size={20} class="text-emerald-400 shrink-0 mt-0.5"/>
                                <div>
                                    <h4 class="text-sm font-bold text-emerald-300 mb-1">Recommended Configuration</h4>
                                    <ul class="space-y-1 text-sm text-muted">
                                        <li class="flex gap-2"><ArrowRight size={14} class="text-emerald-500 shrink-0 mt-0.5"/> SPF with <code class="text-cyan-300">-all</code> (hard reject)</li>
                                        <li class="flex gap-2"><ArrowRight size={14} class="text-emerald-500 shrink-0 mt-0.5"/> DMARC with <code class="text-cyan-300">p=reject</code> and aggregate reporting</li>
                                        <li class="flex gap-2"><ArrowRight size={14} class="text-emerald-500 shrink-0 mt-0.5"/> DKIM configured for all sending services</li>
                                    </ul>
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
