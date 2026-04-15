<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Globe, Search, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
    function close() { isOpen = false; }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: '1. Record Types', icon: Globe },
        { id: 1, label: '2. Recon Value', icon: Search },
    ];
    function nextTab() { if (activeTab < tabs.length - 1) activeTab++; }
    function prevTab() { if (activeTab > 0) activeTab--; }

    const records = [
        { type: 'A', color: 'text-cyan-400', desc: 'Maps domain → IPv4. Multiple A records = load balancer or CDN. Single A = direct server exposure.' },
        { type: 'AAAA', color: 'text-blue-400', desc: 'Maps domain → IPv6. Reveals IPv6 infrastructure — often less protected than IPv4 equivalents.' },
        { type: 'MX', color: 'text-purple-400', desc: 'Mail exchange servers. Identifies email provider (Google, Microsoft, self-hosted). Target for phishing infrastructure analysis.' },
        { type: 'NS', color: 'text-orange-400', desc: 'Authoritative name servers. Reveals DNS provider (Cloudflare, Route53, etc) and potential zone transfer targets.' },
        { type: 'TXT', color: 'text-yellow-400', desc: 'Stores SPF, DMARC, DKIM, domain verification tokens. Often contains cloud provider ownership proof strings.' },
        { type: 'CNAME', color: 'text-emerald-400', desc: 'Canonical alias chains. Dangling CNAMEs pointing to deprovisioned services are the root cause of subdomain takeover vulnerabilities.' },
        { type: 'SOA', color: 'text-rose-400', desc: 'Start of Authority. Contains primary NS, admin contact email (encoded), and serial number — useful for zone enumeration timing.' },
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
                            <Globe size={28} class="text-accent"/>
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.guide_dns_records_title()}</h2>
                            <p class="text-sm text-muted mt-1">{m.guide_dns_records_subtitle()}</p>
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
                        <h3 class="text-lg font-semibold text-cyan-300 mb-2">DNS Record Type Reference</h3>
                        <p class="text-sm text-muted mb-5">WebQ uses <code class="text-cyan-300">dig</code> to run parallel resolution of all 7 record types simultaneously. Each record type exposes a different layer of infrastructure.</p>
                        <div class="space-y-2">
                            {#each records as r}
                                <div class="flex items-start gap-3 p-3 bg-sunken rounded-lg border border-base">
                                    <span class="font-mono text-sm font-bold {r.color} bg-glass border border-subtle px-2 py-0.5 rounded shrink-0 min-w-[52px] text-center">{r.type}</span>
                                    <p class="text-xs text-muted leading-relaxed">{r.desc}</p>
                                </div>
                            {/each}
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">DNS as a Recon Goldmine</h3>
                        <p class="text-sm text-muted mb-5">DNS data is fully public and requires no interaction with the target's web server. A complete DNS read is typically the first passive step in any infrastructure assessment.</p>
                        <div class="space-y-4">
                            <div class="bg-cyan-950/20 border border-cyan-500/10 rounded-xl p-4">
                                <h4 class="text-accent font-medium mb-3">What Attackers Extract</h4>
                                <ul class="space-y-3 text-sm text-muted">
                                    <li class="flex items-start gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><strong>MX → Email Provider:</strong> If MX points to Google Workspace, attackers know to pivot to credential stuffing via OAuth flows rather than SMTP brute-force.</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><strong>NS → DNS Provider:</strong> Route53 or Cloudflare DNS? Different API key attack surfaces. Zone transfer attempts are made against discovered NS servers.</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><strong>TXT → Cloud Ownership:</strong> TXT records like <code class="text-cyan-300">google-site-verification=…</code> or <code class="text-cyan-300">MS=ms…</code> prove which cloud providers hold accounts.</span></li>
                                    <li class="flex items-start gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><strong>CNAME → Takeover Vectors:</strong> CNAMEs pointing to deprovisioned Heroku, S3, or GitHub Pages resources can be claimed by attackers to serve malicious content under the trusted domain.</span></li>
                                </ul>
                            </div>
                            <div class="bg-[#0d1117] rounded-xl border border-subtle p-4 font-mono text-xs">
                                <p class="text-emerald-400 mb-2">➜ dig +short MX target.com</p>
                                <p class="text-primary-text">10 aspmx.l.google.com.  <span class="text-orange-400 ml-2">← Google Workspace confirmed</span></p>
                                <p class="text-emerald-400 mt-3 mb-2">➜ dig +short TXT target.com</p>
                                <p class="text-primary-text break-all">v=spf1 include:_spf.google.com ~all</p>
                                <p class="text-primary-text break-all">google-site-verification=abc123  <span class="text-yellow-400 ml-2">← GSC ownership</span></p>
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
