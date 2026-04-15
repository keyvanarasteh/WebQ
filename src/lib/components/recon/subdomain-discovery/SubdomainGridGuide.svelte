<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Network, Globe, Target, Search, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() { isOpen = false; }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: '1. Asset Mapping', icon: Target },
        { id: 1, label: '2. Wildcard Risks', icon: Search },
    ];
    function nextTab() { if (activeTab < tabs.length - 1) activeTab++; }
    function prevTab() { if (activeTab > 0) activeTab--; }
</script>

{#if isOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="fixed inset-0 z-[60] flex items-center justify-center p-4 sm:p-6 bg-overlay backdrop-blur-md"
        transition:fade={{ duration: 200 }} onclick={close}>
        <div class="bg-[#0A0C10] border border-indigo-500/20 rounded-2xl w-full max-w-3xl overflow-hidden shadow-[0_0_50px_-12px_rgba(99,102,241,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }} onclick={(e) => e.stopPropagation()}>

            <!-- Header -->
            <div class="relative bg-gradient-to-r from-indigo-950/40 via-purple-900/10 to-transparent p-6 border-b border-indigo-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-indigo-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button onclick={close} class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-indigo-500/20 rounded-xl transition-all border border-subtle hover:border-indigo-500/30">
                    <X size={18} />
                </button>
                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-indigo-500/20 to-purple-600/10 flex items-center justify-center border border-indigo-500/30">
                            <Network size={28} class="text-indigo-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">Subdomain Asset Datagrid</h2>
                            <p class="text-sm text-muted mt-1">Flat-list exploration of external attack surface footprints.</p>
                        </div>
                    </div>
                </div>
            </div>

            <!-- Content -->
            <div class="flex-1 overflow-y-auto p-6">
                {#if activeTab === 0}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-indigo-300 mb-4">Flat-List Discoveries</h3>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                            <div class="space-y-4">
                                <p class="text-sm text-primary-text leading-relaxed">The OSINT Subdomain scanner aggregates data from public DNS registries, VirusTotal, crt.sh (Certificate Transparency Logs), and Shodan to find forgotten development servers.</p>
                                <div class="bg-indigo-950/20 border border-indigo-500/10 rounded-xl p-4">
                                    <h4 class="text-indigo-400 font-medium mb-3 flex items-center gap-2"><Target size={16}/> Target Profiling</h4>
                                    <ul class="space-y-3 text-sm text-muted">
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-indigo-500 mt-1 shrink-0"/><span><strong>Hidden Staging:</strong> Addresses like <code>staging.api.target.com</code> often lack WAF or rate limiting.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-indigo-500 mt-1 shrink-0"/><span><strong>Legacy Infrastructure:</strong> <code>v1-api...</code> URLs might run vulnerable older frameworks.</span></li>
                                    </ul>
                                </div>
                            </div>
                            <div class="bg-[#0d1117] border border-subtle rounded-xl p-4 font-mono text-xs flex flex-col justify-center">
                                <div class="text-muted space-y-2 bg-glass p-3 rounded-lg border border-subtle overflow-x-hidden">
                                    <p class="text-indigo-300">➜ Found 4,000+ endpoints</p>
                                    <p class="text-emerald-400">admin-dev.example.com</p>
                                    <p class="text-orange-400 hover:ext-white">vpn.corporate.example.com</p>
                                    <p class="text-emerald-400">test-db.example.com</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-indigo-300 mb-4">Wildcard Subdomains & Noise</h3>
                        <p class="text-sm text-muted mb-6">When organizations map <code>*.target.com</code> to a specific application, OSINT tools will incorrectly resolve thousands of fake DNS addresses.</p>
                        <div class="bg-glass border border-subtle rounded-xl p-4 font-mono text-xs text-primary-text">
                            <p class="text-rose-400 mb-2">Wildcard Detection:</p>
                            <p>Web-Analyzer automatically queries randomized non-existent subdomains (e.g. <code>jd92js9x.example.com</code>). If it resolves, the engine flags the domain as containing wildcards and actively filters the noise from your datagrid.</p>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer -->
            <div class="bg-[#0A0C10] p-4 border-t border-indigo-500/10 flex items-center justify-between shrink-0">
                <div class="text-xs text-muted hidden sm:block">Press <kbd class="px-2 py-1 bg-glass rounded mx-1 font-mono">Esc</kbd> to dismiss</div>
                <div class="flex items-center gap-3 w-full sm:w-auto justify-end">
                    {#if activeTab > 0}
                        <button onclick={prevTab} class="flex items-center gap-1.5 px-5 py-2.5 bg-glass hover:bg-glass-hover text-primary-text text-sm font-medium rounded-xl border border-subtle transition-all"><ChevronLeft size={16}/> Previous</button>
                    {/if}
                    {#if activeTab < tabs.length - 1}
                        <button onclick={nextTab} class="flex items-center gap-1.5 px-6 py-2.5 bg-indigo-600 hover:bg-indigo-500 text-primary-text text-sm font-semibold rounded-xl transition-all">Continue <ChevronRight size={16}/></button>
                    {:else}
                        <button onclick={close} class="flex items-center gap-1.5 px-8 py-2.5 bg-emerald-500 hover:bg-emerald-400 text-emerald-950 text-sm font-bold rounded-xl transition-all">Got it</button>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/if}
