<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Gauge, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
    function close() { isOpen = false; }
    let activeTab = $state(0);
    const tabs = [{ id: 0, label: '1. Perf Signals', icon: Gauge }, { id: 1, label: '2. Optimization', icon: ArrowRight }];
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
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30"><Gauge size={28} class="text-accent"/></div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.guide_performance_title()}</h2>
                            <p class="text-sm text-muted mt-1">{m.guide_performance_subtitle()}</p>
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
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Performance Signals Analyzed</h3>
                        <p class="text-sm text-muted mb-4">WebQ inspects HTTP response headers and HTML structure to infer performance characteristics. These are static indicators — for runtime performance use Lighthouse or WebPageTest.</p>
                        <div class="space-y-3">
                            {#each [
                                { signal: 'Response Time', ideal: '< 200ms TTFB', desc: 'Time To First Byte — how quickly the server responds. High TTFB indicates slow backend processing or missing CDN.' },
                                { signal: 'Gzip / Brotli Compression', ideal: 'Detected', desc: 'Compressing HTML/CSS/JS can reduce transfer size by 60–80%. Missing compression increases bandwidth costs and slows load.' },
                                { signal: 'Cache-Control Headers', ideal: 'max-age set', desc: 'Instructs browsers and CDNs how long to cache assets. Missing cache headers means every page visit refetches all resources.' },
                                { signal: 'JavaScript Minification', ideal: 'Minified', desc: 'Unminified JS in production wastes bytes and reveals code structure. Minification typically reduces JS size by 30–60%.' },
                                { signal: 'CSS Minification', ideal: 'Minified', desc: 'Same as JS — inline CSS should be minified. Large inline CSS blocks indicate unoptimized builds.' },
                            ] as s}
                                <div class="flex items-start gap-3 p-3 bg-sunken border border-base rounded-xl">
                                    <span class="text-xs font-mono bg-glass border border-subtle px-2 py-0.5 rounded text-muted shrink-0">{s.ideal}</span>
                                    <div>
                                        <p class="text-sm font-bold text-primary-text">{s.signal}</p>
                                        <p class="text-xs text-muted mt-0.5">{s.desc}</p>
                                    </div>
                                </div>
                            {/each}
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Performance Impact on SEO</h3>
                        <p class="text-sm text-muted mb-4">Page speed is a direct Google ranking factor since 2010, and Core Web Vitals (CWV) became part of the ranking algorithm in 2021. Slow pages rank lower — and have higher bounce rates.</p>
                        <div class="space-y-3">
                            {#each [
                                { cwv: 'LCP (Largest Contentful Paint)', target: '< 2.5s', desc: 'How quickly the largest visible element loads. Typically a hero image or headline.' },
                                { cwv: 'FID / INP (Interaction to Next Paint)', target: '< 200ms', desc: 'Responsiveness to user interactions. Long JavaScript tasks delay responses.' },
                                { cwv: 'CLS (Cumulative Layout Shift)', target: '< 0.1', desc: 'Visual stability. Images without dimensions or late-loading ads cause layout shifts — terrible UX and ranking penalty.' },
                            ] as c}
                                <div class="p-4 bg-sunken border border-base rounded-xl">
                                    <div class="flex items-center justify-between mb-1.5">
                                        <p class="text-sm font-bold text-primary-text">{c.cwv}</p>
                                        <span class="text-xs bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 px-2 py-0.5 rounded">{c.target}</span>
                                    </div>
                                    <p class="text-xs text-muted">{c.desc}</p>
                                </div>
                            {/each}
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
