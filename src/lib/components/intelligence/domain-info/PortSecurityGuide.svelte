<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Server, Shield, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
    function close() { isOpen = false; }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: '1. Port Risk Map', icon: Server },
        { id: 1, label: '2. Hardening', icon: Shield },
    ];
    function nextTab() { if (activeTab < tabs.length - 1) activeTab++; }
    function prevTab() { if (activeTab > 0) activeTab--; }

    const ports = [
        { port: '21', svc: 'FTP', risk: 'critical', desc: 'Plain-text protocol. Credentials and data exposed in transit. Active brute-force target.' },
        { port: '22', svc: 'SSH', risk: 'high', desc: 'Exposed globally invites 24/7 brute-force from automated botnets. Should be IP-restricted or VPN-gated.' },
        { port: '3389', svc: 'RDP', risk: 'critical', desc: 'Primary ransomware entry vector. Multiple critical CVEs (BlueKeep). Never expose to the internet.' },
        { port: '3306', svc: 'MySQL', risk: 'critical', desc: 'Database directly accessible from the internet = instant data exfiltration risk. Bind to localhost only.' },
        { port: '5432', svc: 'PostgreSQL', risk: 'critical', desc: 'Same as MySQL. Database servers should never be internet-facing — always behind a private network.' },
        { port: '80', svc: 'HTTP', risk: 'low', desc: 'Expected. Should redirect to HTTPS (443). Serving content over plain HTTP is fine for redirect purposes.' },
        { port: '443', svc: 'HTTPS', risk: 'safe', desc: 'Standard encrypted web traffic. Expected and required.' },
        { port: '8080/8443', svc: 'Alt HTTP/S', risk: 'medium', desc: 'Often dev servers or admin panels. Exposing these publicly can leak internal tooling or test configurations.' },
    ];

    const riskClass: Record<string, string> = {
        safe: 'border-emerald-500 text-emerald-400',
        low: 'border-blue-500 text-blue-400',
        medium: 'border-orange-500 text-orange-400',
        high: 'border-red-500 text-red-400',
        critical: 'border-rose-600 text-rose-400',
    };
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
                            <Server size={28} class="text-accent"/>
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.guide_ports_title()}</h2>
                            <p class="text-sm text-muted mt-1">{m.guide_ports_subtitle()}</p>
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
                        <h3 class="text-lg font-semibold text-cyan-300 mb-2">Common Port Risk Reference</h3>
                        <p class="text-sm text-muted mb-5">WebQ scans 9 common TCP ports using async connect attempts. Each open port is assessed against known attack patterns. The security score is penalized for high-risk exposed services.</p>
                        <div class="space-y-2">
                            {#each ports as p}
                                <div class="flex items-start gap-3 p-3 bg-sunken rounded-lg border border-base">
                                    <span class="font-mono text-sm font-bold {riskClass[p.risk]} border rounded px-2 py-0.5 shrink-0 min-w-[56px] text-center">{p.port}</span>
                                    <div>
                                        <p class="text-sm font-bold text-primary-text">{p.svc} <span class="text-xs font-normal text-muted ml-1">({p.risk})</span></p>
                                        <p class="text-xs text-muted mt-0.5">{p.desc}</p>
                                    </div>
                                </div>
                            {/each}
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Port Hardening Strategies</h3>
                        <div class="space-y-4">
                            <div class="bg-emerald-950/10 border border-emerald-500/20 rounded-xl p-5 flex gap-4">
                                <div class="bg-emerald-500/10 p-2 rounded-lg h-fit"><Shield size={20} class="text-emerald-400"/></div>
                                <div>
                                    <h4 class="text-sm font-bold text-emerald-300 mb-2">Zero Trust Port Access</h4>
                                    <ul class="space-y-2 text-sm text-muted">
                                        <li class="flex gap-2"><ArrowRight size={14} class="text-emerald-500 shrink-0 mt-0.5"/><span>Move SSH (22) and RDP (3389) behind VPN (WireGuard) or Cloudflare Access — never expose publicly.</span></li>
                                        <li class="flex gap-2"><ArrowRight size={14} class="text-emerald-500 shrink-0 mt-0.5"/><span>Bind database ports (3306, 5432) to <code class="text-cyan-300">127.0.0.1</code> only — if they need remote access, use an encrypted tunnel.</span></li>
                                        <li class="flex gap-2"><ArrowRight size={14} class="text-emerald-500 shrink-0 mt-0.5"/><span>Use UFW/iptables to whitelist only required source IPs for admin services.</span></li>
                                    </ul>
                                </div>
                            </div>
                            <div class="bg-blue-950/10 border border-blue-500/20 rounded-xl p-5">
                                <h4 class="text-sm font-bold text-blue-300 mb-2">Score Calculation</h4>
                                <p class="text-sm text-muted leading-relaxed">The security score starts at 100. Points are deducted for: missing HTTPS (−30), missing security headers (−5 each), and open high-risk ports (−10 for SSH/FTP, −15 for RDP, −20 for exposed databases). A score of 80+ means a well-hardened surface.</p>
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
