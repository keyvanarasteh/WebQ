<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, GitBranch, Search, Component, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() { isOpen = false; }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: '1. Topological Trees', icon: GitBranch },
        { id: 1, label: '2. Deep Discovery', icon: Component },
    ];
    function nextTab() { if (activeTab < tabs.length - 1) activeTab++; }
    function prevTab() { if (activeTab > 0) activeTab--; }
</script>

{#if isOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="fixed inset-0 z-[60] flex items-center justify-center p-4 sm:p-6 bg-overlay backdrop-blur-md"
        transition:fade={{ duration: 200 }} onclick={close}>
        <div class="bg-[#0A0C10] border border-cyan-500/20 rounded-2xl w-full max-w-3xl overflow-hidden shadow-[0_0_50px_-12px_rgba(34,211,238,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }} onclick={(e) => e.stopPropagation()}>

            <!-- Header -->
            <div class="relative bg-gradient-to-r from-cyan-950/40 via-blue-900/10 to-transparent p-6 border-b border-cyan-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-cyan-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button onclick={close} class="absolute top-4 right-4 p-2 text-muted hover:text-primary-text bg-glass hover:bg-cyan-500/20 rounded-xl transition-all border border-subtle hover:border-cyan-500/30">
                    <X size={18} />
                </button>
                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30">
                            <GitBranch size={28} class="text-cyan-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">Recursive Topologies</h2>
                            <p class="text-sm text-muted mt-1">Hierarchical visualization of clustered microservices and cloud origins.</p>
                        </div>
                    </div>
                </div>
            </div>

            <!-- Content -->
            <div class="flex-1 overflow-y-auto p-6">
                {#if activeTab === 0}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Organizational Node Mapping</h3>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                            <div class="space-y-4">
                                <p class="text-sm text-primary-text leading-relaxed">Unlike flat lists, Recursive Tree Mapping algorithmically splices endpoint URLs by dot-separators, automatically clustering related AWS staging slots, Dev APIs, and regional networks into structural trees.</p>
                                <div class="bg-cyan-950/20 border border-cyan-500/10 rounded-xl p-4">
                                    <ul class="space-y-3 text-sm text-muted">
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-cyan-500 mt-1 shrink-0"/><span><strong>Visual Clarity:</strong> Compresses 5,000 domains down into easily explorable root components.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-cyan-500 mt-1 shrink-0"/><span><strong>Pattern Discovery:</strong> Helps isolate naming conventions used by the engineering team (e.g. <code>cluster2.v1.api</code>).</span></li>
                                    </ul>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Deep Vertical Discovery</h3>
                        <p class="text-sm text-muted mb-6">Web-Analyzer automatically traverses down beyond 3 levels deep (e.g., <code>app.sys.internal.target.com</code>), uncovering extremely obscured enterprise applications mapping.</p>
                    </div>
                {/if}
            </div>

            <!-- Footer -->
            <div class="bg-[#0A0C10] p-4 border-t border-cyan-500/10 flex items-center justify-between shrink-0">
                <div class="text-xs text-muted hidden sm:block">Press <kbd class="px-2 py-1 bg-glass rounded">Esc</kbd> to dismiss</div>
                <div class="flex items-center gap-3 justify-end">
                    {#if activeTab > 0}
                        <button onclick={prevTab} class="flex items-center gap-1.5 px-5 py-2.5 bg-glass hover:bg-glass-hover text-primary-text text-sm border-subtle transition-all rounded-xl"><ChevronLeft size={16}/> Previous</button>
                    {/if}
                    {#if activeTab < tabs.length - 1}
                        <button onclick={nextTab} class="flex items-center gap-1.5 px-6 py-2.5 bg-cyan-600 hover:bg-cyan-500 text-primary-text text-sm font-semibold rounded-xl transition-all">Continue <ChevronRight size={16}/></button>
                    {:else}
                        <button onclick={close} class="flex items-center gap-1.5 px-8 py-2.5 bg-emerald-500 hover:bg-emerald-400 text-emerald-950 font-bold rounded-xl transition-all">Got it</button>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/if}
