<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Activity, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
    function close() { isOpen = false; }
    let activeTab = $state(0);
    const tabs = [{ id: 0, label: '1. Tracker Detection', icon: Activity }, { id: 1, label: '2. Privacy & Risk', icon: ArrowRight }];
    function next() { if (activeTab < tabs.length - 1) activeTab++; }
    function prev() { if (activeTab > 0) activeTab--; }
</script>

{#if isOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="fixed inset-0 z-[60] flex items-center justify-center p-4 sm:p-6 bg-overlay backdrop-blur-md" transition:fade={{ duration: 200 }} onclick={close}>
        <div class="bg-[#0A0C10] border border-cyan-500/20 rounded-2xl w-full max-w-3xl overflow-hidden shadow-[0_0_50px_-12px_rgba(6,182,212,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }} onclick={(e) => e.stopPropagation()}>
            <div class="relative bg-gradient-to-r from-cyan-950/40 via-blue-900/10 to-transparent p-6 border-b border-cyan-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-cyan-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button onclick={close} class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-cyan-500/20 rounded-xl transition-all border border-subtle" aria-label="Close"><X size={18}/></button>
                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30"><Activity size={28} class="text-accent"/></div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.guide_analytics_title()}</h2>
                            <p class="text-sm text-muted mt-1">{m.guide_analytics_subtitle()}</p>
                        </div>
                    </div>
                    <div class="flex bg-glass p-1 rounded-lg border border-subtle mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button onclick={() => activeTab = tab.id} class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/20' : 'text-muted hover:text-primary-text hover:bg-glass border border-transparent'}">
                                <tab.icon size={14}/><span class="hidden sm:inline">{tab.label}</span>
                            </button>
                        {/each}
                    </div>
                </div>
            </div>
            <div class="flex-1 overflow-y-auto p-6">
                {#if activeTab === 0}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Marketing Stack Fingerprinting</h3>
                        <p class="text-sm text-muted mb-4">WebQ scans the page HTML and JavaScript for known tracking tool signatures. This reveals the target's marketing stack — competitive intelligence and privacy exposure surface simultaneously.</p>
                        <div class="space-y-2">
                            {#each [
                                { tool: 'Google Tag Manager', sig: 'googletagmanager.com, dataLayer', intel: 'Reveals GTM usage — means tracking is container-managed and may include many sub-trackers loaded dynamically.' },
                                { tool: 'Facebook/Meta Pixel', sig: 'connect.facebook.net, fbq(', intel: 'Facebook retargeting active. Custom audience building from visitor behavior.' },
                                { tool: 'Hotjar', sig: 'static.hotjar.com, hjid', intel: 'Session recording and heatmaps active. User interactions (including form inputs) may be captured.' },
                                { tool: 'Intercom / Zendesk', sig: 'widget.intercom.io, zdassets.com', intel: 'Customer support chat loaded. May reveal CRM vendor and support infrastructure.' },
                                { tool: 'LinkedIn Insight', sig: 'snap.licdn.com, _linkedin_partner_id', intel: 'B2B retargeting enabled. LinkedIn can build audiences from company/job title data of visitors.' },
                            ] as t}
                                <div class="p-3 bg-sunken border border-base rounded-xl">
                                    <div class="flex items-center justify-between mb-1">
                                        <p class="text-sm font-bold text-primary-text">{t.tool}</p>
                                        <code class="text-xs text-muted font-mono">{t.sig.split(',')[0]}</code>
                                    </div>
                                    <p class="text-xs text-muted">{t.intel}</p>
                                </div>
                            {/each}
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-rose-400 mb-4">Privacy Exposure & GDPR Risk</h3>
                        <p class="text-sm text-muted mb-4">Each tracking pixel loaded is a third-party data processor. Under GDPR and CCPA, each one requires explicit consent — failure exposes the site to significant regulatory fines.</p>
                        <div class="space-y-3">
                            <div class="bg-red-950/10 border border-red-500/20 rounded-xl p-4">
                                <h4 class="text-sm font-bold text-red-400 mb-2">Data Minimization Violations</h4>
                                <p class="text-xs text-muted">Session recording tools like Hotjar can capture sensitive form inputs (credit cards, passwords) if not properly configured with masking. Most default configurations record everything.</p>
                            </div>
                            <div class="bg-orange-950/10 border border-orange-500/20 rounded-xl p-4">
                                <h4 class="text-sm font-bold text-orange-400 mb-2">Third-Party Data Sharing</h4>
                                <p class="text-xs text-muted">Each pixel sends visitor data (IP, behavior, demographics) to its respective company (Meta, Google, LinkedIn) — creating data sharing agreements that require disclosure in your privacy policy.</p>
                            </div>
                            <div class="flex gap-2 text-sm text-muted"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span>Use a Consent Management Platform (CMP) like OneTrust or Cookiebot to gate all tracking pixels behind explicit user consent.</span></div>
                        </div>
                    </div>
                {/if}
            </div>
            <div class="bg-[#0A0C10] p-4 border-t border-cyan-500/10 flex items-center justify-between shrink-0">
                <div class="text-xs text-muted hidden sm:block">Press <kbd class="px-2 py-1 bg-glass rounded mx-1 border border-glass font-mono">Esc</kbd> to dismiss</div>
                <div class="flex items-center gap-3 w-full sm:w-auto justify-between sm:justify-end">
                    {#if activeTab > 0}<button onclick={prev} class="flex items-center gap-1.5 px-5 py-2.5 bg-glass hover:bg-glass-hover text-primary-text text-sm font-medium rounded-xl border border-subtle transition-all"><ChevronLeft size={16}/> Previous</button>{:else}<div></div>{/if}
                    {#if activeTab < tabs.length - 1}
                        <button onclick={next} class="flex items-center gap-1.5 px-6 py-2.5 bg-cyan-600 hover:bg-cyan-500 text-primary-text text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(6,182,212,0.4)] transition-all">Continue <ChevronRight size={16}/></button>
                    {:else}
                        <button onclick={close} class="flex items-center gap-1.5 px-8 py-2.5 bg-emerald-500 hover:bg-emerald-400 text-emerald-950 text-sm font-bold rounded-xl transition-all">Got it</button>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/if}
