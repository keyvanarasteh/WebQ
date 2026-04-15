<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Zap, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
    function close() { isOpen = false; }
    let activeTab = $state(0);
    const tabs = [{ id: 0, label: '1. Speed Factors', icon: Zap }, { id: 1, label: '2. Fixes', icon: ArrowRight }];
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
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30"><Zap size={28} class="text-accent"/></div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.guide_page_speed_title()}</h2>
                            <p class="text-sm text-muted mt-1">{m.guide_page_speed_subtitle()}</p>
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
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Render-Blocking Resources</h3>
                        <p class="text-sm text-muted mb-4">WebQ analyzes the HTML for render-blocking patterns — CSS and JS that prevent the browser from showing any content until fully downloaded and parsed.</p>
                        <div class="space-y-3">
                            {#each [
                                { factor: 'Render-Blocking Scripts', severity: 'High', desc: 'Synchronous <script> tags in <head> block HTML parsing completely. The browser cannot render anything until all blocking scripts download and execute.' },
                                { factor: 'Undeferred CSS', severity: 'High', desc: 'External stylesheets in <head> are render-blocking by nature. Critical CSS should be inlined; non-critical CSS loaded with media="print" and JS flip.' },
                                { factor: 'Inline Script Count', severity: 'Medium', desc: 'Large amounts of inline JavaScript increase HTML transfer size and parsing time before first paint.' },
                                { factor: 'External Script Count', severity: 'Medium', desc: 'Each external script = a separate DNS lookup + TCP connection + download. Third-party scripts (analytics, ads, chat) compound this significantly.' },
                                { factor: 'Inline Style Size', severity: 'Low', desc: 'Very large inline style blocks increase initial HTML payload. Extract and serve as external CSS with long cache TTLs.' },
                            ] as f}
                                <div class="flex items-start gap-3 p-3 bg-sunken border border-base rounded-xl">
                                    <span class="text-xs font-bold px-2 py-0.5 rounded shrink-0 {f.severity === 'High' ? 'bg-red-500/10 text-red-400 border border-red-500/30' : f.severity === 'Medium' ? 'bg-orange-500/10 text-orange-400 border border-orange-500/30' : 'bg-yellow-500/10 text-yellow-400 border border-yellow-500/30'}">{f.severity}</span>
                                    <div>
                                        <p class="text-sm font-bold text-primary-text">{f.factor}</p>
                                        <p class="text-xs text-muted mt-0.5">{f.desc}</p>
                                    </div>
                                </div>
                            {/each}
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Fix Render-Blocking Issues</h3>
                        <div class="bg-[#0d1117] rounded-xl border border-cyan-500/20 p-4 font-mono text-xs mb-5">
                            <p class="text-muted italic mb-2">// Patterns to use</p>
                            <p class="text-primary-text">&lt;script src="app.js" <span class="text-emerald-300">defer</span>&gt;&lt;/script&gt;</p>
                            <p class="text-primary-text">&lt;script src="widget.js" <span class="text-emerald-300">async</span>&gt;&lt;/script&gt;</p>
                            <p class="text-primary-text mt-2">&lt;link rel="stylesheet" href="print.css" <span class="text-emerald-300">media="print"</span>&gt;</p>
                            <p class="text-primary-text">&lt;link rel="preload" href="critical.css" as="style"&gt;</p>
                        </div>
                        <div class="space-y-2 text-sm text-muted">
                            <div class="flex gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><strong>defer:</strong> Downloads in parallel, executes after HTML parsing completes. Safe for most scripts.</span></div>
                            <div class="flex gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><strong>async:</strong> Downloads in parallel, executes as soon as downloaded. Good for independent analytics scripts.</span></div>
                            <div class="flex gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><strong>Critical CSS:</strong> Inline only the CSS needed for above-the-fold content. Load the rest after paint.</span></div>
                            <div class="flex gap-2"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span><strong>Resource hints:</strong> Use preconnect/dns-prefetch for third-party origins to reduce connection overhead.</span></div>
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
