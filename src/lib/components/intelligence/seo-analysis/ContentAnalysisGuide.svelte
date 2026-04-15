<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, FileText, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
    function close() { isOpen = false; }
    let activeTab = $state(0);
    const tabs = [{ id: 0, label: '1. Content Signals', icon: FileText }, { id: 1, label: '2. H-Tags & Keywords', icon: ArrowRight }];
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
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30"><FileText size={28} class="text-accent"/></div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.guide_content_analysis_title()}</h2>
                            <p class="text-sm text-muted mt-1">{m.guide_content_analysis_subtitle()}</p>
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
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Content Quality Signals</h3>
                        <p class="text-sm text-muted mb-5">WebQ parses the visible text content of the page and evaluates structural quality signals that Google's quality raters assess when determining page rank.</p>
                        <div class="space-y-3">
                            {#each [
                                { signal: 'Word Count', target: '300–2000+ words', desc: 'Thin content (under 300 words) is a strong negative ranking signal. Comprehensive content that satisfies search intent outperforms thin pages.' },
                                { signal: 'Heading Structure', target: 'H1→H6 hierarchy', desc: 'A clear heading hierarchy helps crawlers understand content structure and topical coverage. Multiple H1s signal a poorly structured page.' },
                                { signal: 'Keyword Density', target: '1–3% primary', desc: 'WebQ identifies the most frequent meaningful terms. Keyword stuffing (>5%) triggers spam penalties; too sparse means missed topical relevance.' },
                                { signal: 'Heading Issues', target: '0 issues', desc: 'Skipping heading levels (H1 → H3) or using headings purely for visual style confuses crawlers about content hierarchy.' },
                            ] as s}
                                <div class="p-4 bg-sunken border border-base rounded-xl">
                                    <div class="flex items-center justify-between mb-1.5">
                                        <p class="text-sm font-bold text-primary-text">{s.signal}</p>
                                        <span class="text-xs bg-cyan-500/10 text-accent border border-cyan-500/20 px-2 py-0.5 rounded">{s.target}</span>
                                    </div>
                                    <p class="text-xs text-muted leading-relaxed">{s.desc}</p>
                                </div>
                            {/each}
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Heading Tags & Keyword Strategy</h3>
                        <div class="bg-[#0d1117] rounded-xl border border-cyan-500/20 p-4 font-mono text-xs mb-5">
                            <p class="text-muted italic mb-3">// Ideal heading structure</p>
                            <p class="text-primary-text">&lt;<span class="text-cyan-400">h1</span>&gt;Primary Keyword Target&lt;/<span class="text-cyan-400">h1</span>&gt;</p>
                            <p class="text-primary-text ml-4">&lt;<span class="text-blue-400">h2</span>&gt;Major Section Topic&lt;/<span class="text-blue-400">h2</span>&gt;</p>
                            <p class="text-primary-text ml-8">&lt;<span class="text-purple-400">h3</span>&gt;Sub-topic under section&lt;/<span class="text-purple-400">h3</span>&gt;</p>
                            <p class="text-primary-text ml-4">&lt;<span class="text-blue-400">h2</span>&gt;Another Major Section&lt;/<span class="text-blue-400">h2</span>&gt;</p>
                            <p class="text-red-400 mt-3">// Bad: Multiple H1s on same page</p>
                            <p class="text-red-300">&lt;h1&gt;Title&lt;/h1&gt; &lt;h1&gt;Another Title&lt;/h1&gt;  ← dilutes signal</p>
                        </div>
                        <div class="space-y-3">
                            <div class="flex items-start gap-2 text-sm text-muted"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span>One H1 per page — make it the target keyword phrase for the page's primary search intent.</span></div>
                            <div class="flex items-start gap-2 text-sm text-muted"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span>H2s should cover the main sub-topics — they act like a table of contents for Google's featured snippets.</span></div>
                            <div class="flex items-start gap-2 text-sm text-muted"><ArrowRight size={14} class="text-accent mt-1 shrink-0"/><span>Top keywords are extracted from visible text. If they do not align with your target keywords, the on-page copy needs revision.</span></div>
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
