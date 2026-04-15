<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Search, Terminal, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() { isOpen = false; }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: '1. Theme & Plugins', icon: Search },
        { id: 1, label: '2. User Exposure', icon: Terminal },
    ];
    function nextTab() { if (activeTab < tabs.length - 1) activeTab++; }
    function prevTab() { if (activeTab > 0) activeTab--; }
</script>

{#if isOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="fixed inset-0 z-[60] flex items-center justify-center p-4 sm:p-6 bg-overlay backdrop-blur-md"
        transition:fade={{ duration: 200 }} onclick={close}>
        <div class="bg-[#0A0C10] border border-blue-500/20 rounded-2xl w-full max-w-3xl overflow-hidden shadow-[0_0_50px_-12px_rgba(59,130,246,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }} onclick={(e) => e.stopPropagation()}>

            <div class="relative bg-gradient-to-r from-blue-950/40 via-cyan-900/10 to-transparent p-6 border-b border-blue-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-blue-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button onclick={close} class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-blue-500/20 rounded-xl transition-all border border-subtle hover:border-blue-500/30">
                    <X size={18} />
                </button>
                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-blue-500/20 to-cyan-600/10 flex items-center justify-center border border-blue-500/30">
                            <Search size={28} class="text-blue-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">CMS Forensics</h2>
                            <p class="text-sm text-muted mt-1">Specialized WordPress footprinting and vulnerability detection.</p>
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex-1 overflow-y-auto p-6">
                {#if activeTab === 0}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-blue-300 mb-4">Core Fingerprinting</h3>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                            <div class="space-y-4">
                                <p class="text-sm text-primary-text leading-relaxed">Most web attacks target unpatched CMS plugins. The Web-Analyzer engine parses raw HTML, manifest files, and API endpoints to extract exact WordPress versions and active extensions.</p>
                                <div class="bg-blue-950/20 border border-blue-500/10 rounded-xl p-4">
                                    <ul class="space-y-3 text-sm text-muted">
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-blue-500 mt-1 shrink-0"/><span><strong>Versions:</strong> Detected via meta tags or `readme.html`. Unpatched cores are highly susceptible to known CVEs.</span></li>
                                    </ul>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-blue-300 mb-4">REST API User Enumeration</h3>
                        <p class="text-sm text-muted mb-6">If the `wp-json/wp/v2/users` endpoint is left exposed, it leaks internal author names, IDs, and usernames. This gives attackers 50% of the credentials needed for a brute-force attack.</p>
                    </div>
                {/if}
            </div>

            <div class="bg-[#0A0C10] p-4 border-t border-blue-500/10 flex items-center justify-between shrink-0">
                <div class="text-xs text-muted hidden sm:block">Press <kbd class="px-2 py-1 bg-glass rounded mx-1 font-mono">Esc</kbd> to dismiss</div>
                <div class="flex items-center gap-3 w-full sm:w-auto justify-end">
                    {#if activeTab > 0}
                        <button onclick={prevTab} class="flex items-center gap-1.5 px-5 py-2.5 bg-glass hover:bg-glass-hover text-primary-text text-sm rounded-xl border border-subtle transition-all"><ChevronLeft size={16}/> Previous</button>
                    {/if}
                    {#if activeTab < tabs.length - 1}
                        <button onclick={nextTab} class="flex items-center gap-1.5 px-6 py-2.5 bg-blue-600 hover:bg-blue-500 text-primary-text text-sm font-semibold rounded-xl transition-all">Continue <ChevronRight size={16}/></button>
                    {:else}
                        <button onclick={close} class="flex items-center gap-1.5 px-8 py-2.5 bg-emerald-500 hover:bg-emerald-400 text-emerald-950 font-bold rounded-xl transition-all">Got it</button>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/if}
