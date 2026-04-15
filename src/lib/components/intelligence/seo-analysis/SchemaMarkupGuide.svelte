<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Code, ArrowRight, ChevronRight, ChevronLeft } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();
    function close() { isOpen = false; }
    let activeTab = $state(0);
    const tabs = [{ id: 0, label: '1. Schema Types', icon: Code }, { id: 1, label: '2. Rich Results', icon: ArrowRight }];
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
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-blue-600/10 flex items-center justify-center border border-cyan-500/30"><Code size={28} class="text-accent"/></div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-primary-text">{m.guide_schema_title()}</h2>
                            <p class="text-sm text-muted mt-1">{m.guide_schema_subtitle()}</p>
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
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Structured Data Types</h3>
                        <p class="text-sm text-muted mb-4">Schema.org structured data is a semantic vocabulary embedded in HTML that tells search engines exactly what a page is about — enabling rich search results beyond plain blue links.</p>
                        <div class="bg-[#0d1117] rounded-xl border border-cyan-500/20 p-4 font-mono text-xs mb-5">
                            <p class="text-muted italic mb-2">// JSON-LD — recommended format</p>
                            <p class="text-primary-text">&lt;script type="<span class="text-cyan-300">application/ld+json</span>"&gt;</p>
                            <p class="text-primary-text ml-4">{`{`}</p>
                            <p class="text-primary-text ml-6">"@context": "<span class="text-emerald-300">https://schema.org</span>",</p>
                            <p class="text-primary-text ml-6">"@type": "<span class="text-emerald-300">Article</span>",</p>
                            <p class="text-primary-text ml-6">"headline": "Page Title",</p>
                            <p class="text-primary-text ml-6">"author": {`{"@type": "Person", "name": "Author"}`}</p>
                            <p class="text-primary-text ml-4">{`}`}</p>
                            <p class="text-primary-text">&lt;/script&gt;</p>
                        </div>
                        <div class="grid grid-cols-2 gap-2">
                            {#each ['Article', 'Product', 'FAQ', 'BreadcrumbList', 'Organization', 'LocalBusiness', 'Recipe', 'Event'] as type}
                                <div class="p-2.5 bg-sunken border border-base rounded-lg">
                                    <code class="text-xs text-cyan-300">{type}</code>
                                </div>
                            {/each}
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{ duration: 200 }}>
                        <h3 class="text-lg font-semibold text-cyan-300 mb-4">Rich Results in Search</h3>
                        <p class="text-sm text-muted mb-4">Correct schema enables enhanced SERP features that dramatically increase CTR — star ratings, FAQ dropdowns, breadcrumbs, sitelinks, knowledge panels, and more.</p>
                        <div class="space-y-3">
                            {#each [
                                { type: 'Product Schema', result: 'Star ratings, price, availability in search results', impact: '+20–30% CTR increase' },
                                { type: 'FAQ Schema', result: 'Expandable Q&A directly in SERP — occupies 3x normal result space', impact: '+40% SERP space' },
                                { type: 'Article Schema', result: 'Author info, published date, featured in Google News', impact: 'News visibility' },
                                { type: 'BreadcrumbList', result: 'Path hierarchy shown in URL within result snippet', impact: 'Better UX + CTR' },
                            ] as r}
                                <div class="p-3 bg-sunken border border-base rounded-xl">
                                    <div class="flex items-center justify-between mb-1">
                                        <p class="text-sm font-bold text-primary-text">{r.type}</p>
                                        <span class="text-xs bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 px-2 py-0.5 rounded">{r.impact}</span>
                                    </div>
                                    <p class="text-xs text-muted">{r.result}</p>
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
