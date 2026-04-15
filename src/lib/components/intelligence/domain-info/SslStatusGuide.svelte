<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Lock, History, ShieldAlert, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
    function close() { isOpen = false; }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: '1. TLS Protocol', icon: Lock },
        { id: 1, label: '2. CT Log Recon', icon: History },
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
                            <Lock size={28} class="text-accent"/>
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.guide_ssl_title()}</h2>
                            <p class="text-sm text-muted mt-1">{m.guide_ssl_subtitle()}</p>
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
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">TLS Protocol & Certificate Health</h3>
                        <p class="text-sm text-muted mb-5 max-w-2xl">The SSL/TLS certificate is the cryptographic backbone of HTTPS. Expired, weak, or misconfigured certificates create exploitable vectors for traffic interception.</p>
                        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-6">
                            <div class="bg-sunken border border-base p-4 rounded-xl border-t-2 border-t-emerald-500">
                                <p class="text-xs font-bold text-emerald-400 mb-1">TLS 1.3</p>
                                <p class="text-xs text-muted">Forward secrecy via ephemeral keys. Only supported cipher suites with AEAD. Gold standard.</p>
                            </div>
                            <div class="bg-sunken border border-base p-4 rounded-xl border-t-2 border-t-yellow-500">
                                <p class="text-xs font-bold text-yellow-400 mb-1">TLS 1.2</p>
                                <p class="text-xs text-muted">Acceptable but cipher suite selection matters. Weak suites like RC4 or 3DES remain negotiable.</p>
                            </div>
                            <div class="bg-sunken border border-base p-4 rounded-xl border-t-2 border-t-red-500">
                                <p class="text-xs font-bold text-red-400 mb-1">TLS 1.0 / 1.1</p>
                                <p class="text-xs text-muted">Deprecated by RFC 8996. Vulnerable to POODLE, BEAST, CRIME. Reject at edge — never support.</p>
                            </div>
                        </div>
                        <div class="bg-cyan-950/20 border border-cyan-500/10 rounded-xl p-4">
                            <h4 class="text-accent font-medium mb-3 flex items-center gap-2"><ShieldAlert size={16}/> Certificate Expiry Risk</h4>
                            <ul class="space-y-2 text-sm text-muted">
                                <li class="flex items-start gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><strong>&gt;30 days:</strong> Healthy — no immediate action required.</span></li>
                                <li class="flex items-start gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><strong>7–30 days:</strong> Renewal window — automate with Let's Encrypt or ACME.</span></li>
                                <li class="flex items-start gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><strong>&lt;7 days:</strong> Critical — browsers will show hard-block security errors to all users.</span></li>
                                <li class="flex items-start gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><strong>Expired:</strong> MitM opportunity — traffic can be intercepted without browser warnings blocking attackers on the same network.</span></li>
                            </ul>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Certificate Transparency Log Reconnaissance</h3>
                        <p class="text-sm text-muted mb-5 max-w-2xl">Every public certificate issued by any CA is permanently logged in public CT logs (crt.sh, Google Argon, etc). This passive recon channel requires zero interaction with the target.</p>
                        <div class="bg-[#0d1117] rounded-xl border border-cyan-500/30 overflow-hidden font-mono text-sm mb-5">
                            <div class="bg-cyan-500/10 text-accent px-4 py-2 text-xs font-bold border-b border-cyan-500/30">crt.sh subdomain enumeration</div>
                            <div class="p-4 text-muted text-xs space-y-1">
                                <p class="text-emerald-400">➜ curl -s "https://crt.sh/?q=%25.target.com&output=json" | jq -r '.[].name_value' | sort -u</p>
                                <div class="mt-2 text-rose-300">
                                    <p>api.target.com</p>
                                    <p>dev-staging.target.com <span class="text-muted italic"> ← hidden env!</span></p>
                                    <p>vpn.target.com</p>
                                    <p>admin-panel.target.com <span class="text-red-400 font-bold"> ← jackpot</span></p>
                                </div>
                            </div>
                        </div>
                        <div class="bg-cyan-950/20 border border-cyan-500/10 rounded-xl p-4">
                            <p class="text-sm text-muted leading-relaxed">WebQ reads the <strong class="text-primary-text">Subject Alternative Names (SANs)</strong> from the live certificate to surface all hostnames the cert covers. Wildcard certs (<code class="text-cyan-300">*.example.com</code>) hint at a broader hidden surface — query CT logs for the specific issuances.</p>
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
